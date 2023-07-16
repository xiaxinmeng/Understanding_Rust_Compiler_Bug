rust
fn main() { unsafe {
    let x = Box::new(4);
    let x_copy = std::ptr::read(&x);
    drop(x);
    std::mem::forget(x_copy);
} }
