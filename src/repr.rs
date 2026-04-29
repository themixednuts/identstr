use std::ptr::NonNull;

pub(crate) const INLINE_CAPACITY: usize = 2 * std::mem::size_of::<usize>();
pub(crate) const QUOTED_INLINE_CAPACITY: usize = INLINE_CAPACITY - 1;

const INLINE_META_INDEX: usize = INLINE_CAPACITY - 1;
const QUOTED_LEN_INDEX: usize = QUOTED_INLINE_CAPACITY - 1;
const HEAP_LEN_MASK: usize = usize::MAX >> 8;
#[cfg(target_endian = "little")]
const HEAP_MARKER_SHIFT: u32 = usize::BITS - 8;
const SHORT_INLINE_BASE: u8 = 0xC0;
const QUOTED_INLINE_BASE: u8 = 0xD0;
const HEAP_BASE: u8 = 0xE0;
const HEAP_QUOTE_MASK: u8 = 0x1F;

const _: () = {
    assert!(INLINE_CAPACITY <= 16);
};

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Repr {
    storage: Storage,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct HeapRepr {
    ptr: NonNull<u8>,
    meta: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InlineRepr {
    bytes: [u8; INLINE_CAPACITY],
}

#[repr(C)]
#[derive(Clone, Copy)]
union Storage {
    heap: HeapRepr,
    inline: InlineRepr,
}

impl Repr {
    pub(crate) const MAX_INLINE_QUOTE_TAG: u8 = 0x0F;
    pub(crate) const MAX_HEAP_QUOTE_TAG: u8 = HEAP_QUOTE_MASK;

    pub(crate) const fn new_inline(bytes: [u8; INLINE_CAPACITY]) -> Self {
        Self {
            storage: Storage {
                inline: InlineRepr { bytes },
            },
        }
    }

    pub(crate) fn new_heap(ptr: NonNull<u8>, len: usize, quote_tag: u8) -> Self {
        assert!(quote_tag <= Self::MAX_HEAP_QUOTE_TAG);
        assert!(
            len <= HEAP_LEN_MASK,
            "identifier too long for packed heap representation",
        );
        let meta = Self::heap_meta(len, HEAP_BASE | quote_tag);

        Self {
            storage: Storage {
                heap: HeapRepr { ptr, meta },
            },
        }
    }

    pub(crate) const fn is_inline(self) -> bool {
        self.last_byte() < HEAP_BASE
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &[u8] {
        let last = self.last_byte();
        if last >= HEAP_BASE {
            return self.as_heap_bytes();
        }

        let len = if last > QUOTED_INLINE_BASE {
            let marker = unsafe { self.storage.inline.bytes[QUOTED_LEN_INDEX] };
            if (SHORT_INLINE_BASE..QUOTED_INLINE_BASE).contains(&marker) {
                usize::from(marker - SHORT_INLINE_BASE)
            } else {
                QUOTED_INLINE_CAPACITY
            }
        } else if (SHORT_INLINE_BASE..QUOTED_INLINE_BASE).contains(&last) {
            usize::from(last - SHORT_INLINE_BASE)
        } else {
            INLINE_CAPACITY
        };

        unsafe { std::slice::from_raw_parts(self.storage.inline.bytes.as_ptr(), len) }
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub(crate) fn quote_tag(self) -> u8 {
        let last = self.last_byte();
        if last < HEAP_BASE {
            Self::inline_tag(last)
        } else {
            self.heap_tag()
        }
    }

    pub(crate) fn heap_tag(self) -> u8 {
        self.last_byte() & HEAP_QUOTE_MASK
    }

    pub(crate) const fn last_byte(self) -> u8 {
        unsafe { self.storage.inline.bytes[INLINE_META_INDEX] }
    }

    #[inline]
    pub(crate) fn heap_ptr(self) -> NonNull<u8> {
        unsafe { self.storage.heap.ptr }
    }

    #[inline]
    pub(crate) fn heap_len(self) -> usize {
        Self::len_from_heap_meta(self.heap_meta_value())
    }

    #[inline]
    pub(crate) fn heap_raw_str(self) -> *const str {
        let slice =
            std::ptr::slice_from_raw_parts(self.heap_ptr().as_ptr().cast_const(), self.heap_len());
        slice as *const str
    }

    pub(crate) const fn short_inline_byte(len: u8) -> u8 {
        SHORT_INLINE_BASE | len
    }

    pub(crate) const fn quoted_inline_byte(tag: u8) -> u8 {
        QUOTED_INLINE_BASE | tag
    }

    pub(crate) const fn inline_tag(byte: u8) -> u8 {
        if byte <= QUOTED_INLINE_BASE || byte >= HEAP_BASE {
            return 0;
        }
        byte - QUOTED_INLINE_BASE
    }

    #[inline]
    fn as_heap_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.heap_ptr().as_ptr(), self.heap_len()) }
    }

    const fn heap_meta(len: usize, marker: u8) -> usize {
        #[cfg(target_endian = "little")]
        {
            len | (marker as usize) << HEAP_MARKER_SHIFT
        }

        #[cfg(target_endian = "big")]
        {
            (len << 8) | marker as usize
        }
    }

    const fn len_from_heap_meta(meta: usize) -> usize {
        #[cfg(target_endian = "little")]
        {
            meta & HEAP_LEN_MASK
        }

        #[cfg(target_endian = "big")]
        {
            meta >> 8
        }
    }

    fn heap_meta_value(self) -> usize {
        unsafe { self.storage.heap.meta }
    }
}

// SAFETY: `Repr` only stores immutable string bytes plus packed metadata. The
// owning `IdentStr` carries the storage owner type in `PhantomData`, so Send
// and Sync are controlled by `S::Owned`, `Q`, `P`, and `S`.
unsafe impl Send for Repr {}
// SAFETY: Shared access to `Repr` can only produce shared `str` references.
// Storage-specific mutation such as refcount updates occurs through `Storage`
// operations on owned `IdentStr` values.
unsafe impl Sync for Repr {}
