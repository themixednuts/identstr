use super::*;

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

pub(super) fn bench_compare(c: &mut Criterion) {
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

pub(super) fn bench_hash(c: &mut Criterion) {
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
