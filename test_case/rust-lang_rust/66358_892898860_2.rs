rust
#![feature(arbitrary_self_types)]

use std::mem::MaybeUninit;

struct UnsafeCell;
struct T;

impl UnsafeCell {
    fn raw_get(self: *const Self) -> *mut T { unimplemented!() }
}

fn main() {
    let m = MaybeUninit::<UnsafeCell>::uninit();
    unsafe { m.as_ptr().raw_get().write(T) }
    let _uc = unsafe { m.assume_init() };
}
