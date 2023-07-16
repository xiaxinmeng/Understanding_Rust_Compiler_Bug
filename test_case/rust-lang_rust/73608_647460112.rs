rust
use std::{mem, ptr};

unsafe trait Foo {
    const IS_POD: bool;
}

unsafe impl Foo for bool {
    const IS_POD: bool = false;
}

unsafe impl Foo for u8 {
    const IS_POD: bool = true;
}

fn bar<T: Foo + Default>() -> T {
    unsafe {
        if T::IS_POD {
            let mut x = mem::uninitialized();
            ptr::write(&mut x, T::default());
            x
        } else {
            T::default()
        }
    }
}

fn main() {
    let _: bool = bar();
    let _: u8 = bar();
}
