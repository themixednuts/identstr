//! Lookup keys for identifier text.

use std::{
    borrow::{Borrow, Cow},
    cmp::Ordering,
    convert::Infallible,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::ManuallyDrop,
    ops::Deref,
    rc::Rc,
    str::FromStr,
    sync::Arc,
};

use crate::{
    Quote, QuoteStyle,
    policy::{self, KeyPolicy},
    repr::{INLINE_CAPACITY, Repr},
    storage::{BoxStorage, Storage},
};

/// Owned lookup key for identifier text.
///
/// [`crate::IdentStr`] can be used as a map key when you want to keep the
/// original spelling and quote style. Use `Key` for maps that only need
/// normalized lookup text.
///
/// Short keys are stored inline without allocating; longer keys spill to a
/// single heap allocation.
///
/// Owned queries should be converted with [`Key::new`] before lookup so
/// quote parsing and policy normalization are applied consistently. For
/// allocation-free lookups of unquoted query text, use [`KeyStr`].
///
/// `Key` deliberately does not implement `Borrow<str>`. That impl would
/// make maps hash `&str` queries with `str`'s own `Hash` and compare them
/// as bytes, neither of which matches the policy's hashing or equality, so
/// lookups would silently miss. [`KeyStr`] is the sound borrowed form:
/// its `Hash` and `Eq` route through the same policy as `Key`, which is
/// what the `Borrow<KeyStr<P>>` contract requires.
///
/// ```rust
/// use std::collections::HashMap;
///
/// use identstr::{Key, policy};
///
/// let mut columns = HashMap::new();
/// columns.insert(Key::<policy::Ascii>::new("\"CustomerId\""), 0);
/// columns.insert(Key::<policy::Ascii>::new("email"), 1);
///
/// assert_eq!(columns.get(&Key::new("customerid")), Some(&0));
/// assert_eq!(columns.get(&Key::new("\"EMAIL\"")), Some(&1));
/// ```
pub struct Key<P: KeyPolicy = policy::Ascii> {
    repr: Repr,
    marker: PhantomData<P>,
}

/// Borrowed lookup text for [`Key`] maps.
///
/// `KeyStr` wraps unquoted query text and applies policy normalization on
/// the fly during comparison and hashing, so map lookups do not allocate:
///
/// ```rust
/// use std::collections::HashMap;
///
/// use identstr::{Key, KeyStr, policy};
///
/// let mut tables = HashMap::new();
/// tables.insert(Key::<policy::Ascii>::new("\"Users\""), 7);
///
/// assert_eq!(tables.get(KeyStr::new("USERS")), Some(&7));
/// ```
///
/// Comparisons against plain string types apply the same policy, so `==`
/// with a `&str` is policy equality rather than byte equality:
///
/// ```rust
/// use identstr::{KeyStr, policy};
///
/// let name = KeyStr::<policy::Ascii>::new("Binary");
///
/// assert_eq!(name, "BINARY");
/// assert_ne!(KeyStr::<policy::Exact>::new("Binary"), "BINARY");
/// ```
///
/// Unlike [`Key::new`], `KeyStr` does not parse quote delimiters. Convert
/// quoted query text with [`Key::new`] instead.
#[repr(transparent)]
pub struct KeyStr<P: KeyPolicy = policy::Ascii> {
    marker: PhantomData<P>,
    value: str,
}

impl<P: KeyPolicy> Key<P> {
    /// Builds a key from identifier source text.
    ///
    /// When the input is surrounded by a recognized [`Quote`] pair, the
    /// surrounding quotes are removed and doubled closing delimiters are
    /// unescaped before the key text is stored. Otherwise, including
    /// malformed quoted text, the input is stored as raw identifier text.
    ///
    /// Use [`from_source`](Self::from_source) to parse a custom
    /// [`QuoteStyle`] instead of [`Quote`].
    ///
    /// ```rust
    /// use identstr::{Key, policy};
    ///
    /// let key = Key::<policy::Ascii>::new("\"Users\"");
    /// assert_eq!(key.as_str(), "users");
    /// ```
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self::from_source::<Quote>(value)
    }

    /// Builds a key from identifier source text with quote style `Q`.
    ///
    /// This behaves like [`new`](Self::new) with `Q` deciding which quote
    /// pairs are recognized.
    ///
    /// ```rust
    /// use identstr::{Key, Quote, policy};
    ///
    /// let key = Key::<policy::Ascii>::from_source::<Quote>("[Users]");
    ///
    /// assert_eq!(key.as_str(), "users");
    /// ```
    #[must_use]
    pub fn from_source<Q: QuoteStyle>(value: &str) -> Self {
        match crate::parse::quoted_source::<Q>(value) {
            Some((_, Cow::Borrowed(inner))) => Self::from_unquoted(inner),
            Some((_, Cow::Owned(inner))) => Self::from_unquoted_string(inner),
            None => Self::from_unquoted(value),
        }
    }

    /// Builds a key from already-unquoted identifier text.
    ///
    /// Use this when the input is already identifier text and should not be
    /// parsed for surrounding quote delimiters.
    ///
    /// ```rust
    /// use identstr::{Key, policy};
    ///
    /// let key = Key::<policy::Ascii>::from_unquoted("\"Users\"");
    /// assert_eq!(key.as_str(), "\"users\"");
    /// ```
    #[must_use]
    pub fn from_unquoted(value: &str) -> Self {
        let mut buf = [0_u8; INLINE_CAPACITY];
        if let Some(len) = P::write_key_inline(value, &mut buf)
            && let Ok(text) = std::str::from_utf8(&buf[..len])
        {
            return Self::from_repr(Repr::inline_from(text, None));
        }

        Self::from_box(P::key(value))
    }

    /// Returns the lookup text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.repr.as_str()
    }

    /// Returns the lookup bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.repr.as_bytes()
    }

    /// Returns `true` when the text is held without an allocation.
    #[must_use]
    pub const fn is_inline(&self) -> bool {
        self.repr.is_inline()
    }

    pub(crate) fn from_unquoted_string(value: String) -> Self {
        Self::from_box(P::into_key(value.into_boxed_str()))
    }

    fn from_source_box(value: Box<str>) -> Self {
        match crate::parse::quoted_source::<Quote>(&value) {
            Some((_, Cow::Borrowed(inner))) => Self::from_unquoted(inner),
            Some((_, Cow::Owned(inner))) => Self::from_box(P::into_key(inner.into_boxed_str())),
            None => Self::from_box(P::into_key(value)),
        }
    }

    // Keeps the existing allocation rather than paying a copy to inline it;
    // only `from_unquoted` builds inline keys, where doing so avoids the
    // allocation entirely.
    fn from_box(value: Box<str>) -> Self {
        Self::from_repr(BoxStorage::into_repr(value, 0))
    }

    fn from_repr(repr: Repr) -> Self {
        Self {
            repr,
            marker: PhantomData,
        }
    }

    fn into_box(self) -> Box<str> {
        let this = ManuallyDrop::new(self);
        if this.repr.is_inline() {
            Box::<str>::from(this.as_str())
        } else {
            BoxStorage::into_owned(this.repr)
        }
    }
}

impl<P: KeyPolicy> Drop for Key<P> {
    fn drop(&mut self) {
        if self.repr.is_inline() {
            return;
        }

        BoxStorage::drop_repr(self.repr);
    }
}

impl<P: KeyPolicy> Clone for Key<P> {
    fn clone(&self) -> Self {
        let repr = if self.repr.is_inline() {
            self.repr
        } else {
            BoxStorage::clone_repr(&self.repr)
        };

        Self::from_repr(repr)
    }
}

impl<P: KeyPolicy> KeyStr<P> {
    /// Wraps unquoted query text for allocation-free map lookups.
    #[must_use]
    pub fn new(value: &str) -> &Self {
        // SAFETY: `KeyStr` is `repr(transparent)` over `str`, so the cast
        // preserves layout, metadata, and lifetime.
        unsafe { &*(std::ptr::from_ref::<str>(value) as *const Self) }
    }

    /// Returns the query text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl<P: KeyPolicy> fmt::Debug for KeyStr<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<P: KeyPolicy> fmt::Display for KeyStr<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<P: KeyPolicy> AsRef<str> for KeyStr<P> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<P: KeyPolicy> AsRef<[u8]> for KeyStr<P> {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl<P: KeyPolicy> Deref for KeyStr<P> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<P: KeyPolicy> Default for &KeyStr<P> {
    fn default() -> Self {
        KeyStr::new("")
    }
}

impl<'a, P: KeyPolicy> From<&'a str> for &'a KeyStr<P> {
    fn from(value: &'a str) -> Self {
        KeyStr::new(value)
    }
}

impl<P: KeyPolicy> PartialEq for KeyStr<P> {
    fn eq(&self, other: &Self) -> bool {
        P::eq(self.as_str(), other.as_str())
    }
}

impl<P: KeyPolicy> Eq for KeyStr<P> {}

impl<P: KeyPolicy> PartialOrd for KeyStr<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: KeyPolicy> Ord for KeyStr<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        P::cmp(self.as_str(), other.as_str())
    }
}

impl<P: KeyPolicy> Hash for KeyStr<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        P::hash(self.as_str(), state);
    }
}

// `KeyStr` hashes and compares through the same policy as `Key`, so this
// upholds the `Borrow` contract that owned and borrowed forms agree. A
// `Borrow<str>` impl would not: `str` hashes and compares as raw bytes.
impl<P: KeyPolicy> Borrow<KeyStr<P>> for Key<P> {
    fn borrow(&self) -> &KeyStr<P> {
        KeyStr::new(self.as_str())
    }
}

impl<P: KeyPolicy> ToOwned for KeyStr<P> {
    type Owned = Key<P>;

    fn to_owned(&self) -> Key<P> {
        Key::from_unquoted(self.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for Key<P> {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self.as_str(), other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for KeyStr<P> {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self.as_str(), other.as_str())
    }
}

impl<P: KeyPolicy, Q: QuoteStyle, S: Storage> PartialEq<crate::IdentStr<Q, P, S>> for KeyStr<P> {
    fn eq(&self, other: &crate::IdentStr<Q, P, S>) -> bool {
        P::eq(self.as_str(), other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<str> for KeyStr<P> {
    fn eq(&self, other: &str) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<&str> for KeyStr<P> {
    fn eq(&self, other: &&str) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<String> for KeyStr<P> {
    fn eq(&self, other: &String) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<&String> for KeyStr<P> {
    fn eq(&self, other: &&String) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<'a, P: KeyPolicy> PartialEq<Cow<'a, str>> for KeyStr<P> {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<Box<str>> for KeyStr<P> {
    fn eq(&self, other: &Box<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<Arc<str>> for KeyStr<P> {
    fn eq(&self, other: &Arc<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<Rc<str>> for KeyStr<P> {
    fn eq(&self, other: &Rc<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for str {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for &str {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for String {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for &String {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for Cow<'_, str> {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for Box<str> {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for Arc<str> {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<KeyStr<P>> for Rc<str> {
    fn eq(&self, other: &KeyStr<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialOrd<str> for KeyStr<P> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(P::cmp(self.as_str(), other))
    }
}

impl<P: KeyPolicy> PartialOrd<&str> for KeyStr<P> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        Some(P::cmp(self.as_str(), other))
    }
}

impl<P: KeyPolicy> PartialOrd<String> for KeyStr<P> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(P::cmp(self.as_str(), other))
    }
}

impl<P: KeyPolicy> PartialOrd<KeyStr<P>> for str {
    fn partial_cmp(&self, other: &KeyStr<P>) -> Option<Ordering> {
        Some(P::cmp(self, other.as_str()))
    }
}

impl<P: KeyPolicy> PartialOrd<KeyStr<P>> for &str {
    fn partial_cmp(&self, other: &KeyStr<P>) -> Option<Ordering> {
        Some(P::cmp(self, other.as_str()))
    }
}

impl<P: KeyPolicy> PartialOrd<KeyStr<P>> for String {
    fn partial_cmp(&self, other: &KeyStr<P>) -> Option<Ordering> {
        Some(P::cmp(self, other.as_str()))
    }
}

impl<P: KeyPolicy> fmt::Debug for Key<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<P: KeyPolicy> fmt::Display for Key<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<P: KeyPolicy> Deref for Key<P> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<P: KeyPolicy> AsRef<str> for Key<P> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<P: KeyPolicy> AsRef<[u8]> for Key<P> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<P: KeyPolicy> Default for Key<P> {
    fn default() -> Self {
        Self::new("")
    }
}

impl<P: KeyPolicy> PartialEq for Key<P> {
    fn eq(&self, other: &Self) -> bool {
        // Inline reprs are canonical, so two inline keys can be compared as
        // raw bytes without decoding their lengths.
        if let (Some(lhs), Some(rhs)) = (self.repr.inline_array(), other.repr.inline_array()) {
            return lhs == rhs;
        }

        self.as_str() == other.as_str()
    }
}

impl<P: KeyPolicy, Q: QuoteStyle, S: Storage> PartialEq<crate::IdentStr<Q, P, S>> for Key<P> {
    fn eq(&self, other: &crate::IdentStr<Q, P, S>) -> bool {
        P::eq(other.as_str(), self.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<str> for Key<P> {
    fn eq(&self, other: &str) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<&str> for Key<P> {
    fn eq(&self, other: &&str) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<String> for Key<P> {
    fn eq(&self, other: &String) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<&String> for Key<P> {
    fn eq(&self, other: &&String) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<'a, P: KeyPolicy> PartialEq<Cow<'a, str>> for Key<P> {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<Box<str>> for Key<P> {
    fn eq(&self, other: &Box<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<Arc<str>> for Key<P> {
    fn eq(&self, other: &Arc<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> PartialEq<Rc<str>> for Key<P> {
    fn eq(&self, other: &Rc<str>) -> bool {
        P::eq(self.as_str(), other)
    }
}

impl<P: KeyPolicy> Eq for Key<P> {}

impl<P: KeyPolicy> PartialOrd for Key<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: KeyPolicy> Ord for Key<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<P: KeyPolicy> PartialOrd<str> for Key<P> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(P::cmp(self.as_str(), other))
    }
}

impl<P: KeyPolicy> PartialOrd<&str> for Key<P> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        Some(P::cmp(self.as_str(), other))
    }
}

impl<P: KeyPolicy> PartialOrd<String> for Key<P> {
    fn partial_cmp(&self, other: &String) -> Option<Ordering> {
        Some(P::cmp(self.as_str(), other))
    }
}

impl<P: KeyPolicy> Hash for Key<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        P::hash_key(self.as_str(), state);
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for str {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for &str {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for String {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for &String {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Cow<'_, str> {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Box<str> {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Arc<str> {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Rc<str> {
    fn eq(&self, other: &Key<P>) -> bool {
        P::eq(self, other.as_str())
    }
}

impl<P: KeyPolicy> PartialOrd<Key<P>> for str {
    fn partial_cmp(&self, other: &Key<P>) -> Option<Ordering> {
        Some(P::cmp(self, other.as_str()))
    }
}

impl<P: KeyPolicy> PartialOrd<Key<P>> for &str {
    fn partial_cmp(&self, other: &Key<P>) -> Option<Ordering> {
        Some(P::cmp(self, other.as_str()))
    }
}

impl<P: KeyPolicy> PartialOrd<Key<P>> for String {
    fn partial_cmp(&self, other: &Key<P>) -> Option<Ordering> {
        Some(P::cmp(self, other.as_str()))
    }
}

impl<P: KeyPolicy> From<&str> for Key<P> {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl<P: KeyPolicy> From<String> for Key<P> {
    fn from(value: String) -> Self {
        Self::from_source_box(value.into_boxed_str())
    }
}

impl<'a, P: KeyPolicy> From<Cow<'a, str>> for Key<P> {
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(value) => Self::new(value),
            Cow::Owned(value) => Self::from(value),
        }
    }
}

impl<P: KeyPolicy> From<Box<str>> for Key<P> {
    fn from(value: Box<str>) -> Self {
        Self::from_source_box(value)
    }
}

impl<P: KeyPolicy, Q: QuoteStyle, S: Storage> From<&crate::IdentStr<Q, P, S>> for Key<P> {
    fn from(value: &crate::IdentStr<Q, P, S>) -> Self {
        Self::from_unquoted(value.as_str())
    }
}

impl<P: KeyPolicy> From<Key<P>> for Box<str> {
    fn from(value: Key<P>) -> Self {
        value.into_box()
    }
}

impl<P: KeyPolicy> From<Key<P>> for String {
    fn from(value: Key<P>) -> Self {
        let value: Box<str> = value.into();
        value.into_string()
    }
}

impl<P: KeyPolicy> FromStr for Key<P> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
