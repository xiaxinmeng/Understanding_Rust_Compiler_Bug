rust
use std::mem::MaybeUninit;

#[repr(C)]
#[derive(Default)]
struct Foo {
    a: u16,
    b: u8
}

fn main() {
    let mut f = Foo::default();
    let fmur = &mut f as *mut Foo as *mut MaybeUninit<u8>;
    unsafe {
        dbg!(*std::slice::from_raw_parts_mut(fmur, 4)[3].write(42));
    }
}
