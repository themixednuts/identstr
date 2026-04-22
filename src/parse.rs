use std::borrow::Cow;

use crate::QuoteTag;

#[inline]
pub(crate) fn quoted_source<Q: QuoteTag>(value: &str) -> Option<(Q, Cow<'_, str>)> {
    let (quote, inner) = Q::split_source(value)?;
    Some((quote, quoted_inner(inner, quote.close_byte())?))
}

#[inline]
fn quoted_inner(value: &str, escape: u8) -> Option<Cow<'_, str>> {
    let bytes = value.as_bytes();
    if bytes.len() < 2 {
        return Some(Cow::Borrowed(value));
    }

    let mut index = 0;

    while index + 1 < bytes.len() {
        if bytes[index] == escape {
            if bytes[index + 1] != escape {
                return None;
            }

            return Some(Cow::Owned(quoted_inner_slow(value, escape, index)?));
        }

        index += 1;
    }

    if bytes[index] == escape {
        return None;
    }

    Some(Cow::Borrowed(value))
}

#[cold]
fn quoted_inner_slow(value: &str, escape: u8, first_escape: usize) -> Option<String> {
    let bytes = value.as_bytes();
    let mut unescaped = String::with_capacity(value.len() - 1);
    unescaped.push_str(&value[..first_escape]);
    unescaped.push(escape as char);

    let mut index = first_escape + 2;
    let mut start = index;
    while index + 1 < bytes.len() {
        if bytes[index] != escape {
            index += 1;
            continue;
        }

        if bytes[index + 1] != escape {
            return None;
        }

        unescaped.push_str(&value[start..index]);
        unescaped.push(escape as char);
        index += 2;
        start = index;
    }

    if index < bytes.len() && bytes[index] == escape {
        return None;
    }

    unescaped.push_str(&value[start..]);
    Some(unescaped)
}
