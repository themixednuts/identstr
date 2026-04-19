use crate::QuoteTag;

/// Common SQL-style identifier quote delimiters.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quote {
    Double = 1,
    Single = 2,
    Backtick = 3,
    Bracket = 4,
}

impl Quote {
    /// Returns the stored tag for this quote style.
    #[must_use]
    pub const fn tag(self) -> u8 {
        self as u8
    }

    /// Converts a stored quote tag into a [`Quote`] value.
    #[must_use]
    pub const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
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
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_tag(value).ok_or(())
    }
}

impl TryFrom<char> for Quote {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Self::from_open(value).ok_or(())
    }
}

impl QuoteTag for Quote {
    #[inline]
    fn encode(self) -> u8 {
        self.tag()
    }

    #[inline]
    fn decode(tag: u8) -> Option<Self> {
        Self::from_tag(tag)
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
