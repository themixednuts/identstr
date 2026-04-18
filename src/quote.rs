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

    /// Returns the closing delimiter for this quote style.
    #[must_use]
    pub const fn close(self) -> char {
        match self {
            Self::Bracket => ']',
            _ => self.open(),
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
    fn encode(self) -> u8 {
        self.tag()
    }

    fn decode(tag: u8) -> Option<Self> {
        Self::from_tag(tag)
    }
}
