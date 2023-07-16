rust
pub static mut OUTSIDE: i32 = 123;

fn main() {
    static mut INSIDE: i32 = 456;

    unsafe { (OUTSIDE, INSIDE) };
}
