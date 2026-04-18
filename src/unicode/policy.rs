use std::{
    borrow::Cow,
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use caseless::Caseless;
use icu_casemap::{CaseMapper, CaseMapperBorrowed};
use unicode_normalization::UnicodeNormalization;

use crate::policy::{KeyPolicy, Policy};

/// Unicode canonical caseless comparison.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeNfc;

/// Unicode compatibility caseless comparison.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeNfkc;

/// Turkic-aware Unicode canonical caseless comparison.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeTurkicNfc;

/// Turkic-aware Unicode compatibility caseless comparison.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnicodeTurkicNfkc;

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

fn turkic_fold(value: &str) -> Cow<'_, str> {
    TURKIC_CASE_MAPPER.fold_turkic_string(value)
}

fn turkic_canonical_key(value: &str) -> Box<str> {
    turkic_fold(value)
        .chars()
        .nfd()
        .collect::<String>()
        .into_boxed_str()
}

fn turkic_compatibility_key(value: &str) -> Box<str> {
    turkic_fold(value)
        .chars()
        .nfkd()
        .collect::<String>()
        .into_boxed_str()
}

impl Policy for UnicodeNfc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs.chars().canonical_caseless_match(rhs.chars())
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
}

impl Policy for UnicodeNfkc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        lhs.chars().compatibility_caseless_match(rhs.chars())
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
}

impl Policy for UnicodeTurkicNfc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        let lhs = turkic_fold(lhs);
        let rhs = turkic_fold(rhs);
        lhs.chars().nfd().eq(rhs.chars().nfd())
    }

    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        let lhs = turkic_fold(lhs);
        let rhs = turkic_fold(rhs);
        cmp_chars(lhs.chars().nfd(), rhs.chars().nfd())
    }

    fn hash<H: Hasher>(value: &str, state: &mut H) {
        let value = turkic_fold(value);
        hash_chars(value.chars().nfd(), state);
    }
}

impl KeyPolicy for UnicodeTurkicNfc {
    fn into_key(value: Box<str>) -> Box<str> {
        turkic_canonical_key(&value)
    }
}

impl Policy for UnicodeTurkicNfkc {
    fn eq(lhs: &str, rhs: &str) -> bool {
        let lhs = turkic_fold(lhs);
        let rhs = turkic_fold(rhs);
        lhs.chars().nfkd().eq(rhs.chars().nfkd())
    }

    fn cmp(lhs: &str, rhs: &str) -> Ordering {
        let lhs = turkic_fold(lhs);
        let rhs = turkic_fold(rhs);
        cmp_chars(lhs.chars().nfkd(), rhs.chars().nfkd())
    }

    fn hash<H: Hasher>(value: &str, state: &mut H) {
        let value = turkic_fold(value);
        hash_chars(value.chars().nfkd(), state);
    }
}

impl KeyPolicy for UnicodeTurkicNfkc {
    fn into_key(value: Box<str>) -> Box<str> {
        turkic_compatibility_key(&value)
    }
}
