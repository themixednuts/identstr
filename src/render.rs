use std::fmt;

use crate::Quote;

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

pub(crate) fn write_quoted(
    value: &str,
    quote: Option<Quote>,
    output: &mut (impl fmt::Write + ?Sized),
) -> fmt::Result {
    let Some(quote) = quote else {
        return output.write_str(value);
    };

    let escape = quote.close_byte();
    if let Some(first_escape) = find_byte(value, escape) {
        return write_escaped(value, quote, first_escape, output);
    }

    output.write_char(quote.open())?;
    output.write_str(value)?;
    output.write_char(quote.close())
}

pub(crate) fn to_string(value: &str, quote: Option<Quote>) -> String {
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

fn push_unescaped(value: &str, quote: Quote, output: &mut String) {
    output.push(quote.open());
    output.push_str(value);
    output.push(quote.close());
}

#[cold]
fn push_escaped(value: &str, quote: Quote, first_escape: usize, output: &mut String) {
    let close = quote.close();
    let escape = quote.close_byte();
    let bytes = value.as_bytes();
    let mut start = 0;
    let mut index = first_escape;

    output.push(quote.open());

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
fn write_escaped(
    value: &str,
    quote: Quote,
    first_escape: usize,
    output: &mut (impl fmt::Write + ?Sized),
) -> fmt::Result {
    let close = quote.close();
    let escape = quote.close_byte();
    let bytes = value.as_bytes();
    let mut start = 0;
    let mut index = first_escape;

    output.write_char(quote.open())?;

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
