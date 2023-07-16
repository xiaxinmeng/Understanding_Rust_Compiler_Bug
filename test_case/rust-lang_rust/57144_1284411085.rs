rust
use core::mem::{size_of, ManuallyDrop};

pub const unsafe fn zeroed<T: Sized>() -> ManuallyDrop<T> {
    union U<T: Sized> {
        bytes: [u8; size_of::<T>()],
        value: ManuallyDrop<T>,
    }

    U {
        bytes: [0; size_of::<T>()],
    }
    .value
}
