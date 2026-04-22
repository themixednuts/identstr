use std::{borrow::Cow, rc::Rc, sync::Arc};

use crate::{ArcSpill, BoxSpill, IdentStr, Policy, QuoteTag, RcSpill, Spill};

mod sealed {
    pub trait Sealed {}
}

#[doc(hidden)]
pub trait Input<S: Spill>: sealed::Sealed {
    #[doc(hidden)]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, S>;

    #[doc(hidden)]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, S>;
}

impl<T: ?Sized + AsRef<str>> sealed::Sealed for &T {}

impl<S: Spill, T: ?Sized + AsRef<str>> Input<S> for &T {
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

impl<S: Spill> Input<S> for String {
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

impl<S: Spill> Input<S> for Cow<'_, str> {
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

impl Input<BoxSpill> for Box<str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, BoxSpill> {
        IdentStr::from_owned(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, BoxSpill> {
        IdentStr::from_source_owned(self)
    }
}

impl sealed::Sealed for Arc<str> {}

impl Input<ArcSpill> for Arc<str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, ArcSpill> {
        IdentStr::from_owned(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, ArcSpill> {
        IdentStr::from_source_owned(self)
    }
}

impl sealed::Sealed for Rc<str> {}

impl Input<RcSpill> for Rc<str> {
    #[inline]
    fn into_raw<Q: QuoteTag, P: Policy>(self, quote: Option<Q>) -> IdentStr<Q, P, RcSpill> {
        IdentStr::from_owned(self, quote)
    }

    #[inline]
    fn into_source<Q: QuoteTag, P: Policy>(self) -> IdentStr<Q, P, RcSpill> {
        IdentStr::from_source_owned(self)
    }
}
