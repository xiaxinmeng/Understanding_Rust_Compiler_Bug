 rust
extern crate alloc;

fn main() {
    unsafe {
        println!("{}", alloc::heap::allocate(0 - 1, 8));
    }
}
