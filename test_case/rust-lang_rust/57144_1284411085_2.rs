rust
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use core::mem::{size_of, ManuallyDrop};

pub const unsafe fn zeroed<T: Sized>() -> ManuallyDrop<T>
where
    [(); size_of::<T>()]:,
{
    union U<T: Sized>
    where
        [(); size_of::<T>()]:,
    {
        bytes: [u8; size_of::<T>()],
        value: ManuallyDrop<T>,
    }

    U {
        bytes: [0; size_of::<T>()],
    }
    .value
}
