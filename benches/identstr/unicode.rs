use super::*;

#[cfg(feature = "unicode")]
pub(super) fn bench_unicode(c: &mut Criterion) {
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
