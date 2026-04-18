//! Cached keys for identifier policies.

use std::{
    borrow::Borrow,
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

use crate::policy::{self, KeyPolicy};

/// Owned key text for a [`crate::policy::KeyPolicy`].
///
/// Use this when the same identifier participates in repeated comparisons,
/// hashing, or map lookups under one policy.
pub struct Key<P: KeyPolicy = policy::Ascii> {
    value: Box<str>,
    marker: PhantomData<P>,
}

impl<P: KeyPolicy> Key<P> {
    /// Builds a key for the provided text.
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self::from_box(P::key(value))
    }

    /// Returns the stored key text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the stored key bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
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

impl<P: KeyPolicy> Borrow<str> for Key<P> {
    fn borrow(&self) -> &str {
        self.as_str()
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

impl<P: KeyPolicy> PartialEq<str> for Key<P> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<P: KeyPolicy> PartialEq<&str> for Key<P> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<P: KeyPolicy> PartialEq<String> for Key<P> {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other
    }
}

impl<P: KeyPolicy> PartialEq<&String> for Key<P> {
    fn eq(&self, other: &&String) -> bool {
        self.as_str() == **other
    }
}

impl<'a, P: KeyPolicy> PartialEq<Cow<'a, str>> for Key<P> {
    fn eq(&self, other: &Cow<'a, str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<P: KeyPolicy> PartialEq<Box<str>> for Key<P> {
    fn eq(&self, other: &Box<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<P: KeyPolicy> PartialEq<Arc<str>> for Key<P> {
    fn eq(&self, other: &Arc<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<P: KeyPolicy> PartialEq<Rc<str>> for Key<P> {
    fn eq(&self, other: &Rc<str>) -> bool {
        self.as_str() == other.as_ref()
    }
}

impl<P: KeyPolicy, Q: crate::QuoteTag, S: crate::Spill> PartialEq<crate::IdentStr<Q, P, S>>
    for Key<P>
{
    fn eq(&self, other: &crate::IdentStr<Q, P, S>) -> bool {
        P::eq(other.as_str(), self.as_str())
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for str {
    fn eq(&self, other: &Key<P>) -> bool {
        self == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for &str {
    fn eq(&self, other: &Key<P>) -> bool {
        *self == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for String {
    fn eq(&self, other: &Key<P>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for &String {
    fn eq(&self, other: &Key<P>) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Cow<'_, str> {
    fn eq(&self, other: &Key<P>) -> bool {
        self.as_ref() == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Box<str> {
    fn eq(&self, other: &Key<P>) -> bool {
        self.as_ref() == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Arc<str> {
    fn eq(&self, other: &Key<P>) -> bool {
        self.as_ref() == other.as_str()
    }
}

impl<P: KeyPolicy> PartialEq<Key<P>> for Rc<str> {
    fn eq(&self, other: &Key<P>) -> bool {
        self.as_ref() == other.as_str()
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
        Self::from_box(P::into_key(value.into_boxed_str()))
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
        Self::from_box(P::into_key(value))
    }
}

impl<P: KeyPolicy, Q: crate::QuoteTag, S: crate::Spill> From<&crate::IdentStr<Q, P, S>> for Key<P> {
    fn from(value: &crate::IdentStr<Q, P, S>) -> Self {
        Self::new(value.as_str())
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
