use super::*;

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

pub(super) fn bench_read(c: &mut Criterion) {
    bench_read_text(c);
    bench_read_quoted_parts(c);
    bench_read_key(c);
}

pub(super) fn bench_render_quoted(c: &mut Criterion) {
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
