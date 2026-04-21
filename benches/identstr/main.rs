#![allow(
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::wildcard_imports
)]

use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Write,
    hash::{DefaultHasher, Hash, Hasher},
    hint::black_box,
    rc::Rc,
    sync::Arc,
};

use compact_str::CompactString;
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use identstr::{ArcSpill, BoxSpill, IdentStr, Key, Quote, RcSpill, policy};
use uncased::Uncased;

type AsciiIdent = IdentStr<Quote, policy::Ascii, BoxSpill>;
type AsciiArcIdent = IdentStr<Quote, policy::Ascii, ArcSpill>;
type AsciiKey = Key<policy::Ascii>;
type AsciiEntry = (AsciiIdent, usize);
type StringKey = String;
type StaticKey = &'static str;

#[derive(Clone)]
struct NaiveBoxIdent {
    value: Box<str>,
    quote: Option<Quote>,
}

#[derive(Clone)]
struct NaiveCompactIdent {
    value: CompactString,
    quote: Option<Quote>,
}

#[derive(Clone)]
struct NaiveArcIdent {
    value: Arc<str>,
    quote: Option<Quote>,
}

#[derive(Clone)]
struct NaiveRcIdent {
    value: Rc<str>,
    quote: Option<Quote>,
}

#[derive(Clone)]
struct NaiveUncasedIdent {
    value: Uncased<'static>,
    quote: Option<Quote>,
}

impl NaiveBoxIdent {
    fn new(value: &str, quote: Option<Quote>) -> Self {
        Self {
            value: Box::<str>::from(value),
            quote,
        }
    }

    fn eq(&self, other: &Self) -> bool {
        black_box(self.quote);
        black_box(other.quote);
        self.value.eq_ignore_ascii_case(&other.value)
    }

    fn as_str(&self) -> &str {
        &self.value
    }
}

impl NaiveCompactIdent {
    fn new(value: &str, quote: Option<Quote>) -> Self {
        Self {
            value: CompactString::from(value),
            quote,
        }
    }

    fn from_string(value: String, quote: Option<Quote>) -> Self {
        Self {
            value: CompactString::from(value),
            quote,
        }
    }

    fn eq(&self, other: &Self) -> bool {
        black_box(self.quote);
        black_box(other.quote);
        self.value.eq_ignore_ascii_case(&other.value)
    }

    fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

impl NaiveArcIdent {
    fn new(value: &str, quote: Option<Quote>) -> Self {
        Self {
            value: Arc::<str>::from(value),
            quote,
        }
    }

    fn from_string(value: String, quote: Option<Quote>) -> Self {
        Self {
            value: Arc::<str>::from(value),
            quote,
        }
    }

    fn eq(&self, other: &Self) -> bool {
        black_box(self.quote);
        black_box(other.quote);
        self.value.eq_ignore_ascii_case(&other.value)
    }

    fn as_str(&self) -> &str {
        &self.value
    }
}

impl NaiveRcIdent {
    fn new(value: &str, quote: Option<Quote>) -> Self {
        Self {
            value: Rc::<str>::from(value),
            quote,
        }
    }

    fn from_string(value: String, quote: Option<Quote>) -> Self {
        Self {
            value: Rc::<str>::from(value),
            quote,
        }
    }

    fn eq(&self, other: &Self) -> bool {
        black_box(self.quote);
        black_box(other.quote);
        self.value.eq_ignore_ascii_case(&other.value)
    }

    fn as_str(&self) -> &str {
        &self.value
    }
}

impl NaiveUncasedIdent {
    fn new(value: &str, quote: Option<Quote>) -> Self {
        Self {
            value: Uncased::from(value.to_owned()),
            quote,
        }
    }

    fn from_string(value: String, quote: Option<Quote>) -> Self {
        Self {
            value: Uncased::from(value),
            quote,
        }
    }

    fn eq(&self, other: &Self) -> bool {
        black_box(self.quote);
        black_box(other.quote);
        self.value == other.value
    }

    fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

fn hash_ascii_bytes<H: Hasher>(value: &str, state: &mut H) {
    value.len().hash(state);
    for byte in value.bytes() {
        state.write_u8(byte.to_ascii_lowercase());
    }
}

fn naive_ascii_key(value: &str) -> Box<str> {
    if value.bytes().any(|byte| byte.is_ascii_uppercase()) {
        value.to_ascii_lowercase().into_boxed_str()
    } else {
        Box::<str>::from(value)
    }
}

fn naive_arc_key(value: &str) -> Arc<str> {
    if value.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Arc::<str>::from(value.to_ascii_lowercase())
    } else {
        Arc::<str>::from(value)
    }
}

fn naive_rc_key(value: &str) -> Rc<str> {
    if value.bytes().any(|byte| byte.is_ascii_uppercase()) {
        Rc::<str>::from(value.to_ascii_lowercase())
    } else {
        Rc::<str>::from(value)
    }
}

fn naive_compact_key(value: &str) -> CompactString {
    if value.bytes().any(|byte| byte.is_ascii_uppercase()) {
        CompactString::from(value.to_ascii_lowercase())
    } else {
        CompactString::from(value)
    }
}

fn naive_uncased_key(value: &str) -> Uncased<'static> {
    Uncased::from(value.to_owned())
}

const MAP_NAMES_SHORT: &[&str] = &[
    "Users",
    "orders",
    "Customer_ID",
    "line_items",
    "Sessions",
    "role_map",
    "audit_log",
    "TeamName",
];

const MAP_NAMES_SHORT_LOWER: &[&str] = &[
    "users",
    "orders",
    "customer_id",
    "line_items",
    "sessions",
    "role_map",
    "audit_log",
    "teamname",
];

const MAP_NAMES_LONG: &[&str] = &[
    "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE_ALPHA",
    "this_identifier_name_is_long_enough_to_spill_out_of_line_beta",
    "CustomerIdentifierNameThatSpillsOutOfLineGamma",
    "line_item_identifier_name_that_spills_out_of_line_delta",
    "SESSION_IDENTIFIER_NAME_THAT_SPILLS_OUT_OF_LINE_EPSILON",
    "role_mapping_identifier_name_that_spills_out_of_line_zeta",
    "audit_log_identifier_name_that_spills_out_of_line_eta",
    "TeamIdentifierNameThatSpillsOutOfLineTheta",
];

const MAP_NAMES_LONG_LOWER: &[&str] = &[
    "this_identifier_name_is_long_enough_to_spill_out_of_line_alpha",
    "this_identifier_name_is_long_enough_to_spill_out_of_line_beta",
    "customeridentifiernamethatspillsoutoflinegamma",
    "line_item_identifier_name_that_spills_out_of_line_delta",
    "session_identifier_name_that_spills_out_of_line_epsilon",
    "role_mapping_identifier_name_that_spills_out_of_line_zeta",
    "audit_log_identifier_name_that_spills_out_of_line_eta",
    "teamidentifiernamethatspillsoutoflinetheta",
];

fn consume_text(value: &str) {
    black_box(value.as_ptr());
    black_box(value.len());
}

fn consume_quoted_text(quote: Option<Quote>, value: &str) {
    black_box(quote.map(Quote::open));
    black_box(value.as_ptr());
    black_box(value.len());
    black_box(quote.map(Quote::close));
}

fn write_quoted_naive(quote: Option<Quote>, value: &str, buffer: &mut String) {
    let Some(quote) = quote else {
        buffer.push_str(value);
        return;
    };

    let escape = quote.close();
    buffer.push(quote.open());
    for char in value.chars() {
        buffer.push(char);
        if char == escape {
            buffer.push(char);
        }
    }
    buffer.push(quote.close());
}

fn naive_quoted_string(quote: Option<Quote>, value: &str) -> String {
    let mut rendered = String::new();
    write_quoted_naive(quote, value, &mut rendered);
    rendered
}

fn eq_ident(
    lhs: &IdentStr<Quote, policy::Ascii, BoxSpill>,
    rhs: &IdentStr<Quote, policy::Ascii, BoxSpill>,
) -> bool {
    lhs == rhs
}

fn eq_naive_box(lhs: &NaiveBoxIdent, rhs: &NaiveBoxIdent) -> bool {
    lhs.eq(rhs)
}

fn eq_naive_arc(lhs: &NaiveArcIdent, rhs: &NaiveArcIdent) -> bool {
    lhs.eq(rhs)
}

fn eq_naive_rc(lhs: &NaiveRcIdent, rhs: &NaiveRcIdent) -> bool {
    lhs.eq(rhs)
}

fn eq_naive_compact(lhs: &NaiveCompactIdent, rhs: &NaiveCompactIdent) -> bool {
    lhs.eq(rhs)
}

fn eq_naive_uncased(lhs: &NaiveUncasedIdent, rhs: &NaiveUncasedIdent) -> bool {
    lhs.eq(rhs)
}

fn eq_std_ascii(lhs: &str, rhs: &str) -> bool {
    lhs.eq_ignore_ascii_case(rhs)
}

fn eq_uncased_ascii(lhs: &str, rhs: &str) -> bool {
    uncased::eq(lhs, rhs)
}

fn cmp_ascii(lhs: &str, rhs: &str) -> Ordering {
    let lhs = lhs.as_bytes();
    let rhs = rhs.as_bytes();
    let shared_len = lhs.len().min(rhs.len());

    let mut index = 0;
    while index < shared_len {
        let lhs = lhs[index].to_ascii_lowercase();
        let rhs = rhs[index].to_ascii_lowercase();
        if lhs != rhs {
            return lhs.cmp(&rhs);
        }
        index += 1;
    }

    lhs.len().cmp(&rhs.len())
}

fn cmp_ident(
    lhs: &IdentStr<Quote, policy::Ascii, BoxSpill>,
    rhs: &IdentStr<Quote, policy::Ascii, BoxSpill>,
) -> Ordering {
    lhs.cmp(rhs)
}

fn cmp_naive_box(lhs: &NaiveBoxIdent, rhs: &NaiveBoxIdent) -> Ordering {
    black_box(lhs.quote);
    black_box(rhs.quote);
    cmp_ascii(lhs.as_str(), rhs.as_str())
}

fn cmp_naive_arc(lhs: &NaiveArcIdent, rhs: &NaiveArcIdent) -> Ordering {
    black_box(lhs.quote);
    black_box(rhs.quote);
    cmp_ascii(lhs.as_str(), rhs.as_str())
}

fn cmp_naive_rc(lhs: &NaiveRcIdent, rhs: &NaiveRcIdent) -> Ordering {
    black_box(lhs.quote);
    black_box(rhs.quote);
    cmp_ascii(lhs.as_str(), rhs.as_str())
}

fn cmp_naive_compact(lhs: &NaiveCompactIdent, rhs: &NaiveCompactIdent) -> Ordering {
    black_box(lhs.quote);
    black_box(rhs.quote);
    cmp_ascii(lhs.as_str(), rhs.as_str())
}

fn cmp_naive_uncased(lhs: &NaiveUncasedIdent, rhs: &NaiveUncasedIdent) -> Ordering {
    black_box(lhs.quote);
    black_box(rhs.quote);
    lhs.value.cmp(&rhs.value)
}

mod backend;
mod compare;
mod construction;
mod lifecycle;
mod maps;
mod read;
mod repeat;
#[cfg(feature = "unicode")]
mod unicode;

criterion_group!(
    benches,
    construction::bench_construction,
    backend::bench_owned_backend_input,
    lifecycle::bench_conversion,
    lifecycle::bench_clone,
    backend::bench_drop,
    maps::bench_maps,
    repeat::bench_repeat_key,
    read::bench_read,
    read::bench_render_quoted,
    compare::bench_compare,
    backend::bench_mixed_backend_eq,
    compare::bench_hash
);
#[cfg(feature = "unicode")]
criterion_group!(unicode_benches, unicode::bench_unicode);

#[cfg(feature = "unicode")]
criterion_main!(benches, unicode_benches);
#[cfg(not(feature = "unicode"))]
criterion_main!(benches);
