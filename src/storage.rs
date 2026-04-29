use std::{ptr::NonNull, rc::Rc, sync::Arc};

use crate::repr::Repr;

mod sealed {
    pub trait Sealed {}
}

/// Selects how allocated identifier text is owned and cloned.
///
/// Choose [`BoxStorage`], [`ArcStorage`], or [`RcStorage`] based on how you
/// want larger values to be shared. This trait is only implemented by the
/// built-in storage modes.
pub trait Storage: sealed::Sealed + Copy + 'static {
    /// Owned string type used by this storage mode.
    type Owned: AsRef<str> + 'static;

    #[doc(hidden)]
    fn from_borrowed(value: &str) -> Self::Owned;

    #[doc(hidden)]
    fn from_box(value: Box<str>) -> Self::Owned;

    #[doc(hidden)]
    fn into_repr(value: Self::Owned, quote_tag: u8) -> Repr;
    #[doc(hidden)]
    fn clone_repr(repr: &Repr) -> Repr;
    #[doc(hidden)]
    fn into_owned(repr: Repr) -> Self::Owned;

    #[doc(hidden)]
    fn drop_repr(repr: Repr) {
        drop(Self::into_owned(repr));
    }
}

/// String storage backed by `Box<str>`.
#[derive(Clone, Copy, Debug, Default)]
pub struct BoxStorage;

impl sealed::Sealed for BoxStorage {}

impl Storage for BoxStorage {
    type Owned = Box<str>;

    #[inline]
    fn from_borrowed(value: &str) -> Self::Owned {
        Box::<str>::from(value)
    }

    #[inline]
    fn from_box(value: Box<str>) -> Self::Owned {
        value
    }

    #[inline]
    fn into_repr(value: Self::Owned, quote_tag: u8) -> Repr {
        let len = value.len();
        let ptr = NonNull::new(Box::into_raw(value).cast::<u8>())
            .expect("boxed str pointer is never null");
        Repr::new_heap(ptr, len, quote_tag)
    }

    #[inline]
    fn clone_repr(repr: &Repr) -> Repr {
        Self::into_repr(Box::<str>::from(repr.as_str()), repr.heap_tag())
    }

    #[inline]
    fn into_owned(repr: Repr) -> Self::Owned {
        let raw = std::ptr::slice_from_raw_parts_mut(repr.heap_ptr().as_ptr(), repr.heap_len())
            as *mut str;
        unsafe { Box::from_raw(raw) }
    }
}

/// String storage backed by `Arc<str>`.
#[derive(Clone, Copy, Debug, Default)]
pub struct ArcStorage;

impl sealed::Sealed for ArcStorage {}

impl Storage for ArcStorage {
    type Owned = Arc<str>;

    #[inline]
    fn from_borrowed(value: &str) -> Self::Owned {
        Arc::<str>::from(value)
    }

    #[inline]
    fn from_box(value: Box<str>) -> Self::Owned {
        Arc::<str>::from(value)
    }

    #[inline]
    fn into_repr(value: Self::Owned, quote_tag: u8) -> Repr {
        let len = value.len();
        let ptr = NonNull::new(Arc::into_raw(value).cast_mut().cast::<u8>())
            .expect("arc str pointer is never null");
        Repr::new_heap(ptr, len, quote_tag)
    }

    #[inline]
    fn clone_repr(repr: &Repr) -> Repr {
        let ptr = repr.heap_raw_str();
        unsafe { Arc::<str>::increment_strong_count(ptr) };
        *repr
    }

    #[inline]
    fn into_owned(repr: Repr) -> Self::Owned {
        unsafe { Arc::from_raw(repr.heap_raw_str()) }
    }

    #[inline]
    fn drop_repr(repr: Repr) {
        unsafe { Arc::<str>::decrement_strong_count(repr.heap_raw_str()) };
    }
}

/// String storage backed by `Rc<str>`.
#[derive(Clone, Copy, Debug, Default)]
pub struct RcStorage;

impl sealed::Sealed for RcStorage {}

impl Storage for RcStorage {
    type Owned = Rc<str>;

    #[inline]
    fn from_borrowed(value: &str) -> Self::Owned {
        Rc::<str>::from(value)
    }

    #[inline]
    fn from_box(value: Box<str>) -> Self::Owned {
        Rc::<str>::from(value)
    }

    #[inline]
    fn into_repr(value: Self::Owned, quote_tag: u8) -> Repr {
        let len = value.len();
        let ptr = NonNull::new(Rc::into_raw(value).cast_mut().cast::<u8>())
            .expect("rc str pointer is never null");
        Repr::new_heap(ptr, len, quote_tag)
    }

    #[inline]
    fn clone_repr(repr: &Repr) -> Repr {
        let ptr = repr.heap_raw_str();
        unsafe { Rc::<str>::increment_strong_count(ptr) };
        *repr
    }

    #[inline]
    fn into_owned(repr: Repr) -> Self::Owned {
        unsafe { Rc::from_raw(repr.heap_raw_str()) }
    }

    #[inline]
    fn drop_repr(repr: Repr) {
        unsafe { Rc::<str>::decrement_strong_count(repr.heap_raw_str()) };
    }
}
