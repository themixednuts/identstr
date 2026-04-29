use super::*;

pub(super) fn bench_repeat_key(c: &mut Criterion) {
    let raw_short: Vec<IdentStr<Quote, policy::Ascii, BoxStorage>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| IdentStr::new(*name))
        .collect();
    let raw_short_arc: Vec<IdentStr<Quote, policy::Ascii, ArcStorage>> = MAP_NAMES_SHORT
        .iter()
        .map(|name| IdentStr::new(*name))
        .collect();
    let key_short: Vec<Key<policy::Ascii>> =
        MAP_NAMES_SHORT.iter().map(|name| Key::new(name)).collect();
    let raw_short_needle = IdentStr::<Quote, policy::Ascii, BoxStorage>::new("users");
    let raw_short_arc_needle = IdentStr::<Quote, policy::Ascii, ArcStorage>::new("users");
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
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &raw_short_arc {
                count += usize::from(black_box(value) == black_box(&raw_short_arc_needle));
            }
            black_box(count);
        });
    });
    group.bench_function("Key", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &key_short {
                count += usize::from(black_box(value) == black_box(&key_short_needle));
            }
            black_box(count);
        });
    });
    group.finish();

    let raw_long: Vec<IdentStr<Quote, policy::Ascii, BoxStorage>> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::new(*name))
        .collect();
    let raw_long_arc: Vec<IdentStr<Quote, policy::Ascii, ArcStorage>> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::new(*name))
        .collect();
    let key_long: Vec<Key<policy::Ascii>> =
        MAP_NAMES_LONG.iter().map(|name| Key::new(name)).collect();
    let raw_long_needle = IdentStr::<Quote, policy::Ascii, BoxStorage>::new(
        "this_identifier_name_is_long_enough_to_spill_out_of_line_alpha",
    );
    let raw_long_arc_needle = IdentStr::<Quote, policy::Ascii, ArcStorage>::new(
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
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            let mut count = 0usize;
            for value in &raw_long_arc {
                count += usize::from(black_box(value) == black_box(&raw_long_arc_needle));
            }
            black_box(count);
        });
    });
    group.bench_function("Key", |b| {
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
