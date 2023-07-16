rust
use std::sync::RwLock;

fn main() {
    let test_struct = RwLock::new(0u32);

    let read_ref = &*test_struct.read().unwrap();
    println!("read {read_ref}");

    println!("before deadlock!");
    *test_struct.write().unwrap() = 5;
    println!("after deadlock!");
}
