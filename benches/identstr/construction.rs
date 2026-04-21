use super::*;

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

    let source_double = "\"customer_id\"";
    let source_double_escaped = "\"customer\"\"id\"";
    let source_bracket = "[customer_id]";
    let source_bracket_escaped = "[customer]]id]";

    let mut group = c.benchmark_group("construct_borrowed_source");
    group.bench_function("new_double", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
                source_double,
            ));
        });
    });
    group.bench_function("with_quote_double", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                "customer_id",
                Quote::Double,
            ));
        });
    });
    group.bench_function("new_double_escaped", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
                source_double_escaped,
            ));
        });
    });
    group.bench_function("with_quote_double_escaped", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                "customer\"id",
                Quote::Double,
            ));
        });
    });
    group.bench_function("new_bracket", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
                source_bracket,
            ));
        });
    });
    group.bench_function("with_quote_bracket", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                "customer_id",
                Quote::Bracket,
            ));
        });
    });
    group.bench_function("new_bracket_escaped", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::new(
                source_bracket_escaped,
            ));
        });
    });
    group.bench_function("with_quote_bracket_escaped", |b| {
        b.iter(|| {
            black_box(IdentStr::<Quote, policy::Ascii, BoxSpill>::with_quote(
                "customer]id",
                Quote::Bracket,
            ));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("construct_key_source");
    group.bench_function("new_raw", |b| {
        b.iter(|| {
            black_box(Key::<policy::Ascii>::new("customer_id"));
        });
    });
    group.bench_function("from_raw", |b| {
        b.iter(|| {
            black_box(Key::<policy::Ascii>::from_raw("customer_id"));
        });
    });
    group.bench_function("new_double", |b| {
        b.iter(|| {
            black_box(Key::<policy::Ascii>::new(source_double));
        });
    });
    group.bench_function("new_double_escaped", |b| {
        b.iter(|| {
            black_box(Key::<policy::Ascii>::new(source_double_escaped));
        });
    });
    group.bench_function("new_bracket", |b| {
        b.iter(|| {
            black_box(Key::<policy::Ascii>::new(source_bracket));
        });
    });
    group.bench_function("new_bracket_escaped", |b| {
        b.iter(|| {
            black_box(Key::<policy::Ascii>::new(source_bracket_escaped));
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

pub(super) fn bench_construction(c: &mut Criterion) {
    let short = "customer_id";
    let long = "this_identifier_name_is_long_enough_to_spill_out_of_line";

    bench_borrowed_construction(c, short, long);
    bench_owned_construction(c, short, long);
}
