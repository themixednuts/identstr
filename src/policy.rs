//! Identifier comparison policies.
//!
//! Policies define how [`crate::IdentStr`] compares, orders, and hashes text.
//! The stored text is unchanged; only trait behavior varies.
//!
//! [`Ascii`] is always available. Unicode-aware policies are available with the
//! `unicode` cargo feature.

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[cfg(feature = "unicode")]
pub use crate::unicode::policy::{UnicodeNfc, UnicodeNfkc, UnicodeTurkicNfc, UnicodeTurkicNfkc};

/// Comparison, ordering, and hashing behavior for identifier text.
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
    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        Self::hash(value, state);
    }
}

/// ASCII case-insensitive comparison.
///
/// ASCII letters fold by case, and non-ASCII bytes are compared exactly.
#[derive(Clone, Copy, Debug, Default)]
pub struct Ascii;

const ASCII_HASH_CHUNK_LEN: usize = 64;

#[inline]
const fn fold_ascii(byte: u8) -> u8 {
    byte.to_ascii_lowercase()
}

#[inline]
fn has_upper(value: &str) -> bool {
    has_upper_bytes(value.as_bytes())
}

#[inline]
fn has_upper_bytes(bytes: &[u8]) -> bool {
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index].is_ascii_uppercase() {
            return true;
        }
        index += 1;
    }

    false
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
    cmp_ascii_bytes(lhs.as_bytes(), rhs.as_bytes())
}

#[inline]
fn cmp_ascii_bytes(lhs: &[u8], rhs: &[u8]) -> Ordering {
    let shared_len = lhs.len().min(rhs.len());

    let mut index = 0;
    while index < shared_len {
        let lhs = fold_ascii(lhs[index]);
        let rhs = fold_ascii(rhs[index]);
        if lhs != rhs {
            return lhs.cmp(&rhs);
        }
        index += 1;
    }

    lhs.len().cmp(&rhs.len())
}

#[inline]
fn eq_ascii(lhs: &str, rhs: &str) -> bool {
    lhs.eq_ignore_ascii_case(rhs)
}

#[inline]
fn hash_ascii<H: Hasher>(value: &str, state: &mut H) {
    hash_ascii_bytes(value.as_bytes(), state);
}

#[inline]
fn hash_ascii_bytes<H: Hasher>(bytes: &[u8], state: &mut H) {
    bytes.len().hash(state);

    if !has_upper_bytes(bytes) {
        state.write(bytes);
        return;
    }

    let mut scratch = [0_u8; ASCII_HASH_CHUNK_LEN];
    let mut chunks = bytes.chunks_exact(ASCII_HASH_CHUNK_LEN);
    for chunk in &mut chunks {
        let mut index = 0;
        while index < ASCII_HASH_CHUNK_LEN {
            scratch[index] = fold_ascii(chunk[index]);
            index += 1;
        }
        state.write(&scratch);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        let mut index = 0;
        while index < remainder.len() {
            scratch[index] = fold_ascii(remainder[index]);
            index += 1;
        }
        state.write(&scratch[..remainder.len()]);
    }
}

impl Policy for Ascii {
    #[inline]
    fn eq(lhs: &str, rhs: &str) -> bool {
        eq_ascii(lhs, rhs)
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
        let bytes = value.as_bytes();
        bytes.len().hash(state);
        state.write(bytes);
    }
}
