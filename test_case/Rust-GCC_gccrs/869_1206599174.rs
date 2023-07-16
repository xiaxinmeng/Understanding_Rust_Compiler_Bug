rust
mod intrinsics {
    extern "rust-intrinsic" {
        pub fn wrapping_add<T>(a: T, b: T) -> T;
        pub fn rotate_left<T>(a: T, b: T) -> T;
        pub fn rotate_right<T>(a: T, b: T) -> T;
        pub fn offset<T>(ptr: *const T, count: isize) -> *const T;
    }
}

mod mem {
    extern "rust-intrinsic" {
        fn transmute<T, U>(_: T) -> U;
        fn size_of<T>() -> usize;
    }
}

macro_rules! impl_uint {
    ($($ty:ident = $lang:literal),*) => {
        $(
            impl $ty {
                pub fn wrapping_add(self, rhs: Self) -> Self {
                    // intrinsics::wrapping_add(self, rhs)
                    self + rhs
                }

                pub fn rotate_left(self, n: u32) -> Self {
                    unsafe {
                        intrinsics::rotate_left(self, n as Self)
                    }
                }

                pub fn rotate_right(self, n: u32) -> Self {
                    unsafe {
                        intrinsics::rotate_right(self, n as Self)
                    }
                }

                pub fn to_le(self) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        self
                    }
                }

                pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                    Self::from_le(Self::from_ne_bytes(bytes))
                }

                pub const fn from_le(x: Self) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        x
                    }
                }

                pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
                    unsafe { mem::transmute(bytes) }
                }
            }
        )*
    }
}

impl_uint!(
    u8 = "u8",
    u16 = "u16",
    u32 = "u32",
    u64 = "u64",
    u128 = "u128",
    usize = "usize"
);

#[lang = "Range"]
pub struct Range<Idx> {
    pub start: Idx,
    pub end: Idx,
}

pub enum Option<T> {
    None,
    Some(T),
}

trait Step {
    /// Returns the number of steps between two step objects. The count is
    /// inclusive of `start` and exclusive of `end`.
    ///
    /// Returns `None` if it is not possible to calculate `steps_between`
    /// without overflow.
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    /// Replaces this step with `1`, returning itself
    fn replace_one(&mut self) -> Self;

    /// Replaces this step with `0`, returning itself
    fn replace_zero(&mut self) -> Self;

    /// Adds one to this step, returning the result
    fn add_one(&self) -> Self;

    /// Subtracts one to this step, returning the result
    fn sub_one(&self) -> Self;

    /// Add an usize, returning None on overflow
    fn add_usize(&self, n: usize) -> Option<Self>;
}

// These are still macro-generated because the integer literals resolve to different types.
macro_rules! step_identical_methods {
    () => {
        #[inline]
        fn replace_one(&mut self) -> Self {
            mem::replace(self, 1)
        }

        #[inline]
        fn replace_zero(&mut self) -> Self {
            mem::replace(self, 0)
        }

        #[inline]
        fn add_one(&self) -> Self {
            Add::add(*self, 1)
        }

        #[inline]
        fn sub_one(&self) -> Self {
            Sub::sub(*self, 1)
        }
    };
}

macro_rules! step_impl_unsigned {
    ($($t:ty)*) => ($(
        impl Step for $t {
            fn steps_between(start: &$t, end: &$t) -> Option<usize> {
                if *start < *end {
                    // Note: We assume $t <= usize here
                    Some((*end - *start) as usize)
                } else {
                    Some(0)
                }
            }

            fn add_usize(&self, n: usize) -> Option<Self> {
                match <$t>::try_from(n) {
                    Ok(n_as_t) => self.checked_add(n_as_t),
                    Err(_) => None,
                }
            }

            step_identical_methods!();
        }
    )*)
}
macro_rules! step_impl_signed {
    ($( [$t:ty : $unsigned:ty] )*) => ($(
        #[unstable(feature = "step_trait",
                   reason = "likely to be replaced by finer-grained traits",
                   issue = "42168")]
        impl Step for $t {
            #[inline]
            #[allow(trivial_numeric_casts)]
            fn steps_between(start: &$t, end: &$t) -> Option<usize> {
                if *start < *end {
                    // Note: We assume $t <= isize here
                    // Use .wrapping_sub and cast to usize to compute the
                    // difference that may not fit inside the range of isize.
                    Some((*end as isize).wrapping_sub(*start as isize) as usize)
                } else {
                    Some(0)
                }
            }

            #[inline]
            #[allow(unreachable_patterns)]
            fn add_usize(&self, n: usize) -> Option<Self> {
                match <$unsigned>::try_from(n) {
                    Ok(n_as_unsigned) => {
                        // Wrapping in unsigned space handles cases like
                        // `-120_i8.add_usize(200) == Some(80_i8)`,
                        // even though 200_usize is out of range for i8.
                        let wrapped = (*self as $unsigned).wrapping_add(n_as_unsigned) as $t;
                        if wrapped >= *self {
                            Some(wrapped)
                        } else {
                            None  // Addition overflowed
                        }
                    }
                    Err(_) => None,
                }
            }

            step_identical_methods!();
        }
    )*)
}

macro_rules! step_impl_no_between {
    ($($t:ty)*) => ($(
        impl Step for $t {
            #[inline]
            fn steps_between(_start: &Self, _end: &Self) -> Option<usize> {
                None
            }

            #[inline]
            fn add_usize(&self, n: usize) -> Option<Self> {
                self.checked_add(n as $t)
            }

            step_identical_methods!();
        }
    )*)
}

step_impl_unsigned!(usize u8 u16 u32);
step_impl_signed!([isize: usize][i8: u8][i16: u16][i32: u32]);
#[cfg(target_pointer_width = "64")]
step_impl_unsigned!(u64);
#[cfg(target_pointer_width = "64")]
step_impl_signed!([i64: u64]);
// If the target pointer width is not 64-bits, we
// assume here that it is less than 64-bits.
#[cfg(not(target_pointer_width = "64"))]
step_impl_no_between!(u64 i64);
step_impl_no_between!(u128 i128);

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl<A: Step> Iterator for Range<A> {
    type Item = A;

    fn next(&mut self) -> Option<A> {
        if self.start < self.end {
            // We check for overflow here, even though it can't actually
            // happen. Adding this check does however help llvm vectorize loops
            // for some ranges that don't get vectorized otherwise,
            // and this won't actually result in an extra check in an optimized build.
            if let Some(mut n) = self.start.add_usize(1) {
                mem::swap(&mut n, &mut self.start);
                Option::Some(n)
            } else {
                Option::None
            }
        } else {
            Option::None
        }
    }
}

pub trait IntoIterator {
    type Item;

    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}

impl<I: Iterator> IntoIterator for I {
    type Item = I::Item;
    type IntoIter = I;

    fn into_iter(self) -> I {
        self
    }
}

pub fn main() {
    for i in 1..2 {}
}

