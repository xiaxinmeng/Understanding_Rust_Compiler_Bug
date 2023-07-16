 rust
#![feature(const_fn)]
#![feature(untagged_unions)]

use std::mem;

const unsafe fn zeroed<T>() -> T {
    union U<T> {
        bytes: [u8; mem::size_of::<T>()],
        data: T,
    }

    U {
        bytes: [0; mem::size_of::<T>()],
    }.data
}
