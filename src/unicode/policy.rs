//! Unicode identifier comparison policies.

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[cfg(feature = "unicode-turkic")]
use std::borrow::Cow;

use caseless::Caseless;
#[cfg(feature = "unicode-turkic")]
use icu_casemap::{CaseMapper, CaseMapperBorrowed};
use unicode_normalization::UnicodeNormalization;
#[cfg(feature = "unicode-turkic")]
use unicode_normalization::{IsNormalized, is_nfc_quick};

use crate::policy::{KeyPolicy, Policy};

/// Unicode canonical caseless comparison.
///
/// Comparison, ordering, and hashing stream through normalization iterators
/// without allocating.
///
/// ```rust
/// use identstr::{IdentStr, Quote, policy};
///
/// let composed = IdentStr::<Quote, policy::UnicodeNfc>::new("Café");
/// let decomposed = IdentStr::<Quote, policy::UnicodeNfc>::new("cafe\u{301}");
///
/// assert_eq!(composed, decomposed);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeNfc;

/// Unicode compatibility caseless comparison.
///
/// Comparison, ordering, and hashing stream through normalization iterators
/// without allocating.
///
/// ```rust
/// use identstr::{IdentStr, Quote, policy};
///
/// let ligature = IdentStr::<Quote, policy::UnicodeNfkc>::new("ﬀ");
/// let expanded = IdentStr::<Quote, policy::UnicodeNfkc>::new("FF");
///
/// assert_eq!(ligature, expanded);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeNfkc;

/// Turkic-aware Unicode canonical caseless comparison.
///
/// Turkic case folding builds normalized strings on every comparison,
/// ordering, and hashing call. Prefer [`crate::Key`] for map-heavy
/// workloads so normalization runs once per value.
///
/// ```rust
/// use identstr::{IdentStr, Quote, policy};
///
/// let dotted = IdentStr::<Quote, policy::UnicodeTurkicNfc>::new("İstanbul");
/// let lower = IdentStr::<Quote, policy::UnicodeTurkicNfc>::new("istanbul");
///
/// assert_eq!(dotted, lower);
/// ```
#[cfg(feature = "unicode-turkic")]
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeTurkicNfc;

/// Turkic-aware Unicode compatibility caseless comparison.
///
/// Turkic case folding builds normalized strings on every comparison,
/// ordering, and hashing call. Prefer [`crate::Key`] for map-heavy
/// workloads so normalization runs once per value.
///
/// ```rust
/// use identstr::{IdentStr, Quote, policy};
///
/// let upper = IdentStr::<Quote, policy::UnicodeTurkicNfkc>::new("IRMAK");
/// let lower = IdentStr::<Quote, policy::UnicodeTurkicNfkc>::new("ırmak");
///
/// assert_eq!(upper, lower);
/// ```
#[cfg(feature = "unicode-turkic")]
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeTurkicNfkc;

#[cfg(feature = "unicode-turkic")]
const TURKIC_CASE_MAPPER: CaseMapperBorrowed<'static> = CaseMapper::new();

fn cmp_chars<I, J>(lhs: I, rhs: J) -> Ordering
where
    I: Iterator<Item = char>,
    J: Iterator<Item = char>,
{
    lhs.cmp(rhs)
}

fn hash_chars<H, I>(chars: I, state: &mut H)
where
    H: Hasher,
    I: Iterator<Item = char>,
{
    let mut count = 0usize;
    for ch in chars {
        ch.hash(state);
        count += 1;
    }
    count.hash(state);
}

fn canonical_chars(value: &str) -> impl Iterator<Item = char> + '_ {
    value.chars().nfd().default_case_fold().nfd()
}

fn compatibility_chars(value: &str) -> impl Iterator<Item = char> + '_ {
    value
        .chars()
        .nfd()
        .default_case_fold()
        .nfkd()
        .default_case_fold()
        .nfkd()
}

fn canonical_key(value: &str) -> Box<str> {
    canonical_chars(value).collect::<String>().into_boxed_str()
}

fn compatibility_key(value: &str) -> Box<str> {
    compatibility_chars(value)
        .collect::<String>()
        .into_boxed_str()
}

#[cfg(feature = "unicode-turkic")]
fn nfc_cow(value: &str) -> Cow<'_, str> {
    match is_nfc_quick(value.chars()) {
        IsNormalized::Yes => Cow::Borrowed(value),
        IsNormalized::No | IsNormalized::Maybe => Cow::Owned(value.chars().nfc().collect()),
    }
}

#[cfg(feature = "unicode-turkic")]
fn turkic_canonical(value: &str) -> String {
    let normalized = nfc_cow(value);
    let folded = TURKIC_CASE_MAPPER.fold_turkic_string(&normalized);
    folded.chars().nfd().collect()
}

#[cfg(feature = "unicode-turkic")]
fn turkic_compatibility(value: &str) -> String {
    let normalized = nfc_cow(value);
    let folded = TURKIC_CASE_MAPPER.fold_turkic_string(&normalized);
    let compatible = folded.chars().nfkd().collect::<String>();
    let renormalized = nfc_cow(&compatible);
    let refolded = TURKIC_CASE_MAPPER.fold_turkic_string(&renormalized);
    refolded.chars().nfkd().collect()
}

impl Policy for UnicodeNfc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs == rhs || lhs.chars().canonical_caseless_match(rhs.chars())
    }

    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        cmp_chars(canonical_chars(lhs), canonical_chars(rhs))
    }

    fn hash<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(canonical_chars(value), state);
    }
}

impl KeyPolicy for UnicodeNfc {
    fn into_key(value: Box<str>) -> Box<str> {
        canonical_key(&value)
    }

    fn key(value: &str) -> Box<str> {
        canonical_key(value)
    }

    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(value.chars(), state);
    }
}

impl Policy for UnicodeNfkc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs == rhs || lhs.chars().compatibility_caseless_match(rhs.chars())
    }

    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        cmp_chars(compatibility_chars(lhs), compatibility_chars(rhs))
    }

    fn hash<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(compatibility_chars(value), state);
    }
}

impl KeyPolicy for UnicodeNfkc {
    fn into_key(value: Box<str>) -> Box<str> {
        compatibility_key(&value)
    }

    fn key(value: &str) -> Box<str> {
        compatibility_key(value)
    }

    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(value.chars(), state);
    }
}

#[cfg(feature = "unicode-turkic")]
impl Policy for UnicodeTurkicNfc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs == rhs || turkic_canonical(lhs) == turkic_canonical(rhs)
    }

    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        turkic_canonical(lhs).cmp(&turkic_canonical(rhs))
    }

    fn hash<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(turkic_canonical(value).chars(), state);
    }
}

#[cfg(feature = "unicode-turkic")]
impl KeyPolicy for UnicodeTurkicNfc {
    fn into_key(value: Box<str>) -> Box<str> {
        turkic_canonical(&value).into_boxed_str()
    }

    fn key(value: &str) -> Box<str> {
        turkic_canonical(value).into_boxed_str()
    }

    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(value.chars(), state);
    }
}

#[cfg(feature = "unicode-turkic")]
impl Policy for UnicodeTurkicNfkc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs == rhs || turkic_compatibility(lhs) == turkic_compatibility(rhs)
    }

    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        turkic_compatibility(lhs).cmp(&turkic_compatibility(rhs))
    }

    fn hash<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(turkic_compatibility(value).chars(), state);
    }
}

#[cfg(feature = "unicode-turkic")]
impl KeyPolicy for UnicodeTurkicNfkc {
    fn into_key(value: Box<str>) -> Box<str> {
        turkic_compatibility(&value).into_boxed_str()
    }

    fn key(value: &str) -> Box<str> {
        turkic_compatibility(value).into_boxed_str()
    }

    fn hash_key<H: Hasher>(value: &str, state: &mut H) {
        hash_chars(value.chars(), state);
    }
}
