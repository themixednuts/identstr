use std::{borrow::Cow, rc::Rc, sync::Arc};

use crate::{ArcStorage, BoxStorage, IdentStr, Policy, QuoteTag, RcStorage, Storage};

mod sealed {
    pub trait Sealed {}
}

/// Accepted input for [`IdentStr`] constructors.
///
/// Implemented for borrowed text, [`String`], [`Cow`], and the owned string
/// type for each built-in storage mode. Constructors use this trait to reuse
/// owned input when possible.
pub trait Input<S: Storage>: sealed::Sealed {
    #[doc(hidden)]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S>;

    #[doc(hidden)]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, S>;
}

impl<T: ?Sized + AsRef<str>> sealed::Sealed for &T {}

impl<S: Storage, T: ?Sized + AsRef<str>> Input<S> for &T {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S> {
        IdentStr::from_borrowed(self.as_ref(), quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, S> {
        IdentStr::from_source_borrowed(self.as_ref())
    }
}

impl sealed::Sealed for String {}

impl<S: Storage> Input<S> for String {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S> {
        IdentStr::from_string(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, S> {
        IdentStr::from_source_string(self)
    }
}

impl sealed::Sealed for Cow<'_, str> {}

impl<S: Storage> Input<S> for Cow<'_, str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S> {
        match self {
            Cow::Borrowed(value) => IdentStr::from_borrowed(value, quote),
            Cow::Owned(value) => IdentStr::from_string(value, quote),
        }
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, S> {
        match self {
            Cow::Borrowed(value) => IdentStr::from_source_borrowed(value),
            Cow::Owned(value) => IdentStr::from_source_string(value),
        }
    }
}

impl sealed::Sealed for Box<str> {}

impl Input<BoxStorage> for Box<str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, BoxStorage> {
        IdentStr::from_owned(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, BoxStorage> {
        IdentStr::from_source_owned(self)
    }
}

impl sealed::Sealed for Arc<str> {}

impl Input<ArcStorage> for Arc<str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, ArcStorage> {
        IdentStr::from_owned(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, ArcStorage> {
        IdentStr::from_source_owned(self)
    }
}

impl sealed::Sealed for Rc<str> {}

impl Input<RcStorage> for Rc<str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, RcStorage> {
        IdentStr::from_owned(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, RcStorage> {
        IdentStr::from_source_owned(self)
    }
}
