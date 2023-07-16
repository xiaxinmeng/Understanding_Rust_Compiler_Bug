rust
use std::mem::MaybeUninit;
use std::ptr;

#[derive(Copy, Clone, Debug)]
enum Fruit {
    Apple,
    _Banana,
}

fn foo() -> Fruit {
    unsafe {
        let mut r = MaybeUninit::uninit();
        ptr::write(r.as_mut_ptr(), Fruit::Apple);
        r.assume_init()
    }
}

fn main() {
    println!("{:?}", foo());
}
