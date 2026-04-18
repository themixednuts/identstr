//! Unicode security helpers for identifier text.

use std::{borrow::Borrow, fmt, ops::Deref};

use unicode_security::{
    GeneralSecurityProfile, MixedScript, RestrictionLevel, RestrictionLevelDetection,
    is_potential_mixed_script_confusable_char, skeleton as unicode_skeleton,
};

/// Cached confusable skeleton for identifier text.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Skeleton(Box<str>);

impl Skeleton {
    /// Builds a skeleton for the provided text.
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self(unicode_skeleton(value).collect::<String>().into_boxed_str())
    }

    /// Returns the cached skeleton text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Skeleton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl fmt::Display for Skeleton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Deref for Skeleton {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for Skeleton {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for Skeleton {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for Skeleton {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for Skeleton {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl From<Skeleton> for Box<str> {
    fn from(value: Skeleton) -> Self {
        value.0
    }
}

impl From<Skeleton> for String {
    fn from(value: Skeleton) -> Self {
        let value: Box<str> = value.into();
        value.into_string()
    }
}

/// Security-oriented summary for identifier text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Analysis {
    /// `true` when every character is allowed in identifiers by the Unicode
    /// General Security Profile.
    pub identifier_allowed: bool,
    /// `true` when the text resolves to a single script set.
    pub single_script: bool,
    /// Unicode restriction level for the text.
    pub restriction_level: RestrictionLevel,
    /// `true` when any character is flagged as potentially mixed-script confusable.
    pub potential_mixed_script_confusable: bool,
}

/// Builds a confusable skeleton for the provided text.
#[must_use]
pub fn skeleton(value: &str) -> Skeleton {
    Skeleton::new(value)
}

/// Returns whether two identifiers share the same confusable skeleton.
#[must_use]
pub fn is_confusable(lhs: &str, rhs: &str) -> bool {
    skeleton(lhs) == skeleton(rhs)
}

/// Analyzes Unicode security properties for the provided text.
#[must_use]
pub fn analyze(value: &str) -> Analysis {
    Analysis {
        identifier_allowed: value
            .chars()
            .all(GeneralSecurityProfile::identifier_allowed),
        single_script: value.is_single_script(),
        restriction_level: value.detect_restriction_level(),
        potential_mixed_script_confusable: value
            .chars()
            .any(is_potential_mixed_script_confusable_char),
    }
}
