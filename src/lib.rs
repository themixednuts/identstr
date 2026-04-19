//! Immutable identifier strings with preserved quote style.
//!
//! [`IdentStr`] stores the unquoted identifier text and, when present, the
//! delimiter used by the source text. Equality, ordering, and hashing follow a
//! configurable policy, while display and string access return the stored text.
//!
//! [`Quote`] and [`policy::Ascii`] cover the common SQL-style case. Unicode
//! policies and security helpers are available with the `unicode` cargo
//! feature.

pub mod key;
pub mod policy;
#[cfg(feature = "unicode")]
pub mod unicode;
#[cfg(feature = "unicode")]
pub use unicode::security;

mod quote;
mod repr;
mod spill;
mod private {
    pub trait Sealed {}
}

use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::Infallible,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

pub use key::Key;
pub use quote::Quote;
pub use spill::{ArcSpill, BoxSpill, RcSpill, Spill};

use policy::{KeyPolicy, Policy};
use repr::{INLINE_CAPACITY, QUOTED_INLINE_CAPACITY, Repr};

const INLINE_LEN_U8: [u8; INLINE_CAPACITY + 1] = [
    0,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    #[cfg(target_pointer_width = "64")]
    9,
    #[cfg(target_pointer_width = "64")]
    10,
    #[cfg(target_pointer_width = "64")]
    11,
    #[cfg(target_pointer_width = "64")]
    12,
    #[cfg(target_pointer_width = "64")]
    13,
    #[cfg(target_pointer_width = "64")]
    14,
    #[cfg(target_pointer_width = "64")]
    15,
    #[cfg(target_pointer_width = "64")]
    16,
];

const fn quote_open_str(quote: Quote) -> &'static str {
    match quote {
        Quote::Double => "\"",
        Quote::Single => "'",
        Quote::Backtick => "`",
        Quote::Bracket => "[",
    }
}

const fn quote_close_str(quote: Quote) -> &'static str {
    match quote {
        Quote::Bracket => "]",
        Quote::Double => "\"",
        Quote::Single => "'",
        Quote::Backtick => "`",
    }
}

const fn quote_escape_byte(quote: Quote) -> u8 {
    match quote {
        Quote::Double => b'"',
        Quote::Single => b'\'',
        Quote::Backtick => b'`',
        Quote::Bracket => b']',
    }
}

fn quoted_capacity(value: &str, quote: Option<Quote>) -> usize {
    let Some(quote) = quote else {
        return value.len();
    };

    let escape = quote_escape_byte(quote);
    value.len() + 2 + value.bytes().filter(|byte| *byte == escape).count()
}

fn write_quoted_to(
    value: &str,
    quote: Option<Quote>,
    output: &mut (impl fmt::Write + ?Sized),
) -> fmt::Result {
    let Some(quote) = quote else {
        return output.write_str(value);
    };

    output.write_str(quote_open_str(quote))?;

    let escape = quote_escape_byte(quote);
    let mut start = 0;

    for (index, byte) in value.bytes().enumerate() {
        if byte != escape {
            continue;
        }

        output.write_str(&value[start..index])?;
        output.write_str(quote_close_str(quote))?;
        output.write_str(quote_close_str(quote))?;
        start = index + 1;
    }

    output.write_str(&value[start..])?;
    output.write_str(quote_close_str(quote))
}

fn push_quoted_to(value: &str, quote: Option<Quote>, output: &mut String) {
    let Some(quote) = quote else {
        output.push_str(value);
        return;
    };

    output.push_str(quote_open_str(quote));

    let escape = quote_escape_byte(quote);
    let mut start = 0;

    for (index, byte) in value.bytes().enumerate() {
        if byte != escape {
            continue;
        }

        output.push_str(&value[start..index]);
        output.push_str(quote_close_str(quote));
        output.push_str(quote_close_str(quote));
        start = index + 1;
    }

    output.push_str(&value[start..]);
    output.push_str(quote_close_str(quote));
}

/// Quote metadata stored alongside an identifier string.
///
/// The default [`Quote`] type covers common SQL delimiters. Custom quote types
/// can be used when a format needs different delimiters.
pub trait QuoteTag: Copy + Eq + 'static {
    /// Encodes this quote marker as a stored tag.
    fn encode(self) -> u8;

    /// Decodes a previously stored quote tag.
    fn decode(tag: u8) -> Option<Self>;
}

/// Immutable identifier text with optional quote metadata.
///
/// Equality, ordering, and hashing follow the configured identifier policy `P`.
/// Quote metadata is available separately via [`IdentStr::quote`] and does not
/// participate in those traits.
pub struct IdentStr<Q: QuoteTag = Quote, P: Policy = policy::Ascii, S: Spill = BoxSpill> {
    repr: Repr,
    marker: PhantomData<(Q, P, S)>,
}

/// Display adapter that renders an identifier with preserved SQL-style quotes.
pub struct Quoted<'a, P: Policy = policy::Ascii, S: Spill = BoxSpill> {
    ident: &'a IdentStr<Quote, P, S>,
}

#[doc(hidden)]
pub trait Input<S: Spill>: private::Sealed {
    #[doc(hidden)]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S>;
}

impl<Q: QuoteTag, P: Policy, S: Spill> IdentStr<Q, P, S> {
    /// Maximum number of bytes stored inline for an unquoted identifier.
    pub const INLINE_CAPACITY: usize = INLINE_CAPACITY;

    /// Maximum number of bytes stored inline for a quoted identifier.
    pub const QUOTED_INLINE_CAPACITY: usize = QUOTED_INLINE_CAPACITY;

    /// Creates an identifier without quote metadata.
    ///
    /// This accepts borrowed text, owned strings, and the owned string type
    /// for the selected storage backend.
    #[must_use]
    #[inline]
    pub fn new(value: impl Input<S>) -> Self {
        value.into_identstr::<Q, P>(None)
    }

    /// Creates an identifier with the given quote metadata.
    #[must_use]
    #[inline]
    pub fn with_quote(value: impl Input<S>, quote: Q) -> Self {
        value.into_identstr::<Q, P>(Some(quote))
    }

    /// Returns the stored identifier text without surrounding quotes.
    #[must_use]
    #[inline]
    pub fn as_str(&self) -> &str {
        self.repr.as_str()
    }

    /// Returns the stored identifier bytes without surrounding quotes.
    #[must_use]
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.repr.as_bytes()
    }

    /// Returns the preserved quote delimiter, if the identifier was quoted.
    #[must_use]
    #[inline]
    pub fn quote(&self) -> Option<Q> {
        Q::decode(self.repr.quote_tag())
    }

    /// Returns `true` when the text is stored inline.
    #[must_use]
    pub const fn is_inline(&self) -> bool {
        self.repr.is_inline()
    }

    /// Creates an empty identifier.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            repr: Repr::new_inline(Self::inline_bytes("", None)),
            marker: PhantomData,
        }
    }

    /// Creates an inline identifier without quote metadata.
    ///
    /// This is available in const contexts and returns `None` when `value`
    /// exceeds the inline capacity.
    #[must_use]
    pub const fn new_inline(value: &str) -> Option<Self> {
        if value.len() > INLINE_CAPACITY {
            return None;
        }

        Some(Self {
            repr: Repr::new_inline(Self::inline_bytes(value, None)),
            marker: PhantomData,
        })
    }

    /// Converts this identifier into the owned string type for `S`.
    #[must_use]
    pub fn into_owned(self) -> S::Owned {
        let this = std::mem::ManuallyDrop::new(self);
        if this.repr.is_inline() {
            S::from_borrowed(this.as_str())
        } else {
            S::into_owned(this.repr)
        }
    }

    /// Returns a cached key for repeated comparisons under `P`.
    ///
    /// This is useful when the same identifier participates in repeated map,
    /// set, or lookup operations under one policy.
    #[must_use]
    pub fn to_key(&self) -> Key<P>
    where
        P: KeyPolicy,
    {
        Key::new(self.as_str())
    }

    /// Returns a confusable skeleton for this identifier.
    #[cfg(feature = "unicode")]
    #[must_use]
    pub fn confusable_skeleton(&self) -> security::Skeleton {
        security::skeleton(self.as_str())
    }

    /// Returns Unicode security information for this identifier.
    #[cfg(feature = "unicode")]
    #[must_use]
    pub fn analyze_security(&self) -> security::Analysis {
        security::analyze(self.as_str())
    }

    #[inline]
    fn from_ref(value: &str, quote: Option<Q>) -> Self {
        if value.len() <= Self::inline_cap(quote) {
            Self::from_inline(value, quote)
        } else {
            Self::new_heap(S::from_borrowed(value), quote)
        }
    }

    #[inline]
    fn from_owned(value: S::Owned, quote: Option<Q>) -> Self {
        if value.as_ref().len() <= Self::inline_cap(quote) {
            Self::from_inline(value.as_ref(), quote)
        } else {
            Self::new_heap(value, quote)
        }
    }

    #[inline]
    fn from_string(value: String, quote: Option<Q>) -> Self {
        if value.len() <= Self::inline_cap(quote) {
            Self::from_inline(&value, quote)
        } else {
            Self::new_heap(S::from_box(value.into_boxed_str()), quote)
        }
    }

    #[inline]
    fn from_inline(value: &str, quote: Option<Q>) -> Self {
        Self {
            repr: Repr::new_inline(Self::inline_bytes(value, quote.and_then(Self::inline_tag))),
            marker: PhantomData,
        }
    }

    #[inline]
    fn new_heap(value: S::Owned, quote: Option<Q>) -> Self {
        Self {
            repr: S::into_repr(value, Self::heap_tag(quote)),
            marker: PhantomData,
        }
    }

    #[inline]
    fn inline_cap(quote: Option<Q>) -> usize {
        match quote.and_then(Self::inline_tag) {
            Some(_) => QUOTED_INLINE_CAPACITY,
            None if quote.is_none() => INLINE_CAPACITY,
            None => 0,
        }
    }

    #[inline]
    fn inline_tag(quote: Q) -> Option<u8> {
        let tag = quote.encode();
        (tag <= Repr::MAX_INLINE_QUOTE_TAG).then_some(tag)
    }

    #[inline]
    fn heap_tag(quote: Option<Q>) -> u8 {
        let tag = quote.map_or(0, QuoteTag::encode);
        assert!(
            tag <= Repr::MAX_HEAP_QUOTE_TAG,
            "quote tag does not fit heap marker space",
        );
        tag
    }

    const fn inline_bytes(value: &str, quote_tag: Option<u8>) -> [u8; INLINE_CAPACITY] {
        let src = value.as_bytes();
        let mut bytes = [0; INLINE_CAPACITY];
        let mut index = 0;
        while index < src.len() {
            bytes[index] = src[index];
            index += 1;
        }

        match quote_tag {
            Some(tag) => {
                if src.len() < QUOTED_INLINE_CAPACITY {
                    bytes[QUOTED_INLINE_CAPACITY - 1] =
                        Repr::short_inline_byte(INLINE_LEN_U8[src.len()]);
                }
                bytes[INLINE_CAPACITY - 1] = Repr::quoted_inline_byte(tag);
            }
            None if src.len() < INLINE_CAPACITY => {
                bytes[INLINE_CAPACITY - 1] = Repr::short_inline_byte(INLINE_LEN_U8[src.len()]);
            }
            None => {}
        }

        bytes
    }
}

impl<P: Policy, S: Spill> IdentStr<Quote, P, S> {
    /// Creates an inline identifier with preserved quote metadata.
    ///
    /// This is available in const contexts and returns `None` when `value`
    /// exceeds the quoted inline capacity.
    #[must_use]
    pub const fn with_quote_inline(value: &str, quote: Quote) -> Option<Self> {
        if value.len() > QUOTED_INLINE_CAPACITY {
            return None;
        }

        Some(Self {
            repr: Repr::new_inline(Self::inline_bytes(value, Some(quote.tag()))),
            marker: PhantomData,
        })
    }

    /// Returns a display adapter that renders with preserved quote style.
    #[must_use]
    pub fn display_quoted(&self) -> Quoted<'_, P, S> {
        Quoted { ident: self }
    }

    /// Writes this identifier with preserved quote style.
    ///
    /// # Errors
    ///
    /// Returns any error reported by the destination writer.
    pub fn write_quoted(&self, output: &mut (impl fmt::Write + ?Sized)) -> fmt::Result {
        write_quoted_to(self.as_str(), self.quote(), output)
    }

    /// Renders this identifier with preserved quote style.
    #[must_use]
    pub fn to_quoted_string(&self) -> String {
        let mut rendered = String::with_capacity(quoted_capacity(self.as_str(), self.quote()));
        push_quoted_to(self.as_str(), self.quote(), &mut rendered);
        rendered
    }
}

impl<P: Policy, S: Spill> fmt::Display for Quoted<'_, P, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ident.write_quoted(f)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Default for IdentStr<Q, P, S> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Clone for IdentStr<Q, P, S> {
    #[inline]
    fn clone(&self) -> Self {
        let repr = if self.repr.is_inline() {
            self.repr
        } else {
            S::clone_repr(&self.repr)
        };

        Self {
            repr,
            marker: PhantomData,
        }
    }
}

impl<Q: QuoteTag + fmt::Debug, P: Policy, S: Spill> fmt::Debug for IdentStr<Q, P, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("IdentStr");
        debug.field("value", &self.as_str());
        debug.field("quote", &self.quote());
        debug.finish()
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> fmt::Display for IdentStr<Q, P, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Drop for IdentStr<Q, P, S> {
    fn drop(&mut self) {
        if self.repr.is_inline() {
            return;
        }

        S::drop_repr(self.repr);
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Deref for IdentStr<Q, P, S> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> AsRef<str> for IdentStr<Q, P, S> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> AsRef<[u8]> for IdentStr<Q, P, S> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<Q: QuoteTag, P: Policy, L: Spill, R: Spill> PartialEq<IdentStr<Q, P, R>>
    for IdentStr<Q, P, L>
{
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, R>) -> bool {
        P::eq(self.as_str(), other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Eq for IdentStr<Q, P, S> {}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialOrd for IdentStr<Q, P, S> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Ord for IdentStr<Q, P, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        P::cmp(self.as_str(), other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> Hash for IdentStr<Q, P, S> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        P::hash_bytes(self.as_bytes(), state);
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<str> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<&str> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<String> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<&String> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &&String) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<'a, Q: QuoteTag, P: Policy, S: Spill> PartialEq<Cow<'a, str>> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<Box<str>> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &Box<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<Arc<str>> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &Arc<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<Rc<str>> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &Rc<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<Q: QuoteTag, P: KeyPolicy, S: Spill> PartialEq<Key<P>> for IdentStr<Q, P, S> {
    #[inline]
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self.as_str(), other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for str {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for &str {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for String {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for &String {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for Cow<'_, str> {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for Box<str> {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for Arc<str> {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> PartialEq<IdentStr<Q, P, S>> for Rc<str> {
    #[inline]
    fn eq(&self, other: &IdentStr<Q, P, S>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> From<&str> for IdentStr<Q, P, S> {
    #[inline]
    fn from(value: &str) -> Self {
        Self::from_ref(value, None)
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> From<String> for IdentStr<Q, P, S> {
    #[inline]
    fn from(value: String) -> Self {
        Self::from_string(value, None)
    }
}

impl<'a, Q: QuoteTag, P: Policy, S: Spill> From<Cow<'a, str>> for IdentStr<Q, P, S> {
    #[inline]
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(value) => Self::from_ref(value, None),
            Cow::Owned(value) => Self::from_string(value, None),
        }
    }
}

impl<Q: QuoteTag, P: Policy> From<Box<str>> for IdentStr<Q, P, BoxSpill> {
    fn from(value: Box<str>) -> Self {
        Self::from_owned(value, None)
    }
}

impl<Q: QuoteTag, P: Policy> From<Arc<str>> for IdentStr<Q, P, ArcSpill> {
    fn from(value: Arc<str>) -> Self {
        Self::from_owned(value, None)
    }
}

impl<Q: QuoteTag, P: Policy> From<Rc<str>> for IdentStr<Q, P, RcSpill> {
    fn from(value: Rc<str>) -> Self {
        Self::from_owned(value, None)
    }
}

impl<Q: QuoteTag, P: Policy> From<IdentStr<Q, P, BoxSpill>> for Box<str> {
    fn from(value: IdentStr<Q, P, BoxSpill>) -> Self {
        value.into_owned()
    }
}

impl<Q: QuoteTag, P: Policy> From<IdentStr<Q, P, BoxSpill>> for String {
    fn from(value: IdentStr<Q, P, BoxSpill>) -> Self {
        let value: Box<str> = value.into();
        value.into_string()
    }
}

impl<Q: QuoteTag, P: Policy> From<IdentStr<Q, P, ArcSpill>> for Arc<str> {
    fn from(value: IdentStr<Q, P, ArcSpill>) -> Self {
        value.into_owned()
    }
}

impl<Q: QuoteTag, P: Policy> From<IdentStr<Q, P, RcSpill>> for Rc<str> {
    fn from(value: IdentStr<Q, P, RcSpill>) -> Self {
        value.into_owned()
    }
}

impl<Q: QuoteTag, P: Policy, S: Spill> FromStr for IdentStr<Q, P, S> {
    type Err = Infallible;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_ref(s, None))
    }
}

unsafe impl<Q: QuoteTag, P: Policy, S: Spill> Send for IdentStr<Q, P, S> where S::Owned: Send {}
unsafe impl<Q: QuoteTag, P: Policy, S: Spill> Sync for IdentStr<Q, P, S> where S::Owned: Sync {}

impl<T: ?Sized + AsRef<str>> private::Sealed for &T {}

impl<S: Spill, T: ?Sized + AsRef<str>> Input<S> for &T {
    #[inline]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S> {
        IdentStr::from_ref(self.as_ref(), quote)
    }
}

impl private::Sealed for String {}

impl<S: Spill> Input<S> for String {
    #[inline]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S> {
        IdentStr::from_string(self, quote)
    }
}

impl private::Sealed for Cow<'_, str> {}

impl<S: Spill> Input<S> for Cow<'_, str> {
    #[inline]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S> {
        match self {
            Cow::Borrowed(value) => IdentStr::from_ref(value, quote),
            Cow::Owned(value) => IdentStr::from_string(value, quote),
        }
    }
}

impl private::Sealed for Box<str> {}

impl Input<BoxSpill> for Box<str> {
    #[inline]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, BoxSpill> {
        IdentStr::from_owned(self, quote)
    }
}

impl private::Sealed for Arc<str> {}

impl Input<ArcSpill> for Arc<str> {
    #[inline]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, ArcSpill> {
        IdentStr::from_owned(self, quote)
    }
}

impl private::Sealed for Rc<str> {}

impl Input<RcSpill> for Rc<str> {
    #[inline]
    fn into_identstr<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, RcSpill> {
        IdentStr::from_owned(self, quote)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "unicode")]
    use super::security;
    use super::{ArcSpill, BoxSpill, IdentStr, Key, Quote, QuoteTag, RcSpill, policy};
    use std::{
        cmp::Ordering,
        hash::{Hash, Hasher},
        mem::size_of,
        rc::Rc,
        str::FromStr,
        sync::Arc,
    };

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum TestQuote {
        Double = 1,
        Single = 2,
        Backtick = 3,
        Bracket = 4,
    }

    impl QuoteTag for TestQuote {
        fn encode(self) -> u8 {
            self as u8
        }

        fn decode(tag: u8) -> Option<Self> {
            match tag {
                1 => Some(Self::Double),
                2 => Some(Self::Single),
                3 => Some(Self::Backtick),
                4 => Some(Self::Bracket),
                _ => None,
            }
        }
    }

    #[repr(u8)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum HeapOnlyQuote {
        Custom = 0x10,
    }

    impl QuoteTag for HeapOnlyQuote {
        fn encode(self) -> u8 {
            self as u8
        }

        fn decode(tag: u8) -> Option<Self> {
            match tag {
                0x10 => Some(Self::Custom),
                _ => None,
            }
        }
    }

    type TestIdentStr = IdentStr<TestQuote>;

    #[test]
    fn storage_uses_two_word_budget() {
        assert_eq!(size_of::<TestIdentStr>(), 2 * size_of::<usize>());
    }

    #[test]
    fn default_quote_tag_matches_sql_delimiters() {
        assert_eq!(Quote::from_open('"'), Some(Quote::Double));
        assert_eq!(Quote::from_tag(Quote::Double.tag()), Some(Quote::Double));
        assert_eq!(Quote::try_from('"'), Ok(Quote::Double));
        assert_eq!(Quote::try_from(Quote::Double.tag()), Ok(Quote::Double));
        assert_eq!(Quote::from_open('\''), Some(Quote::Single));
        assert_eq!(Quote::from_open('`'), Some(Quote::Backtick));
        assert_eq!(Quote::from_open('['), Some(Quote::Bracket));
        assert_eq!(Quote::Double.open(), '"');
        assert_eq!(Quote::Bracket.close(), ']');
    }

    #[test]
    fn uses_full_inline_capacity() {
        let inline = "a".repeat(TestIdentStr::INLINE_CAPACITY);
        let spill = "a".repeat(TestIdentStr::INLINE_CAPACITY + 1);

        let inline_name = TestIdentStr::new(inline.as_str());
        let spill_name = TestIdentStr::new(spill.as_str());

        assert!(inline_name.is_inline());
        assert!(!spill_name.is_inline());
    }

    #[test]
    fn quoted_uses_expected_inline_capacity() {
        let inline = "a".repeat(TestIdentStr::QUOTED_INLINE_CAPACITY);
        let spill = "a".repeat(TestIdentStr::QUOTED_INLINE_CAPACITY + 1);

        let inline_name = TestIdentStr::with_quote(inline.as_str(), TestQuote::Double);
        let spill_name = TestIdentStr::with_quote(spill.as_str(), TestQuote::Double);

        assert!(inline_name.is_inline());
        assert!(!spill_name.is_inline());
    }

    #[test]
    fn quoted_full_inline_roundtrips_utf8_tail() {
        let value = "a".repeat(TestIdentStr::QUOTED_INLINE_CAPACITY - "é".len()) + "é";
        let name = TestIdentStr::with_quote(value.as_str(), TestQuote::Double);

        assert!(name.is_inline());
        assert_eq!(name.as_str(), value);
        assert_eq!(name.as_bytes(), value.as_bytes());
    }

    #[test]
    fn quoted_owned_box_uses_owned_path() {
        let name = IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
            Box::<str>::from("Users"),
            Quote::Double,
        );

        assert!(name.is_inline());
        assert_eq!(name.as_str(), "Users");
        assert_eq!(name.quote(), Some(Quote::Double));
    }

    #[test]
    fn const_inline_constructors_work() {
        const EMPTY: IdentStr<Quote> = IdentStr::empty();
        const INLINE: Option<IdentStr<Quote>> = IdentStr::new_inline("users");
        const QUOTED: Option<IdentStr<Quote>> = IdentStr::with_quote_inline("Users", Quote::Double);
        let inline_value = INLINE;
        let quoted_value = QUOTED;
        let inline = inline_value.as_ref().expect("fits inline");
        let quoted = quoted_value.as_ref().expect("fits inline");

        assert!(EMPTY.is_empty());
        assert_eq!(inline.as_str(), "users");
        assert_eq!(quoted.as_str(), "Users");
        assert_eq!(quoted.quote(), Some(Quote::Double));
    }

    #[test]
    fn long_heap_path_roundtrips_owned_conversions() {
        let raw = "this_identifier_name_is_long_enough_to_spill_out_of_line";
        let name = TestIdentStr::from(raw.to_owned());

        assert_eq!(name.as_str(), raw);
        assert_eq!(name.quote(), None);

        let cloned = name.clone();
        let owned: String = cloned.into();
        assert_eq!(owned, raw);

        let boxed: Box<str> = name.into();
        assert_eq!(&*boxed, raw);
    }

    #[test]
    fn arc_spill_keeps_shared_ownership() {
        let raw = "this_identifier_name_is_long_enough_to_spill_out_of_line";
        let name = IdentStr::<TestQuote, policy::Ascii, ArcSpill>::from(raw);
        let cloned = name.clone();

        let left: Arc<str> = name.into();
        let right: Arc<str> = cloned.into();

        assert!(Arc::ptr_eq(&left, &right));
        assert_eq!(&*left, raw);
    }

    #[test]
    fn rc_spill_keeps_shared_ownership() {
        let raw = "this_identifier_name_is_long_enough_to_spill_out_of_line";
        let name = IdentStr::<TestQuote, policy::Ascii, RcSpill>::from(raw);
        let cloned = name.clone();

        let left: Rc<str> = name.into();
        let right: Rc<str> = cloned.into();

        assert!(Rc::ptr_eq(&left, &right));
        assert_eq!(&*left, raw);
    }

    #[test]
    fn box_spill_roundtrips_exact_owned_value() {
        let raw = "this_identifier_name_is_long_enough_to_spill_out_of_line";
        let name = IdentStr::<TestQuote, policy::Ascii, BoxSpill>::from(Box::<str>::from(raw));
        let boxed: Box<str> = name.into();
        assert_eq!(&*boxed, raw);
    }

    #[test]
    fn short_owned_string_stays_inline() {
        let name = TestIdentStr::from(String::from("short_name"));
        assert!(name.is_inline());
    }

    #[test]
    fn byte_view_matches_stored_text() {
        let name = TestIdentStr::with_quote("Users", TestQuote::Double);
        assert_eq!(name.as_bytes(), b"Users");
        assert_eq!(AsRef::<[u8]>::as_ref(&name), b"Users");
    }

    #[test]
    fn display_quoted_renders_preserved_quote_style() {
        let unquoted = IdentStr::<Quote>::new("Users");
        let double = IdentStr::<Quote>::with_quote("User\"Table", Quote::Double);
        let single = IdentStr::<Quote>::with_quote("User'Table", Quote::Single);
        let backtick = IdentStr::<Quote>::with_quote("User`Table", Quote::Backtick);
        let bracket = IdentStr::<Quote>::with_quote("User]Table", Quote::Bracket);

        assert_eq!(unquoted.to_quoted_string(), "Users");
        assert_eq!(double.display_quoted().to_string(), "\"User\"\"Table\"");
        assert_eq!(single.to_quoted_string(), "'User''Table'");
        assert_eq!(backtick.to_quoted_string(), "`User``Table`");
        assert_eq!(bracket.to_quoted_string(), "[User]]Table]");
        assert_eq!(double.to_string(), "User\"Table");
    }

    #[test]
    fn debug_includes_quote_metadata() {
        let quoted = IdentStr::<Quote>::with_quote("Users", Quote::Double);
        let unquoted = IdentStr::<Quote>::new("users");

        assert_eq!(
            format!("{quoted:?}"),
            "IdentStr { value: \"Users\", quote: Some(Double) }"
        );
        assert_eq!(
            format!("{unquoted:?}"),
            "IdentStr { value: \"users\", quote: None }"
        );
    }

    #[test]
    fn short_name_with_heap_only_quote_spills() {
        let name = IdentStr::<HeapOnlyQuote>::with_quote("short", HeapOnlyQuote::Custom);
        assert!(!name.is_inline());
        assert_eq!(name.quote(), Some(HeapOnlyQuote::Custom));
        assert_eq!(name.as_str(), "short");
    }

    #[test]
    fn string_traits_follow_ascii_identifier_semantics() {
        let unquoted = TestIdentStr::new("Users");
        let quoted = TestIdentStr::with_quote("users", TestQuote::Double);

        assert_eq!(unquoted, quoted);
        assert_eq!(unquoted, "uSeRs");
        assert_eq!("USERs", quoted);
        assert_eq!(unquoted.cmp(&quoted), Ordering::Equal);
    }

    #[test]
    fn owned_string_traits_follow_ascii_identifier_semantics() {
        let ident = TestIdentStr::with_quote("Users", TestQuote::Double);
        let owned = String::from("users");
        let boxed = Box::<str>::from("users");
        let shared = Arc::<str>::from("users");
        let local = Rc::<str>::from("users");
        let cow = std::borrow::Cow::Borrowed("users");

        assert_eq!(ident, owned);
        assert_eq!(ident, boxed);
        assert_eq!(ident, shared);
        assert_eq!(ident, local);
        assert_eq!(ident, cow);
        assert_eq!(owned, ident);
        assert_eq!(boxed, ident);
        assert_eq!(shared, ident);
        assert_eq!(local, ident);
        assert_eq!(cow, ident);
    }

    #[test]
    fn ascii_policy_does_not_case_fold_non_ascii_bytes() {
        let upper = TestIdentStr::new("Ä");
        let lower = TestIdentStr::new("ä");

        assert_ne!(upper, lower);
    }

    #[test]
    fn hashing_uses_identifier_semantics() {
        let lhs = TestIdentStr::new("Users");
        let rhs = TestIdentStr::with_quote("users", TestQuote::Bracket);

        let mut lhs_hasher = std::collections::hash_map::DefaultHasher::new();
        let mut rhs_hasher = std::collections::hash_map::DefaultHasher::new();
        lhs.hash(&mut lhs_hasher);
        rhs.hash(&mut rhs_hasher);

        assert_eq!(lhs_hasher.finish(), rhs_hasher.finish());
    }

    #[test]
    fn equality_is_independent_of_spill_backend() {
        let boxed = IdentStr::<TestQuote>::from("Users");
        let shared = IdentStr::<TestQuote, policy::Ascii, ArcSpill>::from("users");

        assert_eq!(boxed, shared);
    }

    #[test]
    fn key_caches_policy_text() {
        let key = Key::<policy::Ascii>::new("Users");
        assert_eq!(key.as_str(), "users");
    }

    #[test]
    fn key_compares_with_ident_and_string_types() {
        let ident = TestIdentStr::with_quote("Users", TestQuote::Double);
        let key = Key::<policy::Ascii>::new("Users");
        let owned = String::from("users");
        let cow = std::borrow::Cow::Borrowed("users");
        let boxed = Box::<str>::from("users");
        let shared = Arc::<str>::from("users");
        let local = Rc::<str>::from("users");

        assert_eq!(key, "users");
        assert_eq!("users", key);
        assert_eq!(key, owned);
        assert_eq!(owned, key);
        assert_eq!(key, cow);
        assert_eq!(cow, key);
        assert_eq!(key, boxed);
        assert_eq!(boxed, key);
        assert_eq!(key, shared);
        assert_eq!(shared, key);
        assert_eq!(key, local);
        assert_eq!(local, key);
        assert_eq!(key, ident);
        assert_eq!(ident, key);
    }

    #[test]
    fn from_str_parses_ident_and_key() {
        let ident = IdentStr::<Quote>::from_str("Users").expect("infallible parse");
        let key = Key::<policy::Ascii>::from_str("Users").expect("infallible parse");

        assert_eq!(ident.as_str(), "Users");
        assert_eq!(key.as_str(), "users");
        assert_eq!(key.as_bytes(), b"users");
    }

    #[test]
    fn key_reuses_owned_ascii_buffer_for_cached_text() {
        let key = Key::<policy::Ascii>::from(Box::<str>::from("Users"));
        assert_eq!(key.as_str(), "users");
    }

    #[test]
    fn lowercase_ascii_key_reuses_owned_box() {
        let value = Box::<str>::from("users");
        let ptr = value.as_ptr();
        let key = Key::<policy::Ascii>::from(value);
        let value: Box<str> = key.into();
        assert_eq!(ptr, value.as_ptr());
    }

    #[test]
    fn boxed_spill_reuses_original_allocation() {
        let value = Box::<str>::from("this_identifier_name_is_long_enough_to_spill_out_of_line");
        let ptr = value.as_ptr();
        let ident = IdentStr::<TestQuote, policy::Ascii, BoxSpill>::from(value);
        let value: Box<str> = ident.into();
        assert_eq!(ptr, value.as_ptr());
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn unicode_nfc_matches_canonical_casefolded_text() {
        let lhs = IdentStr::<TestQuote, policy::UnicodeNfc>::from("É");
        let rhs = IdentStr::<TestQuote, policy::UnicodeNfc>::from("e\u{301}");

        assert_eq!(lhs, rhs);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn unicode_nfc_hashing_matches_identifier_semantics() {
        let lhs = IdentStr::<TestQuote, policy::UnicodeNfc>::from("É");
        let rhs = IdentStr::<TestQuote, policy::UnicodeNfc>::from("e\u{301}");

        let mut lhs_hasher = std::collections::hash_map::DefaultHasher::new();
        let mut rhs_hasher = std::collections::hash_map::DefaultHasher::new();
        lhs.hash(&mut lhs_hasher);
        rhs.hash(&mut rhs_hasher);

        assert_eq!(lhs_hasher.finish(), rhs_hasher.finish());
        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn unicode_nfkc_matches_compatibility_casefolded_text() {
        let lhs = IdentStr::<TestQuote, policy::UnicodeNfkc>::from("ﬀ");
        let rhs = IdentStr::<TestQuote, policy::UnicodeNfkc>::from("FF");

        assert_eq!(lhs, rhs);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn unicode_nfkc_hashing_matches_identifier_semantics() {
        let lhs = IdentStr::<TestQuote, policy::UnicodeNfkc>::from("ﬀ");
        let rhs = IdentStr::<TestQuote, policy::UnicodeNfkc>::from("FF");

        let mut lhs_hasher = std::collections::hash_map::DefaultHasher::new();
        let mut rhs_hasher = std::collections::hash_map::DefaultHasher::new();
        lhs.hash(&mut lhs_hasher);
        rhs.hash(&mut rhs_hasher);

        assert_eq!(lhs_hasher.finish(), rhs_hasher.finish());
        assert_eq!(lhs.cmp(&rhs), Ordering::Equal);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn unicode_turkic_policies_follow_dotted_i_rules() {
        let lhs = IdentStr::<TestQuote, policy::UnicodeTurkicNfc>::from("İ");
        let rhs = IdentStr::<TestQuote, policy::UnicodeTurkicNfc>::from("i");
        let upper = IdentStr::<TestQuote, policy::UnicodeTurkicNfc>::from("I");
        let dotless = IdentStr::<TestQuote, policy::UnicodeTurkicNfc>::from("ı");

        assert_eq!(lhs, rhs);
        assert_eq!(upper, dotless);
        assert_ne!(lhs, upper);
    }

    #[cfg(feature = "unicode")]
    #[test]
    fn security_helpers_expose_skeleton_and_analysis() {
        let lhs = IdentStr::<TestQuote>::from("ｓ");
        let rhs = IdentStr::<TestQuote>::from("s");
        let safe = IdentStr::<TestQuote>::from("users");

        assert!(security::is_confusable(lhs.as_str(), rhs.as_str()));

        let analysis = safe.analyze_security();
        assert!(analysis.identifier_allowed);
        assert_eq!(lhs.confusable_skeleton().as_str(), "s");
    }
}
