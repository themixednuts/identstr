//! Serde support for identifier types.
//!
//! [`IdentStr`] serializes as its quoted source form so quote style
//! round-trips through [`IdentStr::new`]. [`Key`] serializes as its
//! normalized lookup text and deserializes without quote parsing.

use std::{fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error, Visitor},
};

use crate::{IdentStr, Key, Policy, QuoteStyle, Storage, policy::KeyPolicy};

impl<Q: QuoteStyle, P: Policy, S: Storage> Serialize for IdentStr<Q, P, S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        serializer.collect_str(&self.display_quoted())
    }
}

struct IdentStrVisitor<Q, P, S>(PhantomData<(Q, P, S)>);

impl<Q: QuoteStyle, P: Policy, S: Storage> Visitor<'_> for IdentStrVisitor<Q, P, S> {
    type Value = IdentStr<Q, P, S>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("identifier source text")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(IdentStr::new(value))
    }

    fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(IdentStr::new(value))
    }
}

impl<'de, Q: QuoteStyle, P: Policy, S: Storage> Deserialize<'de> for IdentStr<Q, P, S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(IdentStrVisitor(PhantomData))
    }
}

impl<P: KeyPolicy> Serialize for Key<P> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

struct KeyVisitor<P>(PhantomData<P>);

impl<P: KeyPolicy> Visitor<'_> for KeyVisitor<P> {
    type Value = Key<P>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("identifier lookup text")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Key::from_unquoted(value))
    }

    fn visit_string<E: Error>(self, value: String) -> Result<Self::Value, E> {
        Ok(Key::from_unquoted_string(value))
    }
}

impl<'de, P: KeyPolicy> Deserialize<'de> for Key<P> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(KeyVisitor(PhantomData))
    }
}
