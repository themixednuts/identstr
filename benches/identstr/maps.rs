use super::*;

pub(super) fn bench_maps(c: &mut Criterion) {
    let mut group = c.benchmark_group("map_insert_short");
    group.bench_function("identstr_box", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(
                    IdentStr::<Quote, policy::Ascii, BoxStorage>::new(*name),
                    index,
                );
            }
            black_box(map);
        });
    });
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            let mut map = HashMap::with_capacity(MAP_NAMES_SHORT.len());
            for (index, name) in MAP_NAMES_SHORT.iter().enumerate() {
                map.insert(
                    IdentStr::<Quote, policy::Ascii, ArcStorage>::new(*name),
                    index,
                );
            }
            black_box(map);
        });
    });
    group.bench_function("Key", |b| {
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
                    IdentStr::<Quote, policy::Ascii, BoxStorage>::new(*name),
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
                    IdentStr::<Quote, policy::Ascii, ArcStorage>::new(*name),
                    index,
                );
            }
            black_box(map);
        });
    });
    group.bench_function("Key", |b| {
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

    let identstr_short_map: HashMap<AsciiIdent, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (IdentStr::new(*name), index))
        .collect();
    let identstr_short_arc_map: HashMap<AsciiArcIdent, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| {
            (
                IdentStr::<Quote, policy::Ascii, ArcStorage>::new(*name),
                index,
            )
        })
        .collect();
    let key_short_map: HashMap<AsciiKey, usize> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| (Key::new(name), index))
        .collect();
    let key_short_entry_map: HashMap<AsciiKey, AsciiEntry> = MAP_NAMES_SHORT
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let ident = IdentStr::new(*name);
            let lookup_key = ident.to_key();
            (lookup_key, (ident, index))
        })
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
    let string_short_map: HashMap<StringKey, usize> = MAP_NAMES_SHORT_LOWER
        .iter()
        .enumerate()
        .map(|(index, name)| ((*name).to_owned(), index))
        .collect();
    let str_short_map: HashMap<StaticKey, usize> = MAP_NAMES_SHORT_LOWER
        .iter()
        .enumerate()
        .map(|(index, name)| (*name, index))
        .collect();

    let identstr_short_queries: Vec<AsciiIdent> = MAP_NAMES_SHORT
        .iter()
        .map(|name| IdentStr::new(name.to_ascii_lowercase()))
        .collect();
    let identstr_short_arc_queries: Vec<AsciiArcIdent> = MAP_NAMES_SHORT
        .iter()
        .map(|name| IdentStr::<Quote, policy::Ascii, ArcStorage>::new(name.to_ascii_lowercase()))
        .collect();
    let key_short_queries: Vec<AsciiKey> = MAP_NAMES_SHORT
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
    let string_short_queries: Vec<StaticKey> = MAP_NAMES_SHORT_LOWER.to_vec();
    let str_short_queries: Vec<StaticKey> = MAP_NAMES_SHORT_LOWER.to_vec();

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
    group.bench_function("identstr_arc", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_arc_queries {
                sum += identstr_short_arc_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("Key", |b| {
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
    group.bench_function("string", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for &query in &string_short_queries {
                sum += string_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("str", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &str_short_queries {
                sum += str_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.finish();

    let mut group = c.benchmark_group("map_lookup_short_combined");
    group.bench_function("identstr_box_direct", |b| {
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
    group.bench_function("identstr_box_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                let (stored_ident, value) = identstr_short_map
                    .get_key_value(black_box(query))
                    .expect("short ident present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("identstr_arc_direct", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_arc_queries {
                sum += identstr_short_arc_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("identstr_arc_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_arc_queries {
                let (stored_ident, value) = identstr_short_arc_map
                    .get_key_value(black_box(query))
                    .expect("short arc ident present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_from_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                let lookup_key = black_box(query).to_key();
                sum += key_short_map
                    .get(black_box(&lookup_key))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_from_ident_entry", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                let lookup_key = black_box(query).to_key();
                let (stored_ident, value) = key_short_entry_map
                    .get(black_box(&lookup_key))
                    .expect("short entry present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_from_ident_to_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                let lookup_key = black_box(query).to_key();
                sum += key_short_map
                    .get(black_box(&lookup_key))
                    .copied()
                    .unwrap_or_default();
                black_box(IdentStr::<Quote, policy::Ascii, BoxStorage>::from_raw(
                    lookup_key.as_str(),
                ));
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_cached", |b| {
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
    group.bench_function("key_box_cached_entry", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &key_short_queries {
                let (stored_ident, value) = key_short_entry_map
                    .get(black_box(query))
                    .expect("short entry present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_cached_to_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &key_short_queries {
                sum += key_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
                black_box(IdentStr::<Quote, policy::Ascii, BoxStorage>::from_raw(
                    query.as_str(),
                ));
            }
            black_box(sum);
        });
    });
    group.bench_function("string_from_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                let lower = query.as_str().to_ascii_lowercase();
                sum += string_short_map
                    .get(black_box(lower.as_str()))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("string_cached", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for &query in &string_short_queries {
                sum += string_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("string_cached_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for &query in &string_short_queries {
                let (stored_key, value) = string_short_map
                    .get_key_value(black_box(query))
                    .expect("short string present");
                black_box(stored_key);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("str_from_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_short_queries {
                let lower = query.as_str().to_ascii_lowercase();
                sum += str_short_map
                    .get(black_box(lower.as_str()))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("str_cached", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &str_short_queries {
                sum += str_short_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("str_cached_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &str_short_queries {
                let (stored_key, value) = str_short_map
                    .get_key_value(black_box(query))
                    .expect("short str present");
                black_box(stored_key);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.finish();

    let identstr_long_map: HashMap<AsciiIdent, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (IdentStr::new(*name), index))
        .collect();
    let identstr_long_arc_map: HashMap<AsciiArcIdent, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| {
            (
                IdentStr::<Quote, policy::Ascii, ArcStorage>::new(*name),
                index,
            )
        })
        .collect();
    let key_long_map: HashMap<AsciiKey, usize> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| (Key::new(name), index))
        .collect();
    let key_long_entry_map: HashMap<AsciiKey, AsciiEntry> = MAP_NAMES_LONG
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let ident = IdentStr::new(*name);
            let lookup_key = ident.to_key();
            (lookup_key, (ident, index))
        })
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
    let string_long_map: HashMap<StringKey, usize> = MAP_NAMES_LONG_LOWER
        .iter()
        .enumerate()
        .map(|(index, name)| ((*name).to_owned(), index))
        .collect();
    let str_long_map: HashMap<StaticKey, usize> = MAP_NAMES_LONG_LOWER
        .iter()
        .enumerate()
        .map(|(index, name)| (*name, index))
        .collect();

    let identstr_long_queries: Vec<AsciiIdent> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::new(name.to_ascii_lowercase()))
        .collect();
    let identstr_long_arc_queries: Vec<AsciiArcIdent> = MAP_NAMES_LONG
        .iter()
        .map(|name| IdentStr::<Quote, policy::Ascii, ArcStorage>::new(name.to_ascii_lowercase()))
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
    let string_long_queries: Vec<StaticKey> = MAP_NAMES_LONG_LOWER.to_vec();
    let str_long_queries: Vec<StaticKey> = MAP_NAMES_LONG_LOWER.to_vec();

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
    group.bench_function("Key", |b| {
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
    group.bench_function("string", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for &query in &string_long_queries {
                sum += string_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("str", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &str_long_queries {
                sum += str_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.finish();

    let mut group = c.benchmark_group("map_lookup_long_combined");
    group.bench_function("identstr_box_direct", |b| {
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
    group.bench_function("identstr_box_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                let (stored_ident, value) = identstr_long_map
                    .get_key_value(black_box(query))
                    .expect("long ident present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("identstr_arc_direct", |b| {
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
    group.bench_function("identstr_arc_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_arc_queries {
                let (stored_ident, value) = identstr_long_arc_map
                    .get_key_value(black_box(query))
                    .expect("long arc ident present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_from_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                let lookup_key = black_box(query).to_key();
                sum += key_long_map
                    .get(black_box(&lookup_key))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_from_ident_entry", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                let lookup_key = black_box(query).to_key();
                let (stored_ident, value) = key_long_entry_map
                    .get(black_box(&lookup_key))
                    .expect("long entry present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_from_ident_to_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                let lookup_key = black_box(query).to_key();
                sum += key_long_map
                    .get(black_box(&lookup_key))
                    .copied()
                    .unwrap_or_default();
                black_box(IdentStr::<Quote, policy::Ascii, BoxStorage>::from_raw(
                    lookup_key.as_str(),
                ));
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_cached", |b| {
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
    group.bench_function("key_box_cached_entry", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &key_long_queries {
                let (stored_ident, value) = key_long_entry_map
                    .get(black_box(query))
                    .expect("long entry present");
                black_box(stored_ident);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("key_box_cached_to_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &key_long_queries {
                sum += key_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
                black_box(IdentStr::<Quote, policy::Ascii, BoxStorage>::from_raw(
                    query.as_str(),
                ));
            }
            black_box(sum);
        });
    });
    group.bench_function("string_from_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                let lower = query.as_str().to_ascii_lowercase();
                sum += string_long_map
                    .get(black_box(lower.as_str()))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("string_cached", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for &query in &string_long_queries {
                sum += string_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("string_cached_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for &query in &string_long_queries {
                let (stored_key, value) = string_long_map
                    .get_key_value(black_box(query))
                    .expect("long string present");
                black_box(stored_key);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.bench_function("str_from_ident", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &identstr_long_queries {
                let lower = query.as_str().to_ascii_lowercase();
                sum += str_long_map
                    .get(black_box(lower.as_str()))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("str_cached", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &str_long_queries {
                sum += str_long_map
                    .get(black_box(query))
                    .copied()
                    .unwrap_or_default();
            }
            black_box(sum);
        });
    });
    group.bench_function("str_cached_get_key_value", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for query in &str_long_queries {
                let (stored_key, value) = str_long_map
                    .get_key_value(black_box(query))
                    .expect("long str present");
                black_box(stored_key);
                sum += *value;
            }
            black_box(sum);
        });
    });
    group.finish();
}
