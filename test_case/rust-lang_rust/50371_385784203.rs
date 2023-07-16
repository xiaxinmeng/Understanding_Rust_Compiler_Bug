rust
use std::ptr::write_volatile;

fn main() {
    let mut a = Some(1usize);
    unsafe {
        write_volatile(&mut a, None);
    }
}
