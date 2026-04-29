use super::*;

pub(super) fn bench_clone(c: &mut Criterion) {
    let short_ident = IdentStr::<Quote, policy::Ascii, BoxStorage>::new("customer_id");
    let short_shared = IdentStr::<Quote, policy::Ascii, ArcStorage>::new("customer_id");
    let short_box = NaiveBoxIdent::new("customer_id", Some(Quote::Double));
    let short_arc = NaiveArcIdent::new("customer_id", Some(Quote::Double));
    let short_rc = NaiveRcIdent::new("customer_id", Some(Quote::Double));
    let short_compact = NaiveCompactIdent::new("customer_id", Some(Quote::Double));
    let short_uncased = NaiveUncasedIdent::new("customer_id", Some(Quote::Double));

    let long_ident = IdentStr::<Quote, policy::Ascii, BoxStorage>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line",
    );
    let long_shared = IdentStr::<Quote, policy::Ascii, ArcStorage>::new(
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
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            black_box(short_shared.clone());
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

pub(super) fn bench_conversion(c: &mut Criterion) {
    let short = "customer_id";
    let long = "this_identifier_name_is_long_enough_to_spill_out_of_line";

    let mut group = c.benchmark_group("into_boxed_str");
    group.bench_function("identstr_inline_box", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, BoxStorage>::new(short);
            black_box(Box::<str>::from(ident));
        });
    });
    group.bench_function("identstr_spill_box", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, BoxStorage>::new(long);
            black_box(Box::<str>::from(ident));
        });
    });
    group.finish();

    let mut group = c.benchmark_group("into_arc_str");
    group.bench_function("identstr_inline_arc", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, ArcStorage>::new(short);
            black_box(Arc::<str>::from(ident));
        });
    });
    group.bench_function("identstr_spill_arc", |b| {
        b.iter(|| {
            let ident = IdentStr::<Quote, policy::Ascii, ArcStorage>::new(long);
            black_box(Arc::<str>::from(ident));
        });
    });
    group.finish();
}
