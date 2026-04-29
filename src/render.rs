use std::fmt;

use crate::QuoteTag;

#[inline]
fn find_byte(value: &str, needle: u8) -> Option<usize> {
    let bytes = value.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] == needle {
            return Some(index);
        }

        index += 1;
    }

    None
}

#[inline]
fn find_byte_from(bytes: &[u8], needle: u8, mut index: usize) -> Option<usize> {
    while index < bytes.len() {
        if bytes[index] == needle {
            return Some(index);
        }

        index += 1;
    }

    None
}

#[inline]
fn count_byte_from(bytes: &[u8], needle: u8, mut index: usize) -> usize {
    let mut count = 0;

    while index < bytes.len() {
        count += usize::from(bytes[index] == needle);
        index += 1;
    }

    count
}

#[inline]
fn delimiter(byte: u8) -> char {
    debug_assert!(byte.is_ascii(), "quote delimiter must be ASCII");
    char::from(byte)
}

pub(crate) fn write_quoted<Q: QuoteTag>(
    value: &str,
    quote: Option<Q>,
    output: &mut (impl fmt::Write + ?Sized),
) -> fmt::Result {
    let Some(quote) = quote else {
        return output.write_str(value);
    };

    let escape = quote.close_byte();
    if let Some(first_escape) = find_byte(value, escape) {
        return write_escaped(value, quote, first_escape, output);
    }

    output.write_char(delimiter(quote.open_byte()))?;
    output.write_str(value)?;
    output.write_char(delimiter(quote.close_byte()))
}

pub(crate) fn to_string<Q: QuoteTag>(value: &str, quote: Option<Q>) -> String {
    let Some(quote) = quote else {
        return value.to_owned();
    };

    let escape = quote.close_byte();
    let bytes = value.as_bytes();
    let first_escape = find_byte(value, escape);
    let capacity = match first_escape {
        Some(first_escape) => value.len() + 3 + count_byte_from(bytes, escape, first_escape + 1),
        None => value.len() + 2,
    };

    let mut rendered = String::with_capacity(capacity);
    match first_escape {
        Some(first_escape) => push_escaped(value, quote, first_escape, &mut rendered),
        None => push_unescaped(value, quote, &mut rendered),
    }
    rendered
}

fn push_unescaped<Q: QuoteTag>(value: &str, quote: Q, output: &mut String) {
    output.push(delimiter(quote.open_byte()));
    output.push_str(value);
    output.push(delimiter(quote.close_byte()));
}

#[cold]
fn push_escaped<Q: QuoteTag>(value: &str, quote: Q, first_escape: usize, output: &mut String) {
    let close = delimiter(quote.close_byte());
    let escape = quote.close_byte();
    let bytes = value.as_bytes();
    let mut start = 0;
    let mut index = first_escape;

    output.push(delimiter(quote.open_byte()));

    loop {
        output.push_str(&value[start..index]);
        output.push(close);
        output.push(close);
        start = index + 1;

        let Some(next_escape) = find_byte_from(bytes, escape, start) else {
            break;
        };

        index = next_escape;
    }

    output.push_str(&value[start..]);
    output.push(close);
}

#[cold]
fn write_escaped<Q: QuoteTag>(
    value: &str,
    quote: Q,
    first_escape: usize,
    output: &mut (impl fmt::Write + ?Sized),
) -> fmt::Result {
    let close = delimiter(quote.close_byte());
    let escape = quote.close_byte();
    let bytes = value.as_bytes();
    let mut start = 0;
    let mut index = first_escape;

    output.write_char(delimiter(quote.open_byte()))?;

    loop {
        output.write_str(&value[start..index])?;
        output.write_char(close)?;
        output.write_char(close)?;
        start = index + 1;

        let Some(next_escape) = find_byte_from(bytes, escape, start) else {
            break;
        };

        index = next_escape;
    }

    output.write_str(&value[start..])?;
    output.write_char(close)
}
