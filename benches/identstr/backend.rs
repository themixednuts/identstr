use super::*;

pub(super) fn bench_owned_backend_input(c: &mut Criterion) {
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

pub(super) fn bench_mixed_backend_eq(c: &mut Criterion) {
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

pub(super) fn bench_drop(c: &mut Criterion) {
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
