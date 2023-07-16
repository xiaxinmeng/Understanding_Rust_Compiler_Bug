rust
#![feature(
    const_mut_refs,
    const_panic,
    const_ptr_is_null,
    const_raw_ptr_deref,
    const_slice_from_raw_parts,
)]

use core::char::DecodeUtf16Error;
use core::fmt::{self, Debug, Formatter};
use core::mem::{size_of, MaybeUninit};
use core::str::Utf8Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CapacityError<const N: usize>;

/// This enum represents several different possible "error states" that may be encountered
/// while using a [`StaticString`](crate::string::StaticString).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringError {
    /// Indicates a failed conversion from a `u8` slice to a
    /// [`StaticString`](crate::string::StaticString).
    Utf8(Utf8Error),
    /// Indicates a failed conversion from a `u16` slice to a
    /// [`StaticString`](crate::string::StaticString).
    Utf16(DecodeUtf16Error),
    /// Indicates an attempted access of an invalid UTF-8 character index.
    NotCharBoundary,
    /// Indicates an out-of-bounds indexed access of a [`StaticString`](crate::string::StaticString)
    /// instance.
    OutOfBounds,
}
impl<const N: usize> From<CapacityError<N>> for StringError {
    #[inline(always)]
    fn from(_err: CapacityError<N>) -> Self {
        Self::OutOfBounds
    }
}

/// A [`Vec`](alloc::vec::Vec)-like struct (mostly directly API-compatible where it can be)
/// implemented with const generics around an array of fixed `N` capacity.
struct StaticVec<T, const N: usize> {
    // We create this field in an uninitialized state, and write to it element-wise as needed
    // via pointer methods. At no time should `assume_init` *ever* be called through it.
    data: MaybeUninit<[T; N]>,
    // The constant `N` parameter (and thus the total span of `data`) represent capacity for us,
    // while the field below represents, as its name suggests, the current length of a StaticVec
    // (that is, the current number of "live" elements) just as is the case for a regular `Vec`.
    length: usize,
}

/// A local (identically written) `const fn` version of `slice::from_raw_parts`.
#[inline(always)]
pub(crate) const fn slice_from_raw_parts<'a, T>(data: *const T, length: usize) -> &'a [T] {
    debug_assert!(
        /*
        is_aligned_and_not_null(data),
        "Attempted to create an unaligned or null slice!"
        */
        // See comment starting at line 154 for more info about what's going on here.
        !data.is_null(),
        "Attempted to create a null slice!"
    );
    debug_assert!(
        size_of::<T>().saturating_mul(length) <= isize::MAX as usize,
        "Attempted to create a slice covering at least half of the address space!"
    );
    unsafe { &*core::ptr::slice_from_raw_parts(data, length) }
}

impl<T, const N: usize> StaticVec<T, N> {
    #[inline(always)]
    const fn new() -> Self {
        Self {
            data: Self::new_data_uninit(),
            length: 0,
        }
    }
    #[inline(always)]
    pub(crate) const fn new_data_uninit() -> MaybeUninit<[T; N]> {
        MaybeUninit::uninit()
    }
    #[inline(always)]
    const fn remaining_capacity(&self) -> usize {
        N - self.length
    }
    #[inline(always)]
    const fn len(&self) -> usize {
        self.length
    }
    #[inline(always)]
    pub(crate) const fn first_ptr_mut(this: &mut MaybeUninit<[T; N]>) -> *mut T {
        this as *mut MaybeUninit<[T; N]> as *mut T
    }
    #[inline(always)]
    const fn as_mut_ptr(&mut self) -> *mut T {
        Self::first_ptr_mut(&mut self.data)
    }
    #[inline(always)]
    unsafe fn mut_ptr_at_unchecked(&mut self, index: usize) -> *mut T {
        // We check against `N` as opposed to `length` in our debug assertion here, as these
        // `_unchecked` versions of `ptr_at` and `mut_ptr_at` are primarily intended for
        // initialization-related purposes (and used extensively that way internally throughout the
        // crate.)
        debug_assert!(
            index <= N,
            "In `StaticVec::mut_ptr_at_unchecked`, provided index {} must be within `0..={}`!",
            index,
            N
        );
        self.as_mut_ptr().add(index)
    }
    #[inline(always)]
    unsafe fn set_len(&mut self, new_len: usize) {
        // Most of the `unsafe` functions in this crate that are heavily used internally
        // have debug-build-only assertions where it's useful.
        debug_assert!(
            new_len <= N,
            "In `StaticVec::set_len`, provided length {} exceeds the maximum capacity of {}!",
            new_len,
            N
        );
        self.length = new_len;
    }
    #[inline(always)]
    pub(crate) const fn first_ptr(this: &MaybeUninit<[T; N]>) -> *const T {
        this as *const MaybeUninit<[T; N]> as *const T
    }
    #[inline(always)]
    const fn as_ptr(&self) -> *const T {
        Self::first_ptr(&self.data)
    }

    #[inline(always)]
    const fn as_slice(&self) -> &[T] {
        // Safety: `self.as_ptr()` is a pointer to an array for which the first `length`
        // elements are guaranteed to be initialized. Therefore this is a valid slice.
        slice_from_raw_parts(self.as_ptr(), self.length)
    }
}

pub(crate) struct StaticString<const N: usize> {
    pub(crate) vec: StaticVec<u8, N>,
}

impl<const N: usize> StaticString<N> {
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self {
            vec: StaticVec::new(),
        }
    }
    #[inline(always)]
    pub(crate) fn try_from_str<S: AsRef<str>>(string: S) -> Result<Self, CapacityError<N>> {
        let mut res = Self::new();
        res.try_push_str(string)?;
        Ok(res)
    }

    #[inline(always)]
    pub(crate) const fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline(always)]
    pub(crate) const fn remaining_capacity(&self) -> usize {
        self.vec.remaining_capacity()
    }

    #[inline(always)]
    pub(crate) unsafe fn push_str_unchecked(&mut self, string: &str) {
        let string_length = string.len();
        debug_assert!(string_length <= self.remaining_capacity());
        let old_length = self.len();
        let dest = self.vec.mut_ptr_at_unchecked(old_length);
        string.as_ptr().copy_to_nonoverlapping(dest, string_length);
        self.vec.set_len(old_length + string_length);
    }

    #[inline(always)]
    pub(crate) fn try_push_str<S: AsRef<str>>(
        &mut self,
        string: S,
    ) -> Result<(), CapacityError<N>> {
        let string_ref = string.as_ref();
        match self.vec.remaining_capacity() < string_ref.len() {
            false => {
                unsafe { self.push_str_unchecked(string_ref) };
                Ok(())
            }
            true => Err(CapacityError {}),
        }
    }
    #[inline(always)]
    pub(crate) const fn as_str(&self) -> &str {
        unsafe { &*(self.as_bytes() as *const [u8] as *const str) }
    }

    #[inline(always)]
    pub(crate) const fn as_bytes(&self) -> &[u8] {
        self.vec.as_slice()
    }
}

impl<const N: usize> Debug for StaticString<N> {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("StaticString")
            .field("array", &self.as_str())
            .field("size", &self.len())
            .finish()
    }
}

#[derive(Debug)]
struct User {
    username: StaticString<20>,
    role: StaticString<5>,
}

fn main() -> Result<(), StringError> {
    let user = User {
        username: StaticString::try_from_str("user")?,
        role: StaticString::try_from_str("admin")?,
    };
    println!("{:?}", user);
    Ok(())
}
