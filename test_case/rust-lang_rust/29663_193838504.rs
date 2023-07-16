 rust
#![feature(volatile)]

#[repr(C)]
struct X(u64, u64);

fn main() {
    let mut x = X(0, 0);
    unsafe { std::ptr::write_volatile(&mut x, X(5, 6)) };
}
