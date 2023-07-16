rust
#![no_core]

#![feature(no_core)]
#![feature(intrinsics)]

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
        pub fn transmute<T, U>(_: T) -> U;
        pub fn size_of<T>() -> usize;
    }
}

#[lang = "sized"]
pub trait Sized {}

macro_rules! impl_uint {
    ($($false_ty:ident $ty:ident = $lang:literal),*) => {
        $(
            struct $false_ty($ty);

            impl $false_ty {
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
    U8 u8 = "u8",
    U16 u16 = "u16",
    U32 u32 = "u32",
    U64 u64 = "u64",
    U128 u128 = "u128",
    USIZE usize = "usize"
);
