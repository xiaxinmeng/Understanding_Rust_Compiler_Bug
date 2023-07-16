rust
#[derive(Copy)]
pub union MaybeUninit<T> {
    uninit: (),
    value: ManuallyDrop<T>,
}

impl<T: Copy> Clone for MaybeUninit<T> { ... }

impl<T> MaybeUninit<T> {
    pub const fn new(val: T) -> MaybeUninit<T> { ... }

    pub const fn uninit() -> MaybeUninit<T> { ... }

    pub fn zeroed() -> MaybeUninit<T> { ... }

    pub fn as_ptr(&self) -> *const T { ... }

    pub fn as_mut_ptr(&mut self) -> *mut T { ... }

    pub unsafe fn assume_init(self) -> T { ... }
}
