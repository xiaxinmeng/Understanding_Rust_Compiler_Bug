rust
#![crate_type = "lib"]

use std::mem::{size_of, transmute};

fn bug<T>() -> (usize, T) {
    (
        size_of::<T>(),
        transmute(
            [0u8; size_of::<T>()]
        )
    )
}
