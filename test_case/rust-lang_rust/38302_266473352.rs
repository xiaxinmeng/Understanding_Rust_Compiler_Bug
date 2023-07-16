rust
struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
        if self.0 == "A" {
            panic!();
        }
    }
}

use std::mem::forget;
use std::ptr::drop_in_place;

fn main() {
    let mut x = (PrintDrop("A"), PrintDrop("B"));
    unsafe {
        drop_in_place(&mut x);
        forget(x);
    }
}
