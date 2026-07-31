//! Identifier comparison policies.
//!
//! Policies define how [`crate::IdentStr`] compares, orders, and hashes text.
//! The stored text is unchanged; only trait behavior varies.
//!
//! [`Ascii`] and [`Exact`] are always available. Unicode-aware policies are
//! available with the `unicode` cargo feature, and the Turkic variants with
//! the `unicode-turkic` feature.
//!
//! ```rust
//! use identstr::{IdentStr, Quote, policy};
//!
//! type Insensitive = IdentStr<Quote, policy::Ascii>;
//! type Sensitive = IdentStr<Quote, policy::Exact>;
//!
//! assert_eq!(Insensitive::new("Users"), Insensitive::new("USERS"));
//! assert_ne!(Sensitive::new("Users"), Sensitive::new("USERS"));
//! ```

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[cfg(feature = "unicode")]
pub use crate::unicode::policy::{UnicodeNfc, UnicodeNfkc};
#[cfg(feature = "unicode-turkic")]
pub use crate::unicode::policy::{UnicodeTurkicNfc, UnicodeTurkicNfkc};

/// Comparison, ordering, and hashing behavior for identifier text.
///
/// # Hashing
///
/// [`hash`](Policy::hash) must issue an identical sequence of [`Hasher`]
/// calls for any two policy-equal inputs. Hashers are not required to be
/// insensitive to how bytes are split across [`Hasher::write`] calls, so
/// equal values that produce differently chunked writes can hash differently
/// under hashers such as `FxHasher`.
pub trait Policy: Copy + 'static {
    /// Returns whether two identifiers are equal under this policy.
    #[must_use]
    fn eq(lhs: &str, rhs: &str) -> bool {
        Self::cmp(lhs, rhs).is_eq()
    }

    /// Compares two identifiers under this policy.
    #[must_use]
    fn cmp(lhs: &str, rhs: &str) -> Ordering;

    /// Hashes an identifier under this policy.
    fn hash<H: Hasher>(value: &str, state: &mut H);
}

/// Owned lookup-key generation for identifier policies.
pub trait KeyPolicy: Policy {
    /// Converts owned text into the lookup form used by [`crate::Key`].
    fn into_key(value: Box<str>) -> Box<str>;

    /// Builds lookup text from borrowed input.
    #[must_use]
    fn key(value: &str) -> Box<str> {
        Self::into_key(Box::<str>::from(value))
    }

    /// Hashes lookup text that was already produced by this policy.
    ///
    /// This must agree with [`Policy::hash`] for text in lookup form.
    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        Self::hash(value, state);
    }

    /// Writes the lookup form of `value` into `buf` without allocating.
    ///
    /// Returns the written length when the lookup form fits in `buf`. The
    /// written prefix must be valid UTF-8 and match [`key`](Self::key).
    /// Returning `None` falls back to the allocating path.
    #[doc(hidden)]
    fn write_key_inline(value: &str, buf: &mut [u8]) -> Option<usize> {
        let _ = (value, buf);
        None
    }
}

/// ASCII case-insensitive comparison.
///
/// ASCII letters fold by case, and non-ASCII bytes are compared exactly.
///
/// ```rust
/// use identstr::{Key, policy};
///
/// let key = Key::<policy::Ascii>::new("CustomerId");
///
/// assert_eq!(key.as_str(), "customerid");
/// assert_eq!(key, "CUSTOMERID");
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Ascii;

/// Byte-exact, case-sensitive comparison.
///
/// Use this when identifier lookup should distinguish case, such as quoted
/// identifiers in `PostgreSQL` semantics.
///
/// ```rust
/// use identstr::{Key, policy};
///
/// let key = Key::<policy::Exact>::new("CustomerId");
///
/// assert_eq!(key.as_str(), "CustomerId");
/// assert_ne!(key, "customerid");
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Exact;

const ASCII_HASH_CHUNK_LEN: usize = 64;

#[inline]
fn fold_ascii(byte: u8) -> u8 {
    byte.to_ascii_lowercase()
}

#[inline]
fn has_upper(value: &str) -> bool {
    value.as_bytes().iter().any(u8::is_ascii_uppercase)
}

#[inline]
fn ascii_key(value: Box<str>) -> Box<str> {
    if !has_upper(&value) {
        return value;
    }

    let mut value = value.into_string();
    value.make_ascii_lowercase();
    value.into_boxed_str()
}

#[inline]
fn cmp_ascii(lhs: &str, rhs: &str) -> Ordering {
    lhs.bytes().map(fold_ascii).cmp(rhs.bytes().map(fold_ascii))
}

// Every branch below feeds the hasher a length prefix followed by
// `ASCII_HASH_CHUNK_LEN`-sized writes, so policy-equal text hashes the same
// under chunk-sensitive hashers regardless of case content.
#[inline]
fn hash_ascii<H: Hasher>(value: &str, state: &mut H) {
    let bytes = value.as_bytes();
    bytes.len().hash(state);

    if bytes.len() <= ASCII_HASH_CHUNK_LEN {
        hash_ascii_chunk(bytes, state);
        return;
    }

    for chunk in bytes.chunks(ASCII_HASH_CHUNK_LEN) {
        hash_ascii_chunk(chunk, state);
    }
}

#[inline]
fn hash_ascii_chunk<H: Hasher>(chunk: &[u8], state: &mut H) {
    if chunk.iter().any(u8::is_ascii_uppercase) {
        let mut scratch = [0_u8; ASCII_HASH_CHUNK_LEN];
        let folded = &mut scratch[..chunk.len()];
        folded.copy_from_slice(chunk);
        folded.make_ascii_lowercase();
        state.write(folded);
    } else {
        state.write(chunk);
    }
}

// Lookup text is already folded, so skip the case scan but keep the same
// write pattern as `hash_ascii`.
#[inline]
fn hash_ascii_key<H: Hasher>(value: &str, state: &mut H) {
    let bytes = value.as_bytes();
    bytes.len().hash(state);

    if bytes.len() <= ASCII_HASH_CHUNK_LEN {
        state.write(bytes);
        return;
    }

    for chunk in bytes.chunks(ASCII_HASH_CHUNK_LEN) {
        state.write(chunk);
    }
}

impl Policy for Ascii {
    #[inline]
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs.eq_ignore_ascii_case(rhs)
    }

    #[inline]
    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        cmp_ascii(lhs, rhs)
    }

    #[inline]
    fn hash<H: Hasher>(value: &str, state: &mut H) {
        hash_ascii(value, state);
    }
}

impl KeyPolicy for Ascii {
    #[inline]
    fn into_key(value: Box<str>) -> Box<str> {
        ascii_key(value)
    }

    #[inline]
    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        hash_ascii_key(value, state);
    }

    #[inline]
    fn write_key_inline(value: &str, buf: &mut [u8]) -> Option<usize> {
        let target = buf.get_mut(..value.len())?;
        target.copy_from_slice(value.as_bytes());
        target.make_ascii_lowercase();
        Some(value.len())
    }
}

impl Policy for Exact {
    #[inline]
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs == rhs
    }

    #[inline]
    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        lhs.cmp(rhs)
    }

    #[inline]
    fn hash<H: Hasher>(value: &str, state: &mut H) {
        let bytes = value.as_bytes();
        bytes.len().hash(state);
        state.write(bytes);
    }
}

impl KeyPolicy for Exact {
    #[inline]
    fn into_key(value: Box<str>) -> Box<str> {
        value
    }

    #[inline]
    fn key(value: &str) -> Box<str> {
        Box::<str>::from(value)
    }

    #[inline]
    fn write_key_inline(value: &str, buf: &mut [u8]) -> Option<usize> {
        let target = buf.get_mut(..value.len())?;
        target.copy_from_slice(value.as_bytes());
        Some(value.len())
    }
}
