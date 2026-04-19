#![allow(clippy::similar_names, clippy::too_many_lines)]

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

fn bench_borrowed_construction(c: &mut Criterion, short: &str, long: &str) {
    let mut group = c.benchmark_group("construct_borrowed_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(short));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(NaiveBoxIdent::new(short, None));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(NaiveArcIdent::new(short, None));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(NaiveRcIdent::new(short, None));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(NaiveCompactIdent::new(short, None));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(NaiveUncasedIdent::new(short, None));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("construct_borrowed_quoted_inline");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                short,
                Quote::Double,
            ));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(NaiveBoxIdent::new(short, Some(Quote::Double)));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(NaiveArcIdent::new(short, Some(Quote::Double)));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(NaiveRcIdent::new(short, Some(Quote::Double)));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(NaiveCompactIdent::new(short, Some(Quote::Double)));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(NaiveUncasedIdent::new(short, Some(Quote::Double)));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("construct_borrowed_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(long));
        });
    });
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, ArcSpill>::new(long));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(NaiveBoxIdent::new(long, None));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(NaiveArcIdent::new(long, None));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(NaiveRcIdent::new(long, None));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(NaiveCompactIdent::new(long, None));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(NaiveUncasedIdent::new(long, None));
        });
    });
    group.finish();
}

fn bench_owned_construction_short(c: &mut Criterion, short_owned: &str) {
    let mut group = c.benchmark_group("construct_owned_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::from(black_box(
                short_owned.to_owned(),
            )));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(NaiveBoxIdent {
                value: black_box(short_owned.to_owned()).into_boxed_str(),
                quote: None,
            });
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(NaiveArcIdent::from_string(
                black_box(short_owned.to_owned()),
                None,
            ));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(NaiveRcIdent::from_string(
                black_box(short_owned.to_owned()),
                None,
            ));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(NaiveCompactIdent::from_string(
                black_box(short_owned.to_owned()),
                None,
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(NaiveUncasedIdent::from_string(
                black_box(short_owned.to_owned()),
                None,
            ));
        });
    });
    group.finish();
}

fn bench_owned_construction_quoted_inline(c: &mut Criterion, short_owned: &str) {
    let mut group = c.benchmark_group("construct_owned_quoted_inline");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                black_box(short_owned.to_owned()),
                Quote::Double,
            ));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(NaiveBoxIdent {
                value: black_box(short_owned.to_owned()).into_boxed_str(),
                quote: Some(Quote::Double),
            });
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(NaiveArcIdent::from_string(
                black_box(short_owned.to_owned()),
                Some(Quote::Double),
            ));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(NaiveRcIdent::from_string(
                black_box(short_owned.to_owned()),
                Some(Quote::Double),
            ));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(NaiveCompactIdent::from_string(
                black_box(short_owned.to_owned()),
                Some(Quote::Double),
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(NaiveUncasedIdent::from_string(
                black_box(short_owned.to_owned()),
                Some(Quote::Double),
            ));
        });
    });
    group.finish();
}

fn bench_owned_construction_long(c: &mut Criterion, long_owned: &str) {
    let mut group = c.benchmark_group("construct_owned_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::from(black_box(
                long_owned.to_owned(),
            )));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(NaiveBoxIdent {
                value: black_box(long_owned.to_owned()).into_boxed_str(),
                quote: None,
            });
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(NaiveArcIdent::from_string(
                black_box(long_owned.to_owned()),
                None,
            ));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(NaiveRcIdent::from_string(
                black_box(long_owned.to_owned()),
                None,
            ));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(NaiveCompactIdent::from_string(
                black_box(long_owned.to_owned()),
                None,
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(NaiveUncasedIdent::from_string(
                black_box(long_owned.to_owned()),
                None,
            ));
        });
    });
    group.finish();
}

fn bench_owned_construction(c: &mut Criterion, short: &str, long: &str) {
    bench_owned_construction_short(c, short);
    bench_owned_construction_quoted_inline(c, short);
    bench_owned_construction_long(c, long);
}

fn bench_construction(c: &mut Criterion) {
    let short = "customer_id";
    let long = "this_identifier_name_is_long_enough_to_spill_out_of_line";

    bench_borrowed_construction(c, short, long);
    bench_owned_construction(c, short, long);
}

fn bench_clone(c: &mut Criterion) {
    let short_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new("customer_id");
    let short_box = NaiveBoxIdent::new("customer_id", Some(Quote::Double));
    let short_arc = NaiveArcIdent::new("customer_id", Some(Quote::Double));
    let short_rc = NaiveRcIdent::new("customer_id", Some(Quote::Double));
    let short_compact = NaiveCompactIdent::new("customer_id", Some(Quote::Double));
    let short_uncased = NaiveUncasedIdent::new("customer_id", Some(Quote::Double));

    let long_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
    );
    let long_shared = IdentStr::<Quote, policy::Ascii, ArcSpill>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
    );
    let long_box = NaiveBoxIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_arc = NaiveArcIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_rc = NaiveRcIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_compact = NaiveCompactIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_uncased = NaiveUncasedIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );

    let mut group = c.benchmark_group("clone_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(short_ident.clone());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(short_box.clone());
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(short_arc.clone());
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(short_rc.clone());
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(short_compact.clone());
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(short_uncased.clone());
        });
    });
    group.finish();

    let mut group = c.benchmark_group("clone_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(long_ident.clone());
        });
    });
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            black_box(long_shared.clone());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(long_box.clone());
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(long_arc.clone());
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(long_rc.clone());
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(long_compact.clone());
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(long_uncased.clone());
        });
    });
    group.finish();
}

fn bench_conversion(c: &mut Criterion) {
    let short = "customer_id";
    let long = "this_identifier_name_is_long_enough_to_spill_out_of_line";

    let mut group = c.benchmark_group("into_boxed_str");
    group.bench_function("identstr_inline_box", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(short);
            black_box(Box::<str>::from(ident));
        });
    });
    group.bench_function("identstr_spill_box", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(long);
            black_box(Box::<str>::from(ident));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("into_arc_str");
    group.bench_function("identstr_inline_arc", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, ArcSpill>::new(short);
            black_box(Arc::<str>::from(ident));
        });
    });
    group.bench_function("identstr_spill_arc", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, ArcSpill>::new(long);
            black_box(Arc::<str>::from(ident));
        });
    });
    group.finish();
}

fn bench_read_text(c: &mut Criterion) {
    let short_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new("customer_id");
    let short_box = NaiveBoxIdent::new("customer_id", None);
    let short_arc = NaiveArcIdent::new("customer_id", None);
    let short_rc = NaiveRcIdent::new("customer_id", None);
    let short_compact = NaiveCompactIdent::new("customer_id", None);
    let short_uncased = NaiveUncasedIdent::new("customer_id", None);

    let mut group = c.benchmark_group("read_text_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            consume_text(black_box(&short_ident).as_str());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            consume_text(black_box(&short_box).as_str());
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            consume_text(black_box(&short_arc).as_str());
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            consume_text(black_box(&short_rc).as_str());
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            consume_text(black_box(&short_compact).as_str());
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            consume_text(black_box(&short_uncased).as_str());
        });
    });
    group.finish();

    let long_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
    );
    let long_box = NaiveBoxIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_arc = NaiveArcIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_rc = NaiveRcIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_compact = NaiveCompactIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let long_uncased = NaiveUncasedIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );

    let mut group = c.benchmark_group("read_text_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            consume_text(black_box(&long_ident).as_str());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            consume_text(black_box(&long_box).as_str());
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            consume_text(black_box(&long_arc).as_str());
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            consume_text(black_box(&long_rc).as_str());
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            consume_text(black_box(&long_compact).as_str());
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            consume_text(black_box(&long_uncased).as_str());
        });
    });
    group.finish();
}

fn bench_read_quoted_parts(c: &mut Criterion) {
    let quoted_ident =
        IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote("Users", Quote::Bracket);
    let quoted_box = NaiveBoxIdent::new("Users", Some(Quote::Bracket));
    let quoted_arc = NaiveArcIdent::new("Users", Some(Quote::Bracket));
    let quoted_rc = NaiveRcIdent::new("Users", Some(Quote::Bracket));
    let quoted_compact = NaiveCompactIdent::new("Users", Some(Quote::Bracket));
    let quoted_uncased = NaiveUncasedIdent::new("Users", Some(Quote::Bracket));

    let mut group = c.benchmark_group("read_quoted_parts_inline");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_ident);
            consume_quoted_text(ident.quote(), ident.as_str());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_box);
            consume_quoted_text(ident.quote, ident.as_str());
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_arc);
            consume_quoted_text(ident.quote, ident.as_str());
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_rc);
            consume_quoted_text(ident.quote, ident.as_str());
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_compact);
            consume_quoted_text(ident.quote, ident.as_str());
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_uncased);
            consume_quoted_text(ident.quote, ident.as_str());
        });
    });
    group.finish();
}

fn bench_read_key(c: &mut Criterion) {
    let quoted_ident =
        IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote("Users", Quote::Bracket);
    let quoted_box = NaiveBoxIdent::new("Users", Some(Quote::Bracket));
    let quoted_arc = NaiveArcIdent::new("Users", Some(Quote::Bracket));
    let quoted_rc = NaiveRcIdent::new("Users", Some(Quote::Bracket));
    let quoted_compact = NaiveCompactIdent::new("Users", Some(Quote::Bracket));
    let quoted_uncased = NaiveUncasedIdent::new("Users", Some(Quote::Bracket));

    let mut group = c.benchmark_group("read_key_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(black_box(&quoted_ident).to_key());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&quoted_box).as_str()));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&quoted_arc).as_str()));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&quoted_rc).as_str()));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&quoted_compact).as_str()));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&quoted_uncased).as_str()));
        });
    });
    group.finish();

    let upper_long = "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE";
    let key_long_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(upper_long);
    let key_long_box = NaiveBoxIdent::new(upper_long, None);
    let key_long_arc = NaiveArcIdent::new(upper_long, None);
    let key_long_rc = NaiveRcIdent::new(upper_long, None);
    let key_long_compact = NaiveCompactIdent::new(upper_long, None);
    let key_long_uncased = NaiveUncasedIdent::new(upper_long, None);

    let mut group = c.benchmark_group("read_key_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(black_box(&key_long_ident).to_key());
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&key_long_box).as_str()));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&key_long_arc).as_str()));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&key_long_rc).as_str()));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&key_long_compact).as_str()));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(naive_ascii_key(black_box(&key_long_uncased).as_str()));
        });
    });
    group.finish();
}

fn bench_read(c: &mut Criterion) {
    bench_read_text(c);
    bench_read_quoted_parts(c);
    bench_read_key(c);
}

fn bench_render_quoted(c: &mut Criterion) {
    let unquoted_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new("Users");
    let unquoted_naive = NaiveBoxIdent::new("Users", None);
    let quoted_ident =
        IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote("User\"Table", Quote::Double);
    let quoted_naive = NaiveBoxIdent::new("User\"Table", Some(Quote::Double));
    let bracket_ident =
        IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote("User]Table", Quote::Bracket);
    let bracket_naive = NaiveBoxIdent::new("User]Table", Some(Quote::Bracket));

    let mut group = c.benchmark_group("render_quoted_write");
    group.bench_function("identstr_unquoted", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            write!(
                &mut buffer,
                "{}",
                black_box(&unquoted_ident).display_quoted()
            )
            .expect("write to String");
            black_box(&buffer);
        });
    });
    group.bench_function("naive_unquoted", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            let ident = black_box(&unquoted_naive);
            write_quoted_naive(ident.quote, ident.as_str(), &mut buffer);
            black_box(&buffer);
        });
    });
    group.bench_function("identstr_double_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            write!(&mut buffer, "{}", black_box(&quoted_ident).display_quoted())
                .expect("write to String");
            black_box(&buffer);
        });
    });
    group.bench_function("naive_double_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            let ident = black_box(&quoted_naive);
            write_quoted_naive(ident.quote, ident.as_str(), &mut buffer);
            black_box(&buffer);
        });
    });
    group.bench_function("identstr_bracket_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            write!(
                &mut buffer,
                "{}",
                black_box(&bracket_ident).display_quoted()
            )
            .expect("write to String");
            black_box(&buffer);
        });
    });
    group.bench_function("naive_bracket_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            let ident = black_box(&bracket_naive);
            write_quoted_naive(ident.quote, ident.as_str(), &mut buffer);
            black_box(&buffer);
        });
    });
    group.finish();

    let mut group = c.benchmark_group("render_quoted_write_direct");
    group.bench_function("identstr_unquoted", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            black_box(&unquoted_ident)
                .write_quoted(&mut buffer)
                .expect("write to String");
            black_box(&buffer);
        });
    });
    group.bench_function("naive_unquoted", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            let ident = black_box(&unquoted_naive);
            write_quoted_naive(ident.quote, ident.as_str(), &mut buffer);
            black_box(&buffer);
        });
    });
    group.bench_function("identstr_double_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            black_box(&quoted_ident)
                .write_quoted(&mut buffer)
                .expect("write to String");
            black_box(&buffer);
        });
    });
    group.bench_function("naive_double_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            let ident = black_box(&quoted_naive);
            write_quoted_naive(ident.quote, ident.as_str(), &mut buffer);
            black_box(&buffer);
        });
    });
    group.bench_function("identstr_bracket_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            black_box(&bracket_ident)
                .write_quoted(&mut buffer)
                .expect("write to String");
            black_box(&buffer);
        });
    });
    group.bench_function("naive_bracket_escaped", |b| {
        let mut buffer = String::new();
        b.iter(|| {
            buffer.clear();
            let ident = black_box(&bracket_naive);
            write_quoted_naive(ident.quote, ident.as_str(), &mut buffer);
            black_box(&buffer);
        });
    });
    group.finish();

    let mut group = c.benchmark_group("render_quoted_string");
    group.bench_function("identstr_unquoted", |b| {
        b.iter(|| {
            black_box(black_box(&unquoted_ident).to_quoted_string());
        });
    });
    group.bench_function("naive_unquoted", |b| {
        b.iter(|| {
            let ident = black_box(&unquoted_naive);
            black_box(naive_quoted_string(ident.quote, ident.as_str()));
        });
    });
    group.bench_function("identstr_double_escaped", |b| {
        b.iter(|| {
            black_box(black_box(&quoted_ident).to_quoted_string());
        });
    });
    group.bench_function("naive_double_escaped", |b| {
        b.iter(|| {
            let ident = black_box(&quoted_naive);
            black_box(naive_quoted_string(ident.quote, ident.as_str()));
        });
    });
    group.bench_function("identstr_bracket_escaped", |b| {
        b.iter(|| {
            black_box(black_box(&bracket_ident).to_quoted_string());
        });
    });
    group.bench_function("naive_bracket_escaped", |b| {
        b.iter(|| {
            let ident = black_box(&bracket_naive);
            black_box(naive_quoted_string(ident.quote, ident.as_str()));
        });
    });
    group.finish();
}

fn bench_compare_short(c: &mut Criterion) {
    let lhs_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote("Users", Quote::Double);
    let rhs_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new("users");
    let lhs_box = NaiveBoxIdent::new("Users", Some(Quote::Double));
    let rhs_box = NaiveBoxIdent::new("users", None);
    let lhs_arc = NaiveArcIdent::new("Users", Some(Quote::Double));
    let rhs_arc = NaiveArcIdent::new("users", None);
    let lhs_rc = NaiveRcIdent::new("Users", Some(Quote::Double));
    let rhs_rc = NaiveRcIdent::new("users", None);
    let lhs_compact = NaiveCompactIdent::new("Users", Some(Quote::Double));
    let rhs_compact = NaiveCompactIdent::new("users", None);
    let lhs_uncased = NaiveUncasedIdent::new("Users", Some(Quote::Double));
    let rhs_uncased = NaiveUncasedIdent::new("users", None);

    let mut group = c.benchmark_group("eq_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(eq_ident(black_box(&lhs_ident), black_box(&rhs_ident)));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(eq_naive_box(black_box(&lhs_box), black_box(&rhs_box)));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(eq_naive_arc(black_box(&lhs_arc), black_box(&rhs_arc)));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(eq_naive_rc(black_box(&lhs_rc), black_box(&rhs_rc)));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(eq_naive_compact(
                black_box(&lhs_compact),
                black_box(&rhs_compact),
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(eq_naive_uncased(
                black_box(&lhs_uncased),
                black_box(&rhs_uncased),
            ));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("cmp_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(cmp_ident(black_box(&lhs_ident), black_box(&rhs_ident)));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(cmp_naive_box(black_box(&lhs_box), black_box(&rhs_box)));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(cmp_naive_arc(black_box(&lhs_arc), black_box(&rhs_arc)));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(cmp_naive_rc(black_box(&lhs_rc), black_box(&rhs_rc)));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(cmp_naive_compact(
                black_box(&lhs_compact),
                black_box(&rhs_compact),
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(cmp_naive_uncased(
                black_box(&lhs_uncased),
                black_box(&rhs_uncased),
            ));
        });
    });
    group.finish();
}

fn bench_compare_long(c: &mut Criterion) {
    let lhs_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
    );
    let rhs_ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
    );
    let lhs_box = NaiveBoxIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let rhs_box = NaiveBoxIdent::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
        None,
    );
    let lhs_arc = NaiveArcIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let rhs_arc = NaiveArcIdent::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
        None,
    );
    let lhs_rc = NaiveRcIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let rhs_rc = NaiveRcIdent::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
        None,
    );
    let lhs_compact = NaiveCompactIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let rhs_compact = NaiveCompactIdent::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
        None,
    );
    let lhs_uncased = NaiveUncasedIdent::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
        None,
    );
    let rhs_uncased = NaiveUncasedIdent::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
        None,
    );

    let mut group = c.benchmark_group("eq_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(eq_ident(black_box(&lhs_ident), black_box(&rhs_ident)));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(eq_naive_box(black_box(&lhs_box), black_box(&rhs_box)));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(eq_naive_arc(black_box(&lhs_arc), black_box(&rhs_arc)));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(eq_naive_rc(black_box(&lhs_rc), black_box(&rhs_rc)));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(eq_naive_compact(
                black_box(&lhs_compact),
                black_box(&rhs_compact),
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(eq_naive_uncased(
                black_box(&lhs_uncased),
                black_box(&rhs_uncased),
            ));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("cmp_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            black_box(cmp_ident(black_box(&lhs_ident), black_box(&rhs_ident)));
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            black_box(cmp_naive_box(black_box(&lhs_box), black_box(&rhs_box)));
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            black_box(cmp_naive_arc(black_box(&lhs_arc), black_box(&rhs_arc)));
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            black_box(cmp_naive_rc(black_box(&lhs_rc), black_box(&rhs_rc)));
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            black_box(cmp_naive_compact(
                black_box(&lhs_compact),
                black_box(&rhs_compact),
            ));
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            black_box(cmp_naive_uncased(
                black_box(&lhs_uncased),
                black_box(&rhs_uncased),
            ));
        });
    });
    group.finish();
}

fn bench_compare(c: &mut Criterion) {
    bench_compare_short(c);
    bench_compare_long(c);

    let mut group = c.benchmark_group("ascii_eq_impl_short");
    group.bench_function("std", |b| {
        b.iter(|| {
            black_box(eq_std_ascii(
                black_box("customer_id"),
                black_box("CuStOmEr_Id"),
            ));
        });
    });
    group.bench_function("uncased", |b| {
        b.iter(|| {
            black_box(eq_uncased_ascii(
                black_box("customer_id"),
                black_box("CuStOmEr_Id"),
            ));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("ascii_eq_impl_long");
    group.bench_function("std", |b| {
        b.iter(|| {
            black_box(eq_std_ascii(
                black_box("this_identifier_name_is_long_enough_to_spill_out_of_line"),
                black_box("ThIs_IdEnTiFiEr_NaMe_Is_LoNg_EnOuGh_To_SpIlL_oUt_Of_LiNe"),
            ));
        });
    });
    group.bench_function("uncased", |b| {
        b.iter(|| {
            black_box(eq_uncased_ascii(
                black_box("this_identifier_name_is_long_enough_to_spill_out_of_line"),
                black_box("ThIs_IdEnTiFiEr_NaMe_Is_LoNg_EnOuGh_To_SpIlL_oUt_Of_LiNe"),
            ));
        });
    });
    group.finish();
}

fn bench_hash(c: &mut Criterion) {
    for (label, value) in [
        ("short", "Users"),
        (
            "long",
            "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
        ),
    ] {
        let ident = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(value);
        let naive_box = NaiveBoxIdent::new(value, None);
        let naive_arc = NaiveArcIdent::new(value, None);
        let naive_rc = NaiveRcIdent::new(value, None);
        let naive_compact = NaiveCompactIdent::new(value, None);
        let naive_uncased = NaiveUncasedIdent::new(value, None);

        let mut group = c.benchmark_group("hash");
        group.bench_with_input(
            BenchmarkId::new("identstr_box", label),
            &ident,
            |b, ident| {
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    black_box(ident).hash(&mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("naive_box", label),
            &naive_box,
            |b, ident| {
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    hash_ascii_bytes(black_box(&*ident.value), &mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("naive_arc", label),
            &naive_arc,
            |b, ident| {
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    hash_ascii_bytes(black_box(&*ident.value), &mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("naive_rc", label),
            &naive_rc,
            |b, ident| {
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    hash_ascii_bytes(black_box(&*ident.value), &mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("naive_compact", label),
            &naive_compact,
            |b, ident| {
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    hash_ascii_bytes(black_box(ident.value.as_str()), &mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("naive_uncased", label),
            &naive_uncased,
            |b, ident| {
                b.iter(|| {
                    let mut hasher = DefaultHasher::new();
                    black_box(ident.value.as_uncased_str()).hash(&mut hasher);
                    black_box(hasher.finish());
                });
            },
        );
        group.finish();
    }
}

fn bench_owned_backend_input(c: &mut Criterion) {
    let short = "Users";
    let long = "this_identifier_name_is_long_enough_to_spill_out_of_line";

    let mut group = c.benchmark_group("construct_owned_backend_short");
    group.bench_function("identstr_box", |b| {
        b.iter_batched(
            || Box::<str>::from(short),
            |value| black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(value)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(short),
            |value| black_box(IdentStr::<Quote, policy::Ascii, ArcSpill>::new(value)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(short),
            |value| black_box(IdentStr::<Quote, policy::Ascii, RcSpill>::new(value)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_box", |b| {
        b.iter_batched(
            || Box::<str>::from(short),
            |value| black_box((value, None::<Quote>)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(short),
            |value| black_box((value, None::<Quote>)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(short),
            |value| black_box((value, None::<Quote>)),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("construct_owned_backend_short_quoted");
    group.bench_function("identstr_box", |b| {
        b.iter_batched(
            || Box::<str>::from(short),
            |value| {
                black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                    value,
                    Quote::Double,
                ))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(short),
            |value| {
                black_box(IdentStr::<Quote, policy::Ascii, ArcSpill>::with_quote(
                    value,
                    Quote::Double,
                ))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(short),
            |value| {
                black_box(IdentStr::<Quote, policy::Ascii, RcSpill>::with_quote(
                    value,
                    Quote::Double,
                ))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_box", |b| {
        b.iter_batched(
            || Box::<str>::from(short),
            |value| black_box((value, Some(Quote::Double))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(short),
            |value| black_box((value, Some(Quote::Double))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(short),
            |value| black_box((value, Some(Quote::Double))),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("construct_owned_backend_long");
    group.bench_function("identstr_box", |b| {
        b.iter_batched(
            || Box::<str>::from(long),
            |value| black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(value)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(long),
            |value| black_box(IdentStr::<Quote, policy::Ascii, ArcSpill>::new(value)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(long),
            |value| black_box(IdentStr::<Quote, policy::Ascii, RcSpill>::new(value)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_box", |b| {
        b.iter_batched(
            || Box::<str>::from(long),
            |value| black_box((value, None::<Quote>)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(long),
            |value| black_box((value, None::<Quote>)),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(long),
            |value| black_box((value, None::<Quote>)),
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("construct_owned_backend_long_quoted");
    group.bench_function("identstr_box", |b| {
        b.iter_batched(
            || Box::<str>::from(long),
            |value| {
                black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                    value,
                    Quote::Double,
                ))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(long),
            |value| {
                black_box(IdentStr::<Quote, policy::Ascii, ArcSpill>::with_quote(
                    value,
                    Quote::Double,
                ))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(long),
            |value| {
                black_box(IdentStr::<Quote, policy::Ascii, RcSpill>::with_quote(
                    value,
                    Quote::Double,
                ))
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_box", |b| {
        b.iter_batched(
            || Box::<str>::from(long),
            |value| black_box((value, Some(Quote::Double))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_arc", |b| {
        b.iter_batched(
            || Arc::<str>::from(long),
            |value| black_box((value, Some(Quote::Double))),
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_rc", |b| {
        b.iter_batched(
            || Rc::<str>::from(long),
            |value| black_box((value, Some(Quote::Double))),
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_mixed_backend_eq(c: &mut Criterion) {
    let lhs_short = IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote("Users", Quote::Double);
    let rhs_short_arc = IdentStr::<Quote, policy::Ascii, ArcSpill>::new("users");
    let rhs_short_rc = IdentStr::<Quote, policy::Ascii, RcSpill>::new("users");

    let mut group = c.benchmark_group("eq_mixed_backend_short");
    group.bench_function("box_arc", |b| {
        b.iter(|| {
            black_box(black_box(&lhs_short) == black_box(&rhs_short_arc));
        });
    });
    group.bench_function("box_rc", |b| {
        b.iter(|| {
            black_box(black_box(&lhs_short) == black_box(&rhs_short_rc));
        });
    });
    group.finish();

    let lhs_long = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
    );
    let rhs_long_arc = IdentStr::<Quote, policy::Ascii, ArcSpill>::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
    );
    let rhs_long_rc = IdentStr::<Quote, policy::Ascii, RcSpill>::new(
        "THIS_IDENTIFIER_NAME_IS_LONG_ENOUGH_TO_SPILL_OUT_OF_LINE",
    );

    let mut group = c.benchmark_group("eq_mixed_backend_long");
    group.bench_function("box_arc", |b| {
        b.iter(|| {
            black_box(black_box(&lhs_long) == black_box(&rhs_long_arc));
        });
    });
    group.bench_function("box_rc", |b| {
        b.iter(|| {
            black_box(black_box(&lhs_long) == black_box(&rhs_long_rc));
        });
    });
    group.finish();
}

fn bench_drop(c: &mut Criterion) {
    let short = "customer_id";
    let long = "this_identifier_name_is_long_enough_to_spill_out_of_line";

    let mut group = c.benchmark_group("drop_short");
    group.bench_function("identstr_box", |b| {
        b.iter_batched(
            || IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(short, Quote::Double),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_box", |b| {
        b.iter_batched(
            || NaiveBoxIdent::new(short, Some(Quote::Double)),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_arc", |b| {
        b.iter_batched(
            || NaiveArcIdent::new(short, Some(Quote::Double)),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_rc", |b| {
        b.iter_batched(
            || NaiveRcIdent::new(short, Some(Quote::Double)),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_compact", |b| {
        b.iter_batched(
            || NaiveCompactIdent::new(short, Some(Quote::Double)),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_uncased", |b| {
        b.iter_batched(
            || NaiveUncasedIdent::new(short, Some(Quote::Double)),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.finish();

    let mut group = c.benchmark_group("drop_long");
    group.bench_function("identstr_box", |b| {
        b.iter_batched(
            || IdentStr::<Quote, policy::Ascii, BoxSpill>::new(long),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("identstr_arc", |b| {
        b.iter_batched(
            || IdentStr::<Quote, policy::Ascii, ArcSpill>::new(long),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_box", |b| {
        b.iter_batched(
            || NaiveBoxIdent::new(long, None),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_arc", |b| {
        b.iter_batched(
            || NaiveArcIdent::new(long, None),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_rc", |b| {
        b.iter_batched(
            || NaiveRcIdent::new(long, None),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_compact", |b| {
        b.iter_batched(
            || NaiveCompactIdent::new(long, None),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.bench_function("naive_uncased", |b| {
        b.iter_batched(
            || NaiveUncasedIdent::new(long, None),
            drop,
            BatchSize::SmallInput,
        );
    });
    group.finish();
}

fn bench_maps(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_insert_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(
                    IdentStr::<Quote, policy::Ascii, BoxSpill>::new(*name),
                    index,
                );
            }
            black_box(map);
        });
    });
    group.bench_function("key", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(Key::<policy::Ascii>::new(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(naive_ascii_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(naive_arc_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(naive_rc_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(naive_compact_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(naive_uncased_key(name), index);
            }
            black_box(map);
        });
    });
    group.finish();

    let mut group = c.benchmark_group("map_insert_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(
                    IdentStr::<Quote, policy::Ascii, BoxSpill>::new(*name),
                    index,
                );
            }
            black_box(map);
        });
    });
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(
                    IdentStr::<Quote, policy::Ascii, ArcSpill>::new(*name),
                    index,
                );
            }
            black_box(map);
        });
    });
    group.bench_function("key", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(Key::<policy::Ascii>::new(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(naive_ascii_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(naive_arc_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(naive_rc_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(naive_compact_key(name), index);
            }
            black_box(map);
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_LONG.len());
            for (index, name) in MAP_NAMES_LONG.iter().enumerate() {
                map.insert(naive_uncased_key(name), index);
            }
            black_box(map);
        });
    });
    group.finish();

    let identstr_short_map: HashMap<IdentStr<Quote, policy::Ascii, BoxSpill>, usize> =
        MAP_NAMES_SHORT
            .iter()
            .enumerate()
            .map(|(index, name)| (IdentStr::new(*name), index))
            .collect();
    let key_short_map: HashMap<Key<policy::Ascii>, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (Key::new(name), index))
        .collect();
    let naive_box_short_map: HashMap<Box<str>, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_ascii_key(name), index))
        .collect();
    let naive_arc_short_map: HashMap<Arc<str>, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_arc_key(name), index))
        .collect();
    let naive_rc_short_map: HashMap<Rc<str>, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_rc_key(name), index))
        .collect();
    let naive_compact_short_map: HashMap<CompactString, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_compact_key(name), index))
        .collect();
    let naive_uncased_short_map: HashMap<Uncased<'static>, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_uncased_key(name), index))
        .collect();

    let identstr_short_queries: Vec<IdentStr<Quote, policy::Ascii, BoxSpill>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| IdentStr::new(name.to_ascii_lowercase()))
        .collect();
    let key_short_queries: Vec<Key<policy::Ascii>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| Key::new(&name.to_ascii_lowercase()))
        .collect();
    let naive_box_short_queries: Vec<Box<str>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| naive_ascii_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_arc_short_queries: Vec<Arc<str>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| naive_arc_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_rc_short_queries: Vec<Rc<str>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| naive_rc_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_compact_short_queries: Vec<CompactString> = MAP_NAMES_SHORT
        .iter()
        .map(|name| naive_compact_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_uncased_short_queries: Vec<Uncased<'static>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| naive_uncased_key(&name.to_ascii_lowercase()))
        .collect();

    let mut group = c.benchmark_group("map_lookup_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                sum += identstr_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("key", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &key_short_queries {
                sum += key_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_box_short_queries {
                sum += naive_box_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_arc_short_queries {
                sum += naive_arc_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_rc_short_queries {
                sum += naive_rc_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_compact_short_queries {
                sum += naive_compact_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_uncased_short_queries {
                sum += naive_uncased_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.finish();

    let identstr_long_map: HashMap<IdentStr<Quote, policy::Ascii, BoxSpill>, usize> =
        MAP_NAMES_LONG
            .iter()
            .enumerate()
            .map(|(index, name)| (IdentStr::new(*name), index))
            .collect();
    let identstr_long_arc_map: HashMap<IdentStr<Quote, policy::Ascii, ArcSpill>, usize> =
        MAP_NAMES_LONG
            .iter()
            .enumerate()
            .map(|(index, name)| {
                (
                    IdentStr::<Quote, policy::Ascii, ArcSpill>::new(*name),
                    index,
                )
            })
            .collect();
    let key_long_map: HashMap<Key<policy::Ascii>, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (Key::new(name), index))
        .collect();
    let naive_box_long_map: HashMap<Box<str>, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_ascii_key(name), index))
        .collect();
    let naive_arc_long_map: HashMap<Arc<str>, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_arc_key(name), index))
        .collect();
    let naive_rc_long_map: HashMap<Rc<str>, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_rc_key(name), index))
        .collect();
    let naive_compact_long_map: HashMap<CompactString, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_compact_key(name), index))
        .collect();
    let naive_uncased_long_map: HashMap<Uncased<'static>, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (naive_uncased_key(name), index))
        .collect();

    let identstr_long_queries: Vec<IdentStr<Quote, policy::Ascii, BoxSpill>> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::new(name.to_ascii_lowercase()))
        .collect();
    let identstr_long_arc_queries: Vec<IdentStr<Quote, policy::Ascii, ArcSpill>> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::<Quote, policy::Ascii, ArcSpill>::new(name.to_ascii_lowercase()))
        .collect();
    let key_long_queries: Vec<Key<policy::Ascii>> = MAP_NAMES_LONG
        .iter()
        .map(|name| Key::new(&name.to_ascii_lowercase()))
        .collect();
    let naive_box_long_queries: Vec<Box<str>> = MAP_NAMES_LONG
        .iter()
        .map(|name| naive_ascii_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_arc_long_queries: Vec<Arc<str>> = MAP_NAMES_LONG
        .iter()
        .map(|name| naive_arc_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_rc_long_queries: Vec<Rc<str>> = MAP_NAMES_LONG
        .iter()
        .map(|name| naive_rc_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_compact_long_queries: Vec<CompactString> = MAP_NAMES_LONG
        .iter()
        .map(|name| naive_compact_key(&name.to_ascii_lowercase()))
        .collect();
    let naive_uncased_long_queries: Vec<Uncased<'static>> = MAP_NAMES_LONG
        .iter()
        .map(|name| naive_uncased_key(&name.to_ascii_lowercase()))
        .collect();

    let mut group = c.benchmark_group("map_lookup_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                sum += identstr_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_arc_queries {
                sum += identstr_long_arc_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("key", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &key_long_queries {
                sum += key_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_box", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_box_long_queries {
                sum += naive_box_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_arc", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_arc_long_queries {
                sum += naive_arc_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_rc", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_rc_long_queries {
                sum += naive_rc_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_compact", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_compact_long_queries {
                sum += naive_compact_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("naive_uncased", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &naive_uncased_long_queries {
                sum += naive_uncased_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.finish();
}

fn bench_repeat_key(c: &mut Criterion) {
    let raw_short: Vec<IdentStr<Quote, policy::Ascii, BoxSpill>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| IdentStr::new(*name))
        .collect();
    let key_short: Vec<Key<policy::Ascii>> =
        MAP_NAMES_SHORT.iter().map(|name| Key::new(name)).collect();
    let raw_short_needle = IdentStr::<Quote, policy::Ascii, BoxSpill>::new("users");
    let key_short_needle = Key::<policy::Ascii>::new("users");

    let mut group = c.benchmark_group("repeat_match_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &raw_short {
                count += usize::from(black_box(value) == black_box(&raw_short_needle));
            }
            black_box(count);
        });
    });
    group.bench_function("key", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &key_short {
                count += usize::from(black_box(value) == black_box(&key_short_needle));
            }
            black_box(count);
        });
    });
    group.finish();

    let raw_long: Vec<IdentStr<Quote, policy::Ascii, BoxSpill>> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::new(*name))
        .collect();
    let key_long: Vec<Key<policy::Ascii>> =
        MAP_NAMES_LONG.iter().map(|name| Key::new(name)).collect();
    let raw_long_needle = IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line_alpha",
    );
    let key_long_needle =
        Key::<policy::Ascii>::new("this_identifier_name_is_long_enough_to_spill_out_of_line_alpha");

    let mut group = c.benchmark_group("repeat_match_long");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &raw_long {
                count += usize::from(black_box(value) == black_box(&raw_long_needle));
            }
            black_box(count);
        });
    });
    group.bench_function("key", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &key_long {
                count += usize::from(black_box(value) == black_box(&key_long_needle));
            }
            black_box(count);
        });
    });
    group.finish();
}

#[cfg(feature = "unicode")]
fn bench_unicode(c: &mut Criterion) {
    let lhs_nfc = IdentStr::<Quote, policy::UnicodeNfc>::new("É");
    let rhs_nfc = IdentStr::<Quote, policy::UnicodeNfc>::new("e\u{301}");
    let lhs_nfkc = IdentStr::<Quote, policy::UnicodeNfkc>::new("ﬀ");
    let rhs_nfkc = IdentStr::<Quote, policy::UnicodeNfkc>::new("FF");

    let mut group = c.benchmark_group("unicode_eq");
    group.bench_function("identstr_nfc", |b| {
        b.iter(|| {
            black_box(lhs_nfc == rhs_nfc);
        });
    });
    group.bench_function("identstr_nfkc", |b| {
        b.iter(|| {
            black_box(lhs_nfkc == rhs_nfkc);
        });
    });
    group.finish();

    let mut group = c.benchmark_group("unicode_hash");
    group.bench_function("identstr_nfc", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            black_box(&lhs_nfc).hash(&mut hasher);
            black_box(hasher.finish());
        });
    });
    group.bench_function("identstr_nfkc", |b| {
        b.iter(|| {
            let mut hasher = DefaultHasher::new();
            black_box(&lhs_nfkc).hash(&mut hasher);
            black_box(hasher.finish());
        });
    });
    group.finish();

    let mut group = c.benchmark_group("unicode_key");
    group.bench_function("key_nfc", |b| {
        b.iter(|| {
            black_box(Key::<policy::UnicodeNfc>::new("É"));
        });
    });
    group.bench_function("key_nfkc", |b| {
        b.iter(|| {
            black_box(Key::<policy::UnicodeNfkc>::new("ﬀ"));
        });
    });
    group.bench_function("identstr_nfc", |b| {
        b.iter(|| {
            black_box(lhs_nfc.to_key());
        });
    });
    group.bench_function("identstr_nfkc", |b| {
        b.iter(|| {
            black_box(lhs_nfkc.to_key());
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_construction,
    bench_owned_backend_input,
    bench_conversion,
    bench_clone,
    bench_drop,
    bench_maps,
    bench_repeat_key,
    bench_read,
    bench_render_quoted,
    bench_compare,
    bench_mixed_backend_eq,
    bench_hash
);
#[cfg(feature = "unicode")]
criterion_group!(unicode_benches, bench_unicode);

#[cfg(feature = "unicode")]
criterion_main!(benches, unicode_benches);
#[cfg(not(feature = "unicode"))]
criterion_main!(benches);
