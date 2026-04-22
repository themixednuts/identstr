//! Lookup keys for identifier text.

use std::{
    borrow::Cow,
    cmp::Ordering,
    convert::Infallible,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
};

use crate::{
    Quote,
    policy::{self, KeyPolicy},
};

/// Owned lookup key for identifier text.
///
/// Most code can use [`crate::IdentStr`] directly.
///
/// Use `Key` when you store lookup keys separately from the original
/// identifier text.
pub struct Key<P: KeyPolicy = policy::Ascii> {
    value: Box<str>,
    marker: PhantomData<P>,
}

impl<P: KeyPolicy> Key<P> {
    /// Builds a key from identifier source text.
    ///
    /// When the input is surrounded by a recognized quote pair, the
    /// surrounding quotes are removed and doubled closing delimiters are
    /// unescaped before the key text is stored. Otherwise, including malformed
    /// quoted text, the input is stored as raw identifier text.
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self::from_source_ref(value)
    }

    /// Builds a key from already-unquoted identifier text.
    ///
    /// Use this when the input is already identifier text and should not be
    /// parsed for surrounding quote delimiters.
    #[must_use]
    pub fn from_raw(value: &str) -> Self {
        Self::from_box(P::key(value))
    }

    /// Returns the lookup text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the lookup bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    fn from_source_ref(value: &str) -> Self {
        match crate::parse::quoted_source::<Quote>(value) {
            Some((_, Cow::Borrowed(value))) => Self::from_box(P::key(value)),
            Some((_, Cow::Owned(value))) => Self::from_box(P::into_key(value.into_boxed_str())),
            None => Self::from_raw(value),
        }
    }

    fn from_source_box(value: Box<str>) -> Self {
        match crate::parse::quoted_source::<Quote>(&value) {
            Some((_, Cow::Borrowed(value))) => Self::from_box(P::key(value)),
            Some((_, Cow::Owned(value))) => Self::from_box(P::into_key(value.into_boxed_str())),
            None => Self::from_box(P::into_key(value)),
        }
    }

    fn from_box(value: Box<str>) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<P: KeyPolicy> Clone for Key<P> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: PhantomData,
        }
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
        self.value == other.value
    }
}

impl<P: KeyPolicy, Q: crate::QuoteTag, S: crate::Spill> PartialEq<crate::IdentStr<Q, P, S>>
    for Key<P>
{
    fn eq(&self, other: &crate::IdentStr<Q, P, S>) -> bool {
        P::eq(other.as_str(), self.as_str())
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
        self.value.cmp(&other.value)
    }
}

impl<P: KeyPolicy> Hash for Key<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
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

impl<'a, P: KeyPolicy> From<std::borrow::Cow<'a, str>> for Key<P> {
    fn from(value: std::borrow::Cow<'a, str>) -> Self {
        match value {
            std::borrow::Cow::Borrowed(value) => Self::new(value),
            std::borrow::Cow::Owned(value) => Self::from(value),
        }
    }
}

impl<P: KeyPolicy> From<Box<str>> for Key<P> {
    fn from(value: Box<str>) -> Self {
        Self::from_source_box(value)
    }
}

impl<P: KeyPolicy, Q: crate::QuoteTag, S: crate::Spill> From<&crate::IdentStr<Q, P, S>> for Key<P> {
    fn from(value: &crate::IdentStr<Q, P, S>) -> Self {
        Self::from_raw(value.as_str())
    }
}

impl<P: KeyPolicy> From<Key<P>> for Box<str> {
    fn from(value: Key<P>) -> Self {
        value.value
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
