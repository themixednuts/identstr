use std::borrow::Cow;

use crate::QuoteStyle;

#[inline]
pub(crate) fn quoted_source<Q: QuoteStyle>(value: &str) -> Option<(Q, Cow<'_, str>)> {
    let (quote, inner) = Q::split_source(value)?;
    Some((quote, quoted_inner(inner, quote.close_byte())?))
}

#[inline]
fn quoted_inner(value: &str, escape: u8) -> Option<Cow<'_, str>> {
    let bytes = value.as_bytes();
    let Some(first_escape) = bytes.iter().position(|&byte| byte == escape) else {
        return Some(Cow::Borrowed(value));
    };

    if bytes.get(first_escape + 1) != Some(&escape) {
        return None;
    }

    Some(Cow::Owned(quoted_inner_slow(value, escape, first_escape)?))
}

#[cold]
fn quoted_inner_slow(value: &str, escape: u8, first_escape: usize) -> Option<String> {
    let bytes = value.as_bytes();
    let mut unescaped = String::with_capacity(value.len() - 1);
    let mut start = 0;
    let mut index = first_escape;

    loop {
        unescaped.push_str(&value[start..index]);
        unescaped.push(char::from(escape));
        start = index + 2;

        let Some(offset) = bytes[start..].iter().position(|&byte| byte == escape) else {
            break;
        };

        index = start + offset;
        if bytes.get(index + 1) != Some(&escape) {
            return None;
        }
    }

    unescaped.push_str(&value[start..]);
    Some(unescaped)
}
