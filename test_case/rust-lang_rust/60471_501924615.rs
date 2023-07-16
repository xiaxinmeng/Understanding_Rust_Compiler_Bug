rust
#![crate_type = "lib"]

#![feature(defer_normalization)]

pub unsafe fn size_of_units<T: Sized>() -> [(); std::mem::size_of::<T>()] {
    [(); std::mem::size_of::<T>()]
}
