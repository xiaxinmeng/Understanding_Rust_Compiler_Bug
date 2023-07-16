rust
use std::mem::MaybeUninit;

unsafe fn make_fat_ptr<T: ?Sized>(_: &u32) -> *mut T {
    MaybeUninit::<*mut T>::uninit().assume_init()
}

struct VNode<'src> {
    _x: &'src [VNode<'src>]
}

fn main() {
    let g: *mut dyn Fn(&'_ u32) -> Option<VNode<'_>> =
        unsafe {make_fat_ptr(&12)};
}
