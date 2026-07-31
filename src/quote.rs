use std::{error::Error, fmt, num::NonZeroU8};

use crate::QuoteStyle;

/// Common identifier quote delimiters.
///
/// ```rust
/// use identstr::Quote;
///
/// assert_eq!(Quote::from_open('['), Some(Quote::Bracket));
/// assert_eq!(Quote::Bracket.open(), '[');
/// assert_eq!(Quote::Bracket.close(), ']');
/// ```
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quote {
    Double = 1,
    Single = 2,
    Backtick = 3,
    Bracket = 4,
}

/// Error returned when a delimiter or tag code is not a recognized
/// [`Quote`].
///
/// ```rust
/// use identstr::{IdentStr, InvalidQuote, Quote};
///
/// let invalid: Result<IdentStr, InvalidQuote> = IdentStr::try_with_quote("Users", '!');
///
/// assert!(invalid.is_err());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidQuote(pub(crate) ());

impl fmt::Display for InvalidQuote {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid quote delimiter")
    }
}

impl Error for InvalidQuote {}

impl Quote {
    /// Returns the code used to preserve this quote style.
    #[must_use]
    pub const fn tag(self) -> NonZeroU8 {
        match NonZeroU8::new(self as u8) {
            Some(tag) => tag,
            None => unreachable!(),
        }
    }

    /// Converts a preserved quote code into a [`Quote`] value.
    #[must_use]
    pub const fn from_tag(tag: NonZeroU8) -> Option<Self> {
        match tag.get() {
            1 => Some(Self::Double),
            2 => Some(Self::Single),
            3 => Some(Self::Backtick),
            4 => Some(Self::Bracket),
            _ => None,
        }
    }

    /// Converts an opening quote delimiter into a [`Quote`] value.
    #[must_use]
    pub const fn from_open(quote: char) -> Option<Self> {
        match quote {
            '"' => Some(Self::Double),
            '\'' => Some(Self::Single),
            '`' => Some(Self::Backtick),
            '[' => Some(Self::Bracket),
            _ => None,
        }
    }

    /// Converts an opening quote delimiter byte into a [`Quote`] value.
    #[must_use]
    pub const fn from_open_byte(quote: u8) -> Option<Self> {
        match quote {
            b'"' => Some(Self::Double),
            b'\'' => Some(Self::Single),
            b'`' => Some(Self::Backtick),
            b'[' => Some(Self::Bracket),
            _ => None,
        }
    }

    /// Returns the opening delimiter for this quote style.
    #[must_use]
    pub const fn open(self) -> char {
        match self {
            Self::Double => '"',
            Self::Single => '\'',
            Self::Backtick => '`',
            Self::Bracket => '[',
        }
    }

    /// Returns the opening delimiter byte for this quote style.
    #[must_use]
    pub const fn open_byte(self) -> u8 {
        match self {
            Self::Double => b'"',
            Self::Single => b'\'',
            Self::Backtick => b'`',
            Self::Bracket => b'[',
        }
    }

    /// Returns the closing delimiter for this quote style.
    #[must_use]
    pub const fn close(self) -> char {
        match self {
            Self::Bracket => ']',
            _ => self.open(),
        }
    }

    /// Returns the closing delimiter byte for this quote style.
    #[must_use]
    pub const fn close_byte(self) -> u8 {
        match self {
            Self::Bracket => b']',
            _ => self.open_byte(),
        }
    }
}

impl TryFrom<u8> for Quote {
    type Error = InvalidQuote;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        NonZeroU8::new(value)
            .and_then(Self::from_tag)
            .ok_or(InvalidQuote(()))
    }
}

impl TryFrom<char> for Quote {
    type Error = InvalidQuote;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_open(value).ok_or(InvalidQuote(()))
    }
}

impl QuoteStyle for Quote {
    #[inline]
    fn tag(self) -> NonZeroU8 {
        self.tag()
    }

    #[inline]
    fn from_tag(tag: NonZeroU8) -> Option<Self> {
        Self::from_tag(tag)
    }

    #[inline]
    fn open_byte(self) -> u8 {
        self.open_byte()
    }

    #[inline]
    fn close_byte(self) -> u8 {
        self.close_byte()
    }

    #[inline]
    fn split_source(value: &str) -> Option<(Self, &str)> {
        let bytes = value.as_bytes();
        if bytes.len() < 2 {
            return None;
        }

        let quote = match (bytes[0], bytes[bytes.len() - 1]) {
            (b'"', b'"') => Self::Double,
            (b'\'', b'\'') => Self::Single,
            (b'`', b'`') => Self::Backtick,
            (b'[', b']') => Self::Bracket,
            _ => return None,
        };

        Some((quote, &value[1..value.len() - 1]))
    }
}
