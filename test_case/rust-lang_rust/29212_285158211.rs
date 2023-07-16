rust
use std::sync::atomic::AtomicPtr;

fn main() {
    let ptr = &mut 5;
    let some_ptr  = AtomicPtr::new(ptr);
    
    println!("{:p}", some_ptr);
}
