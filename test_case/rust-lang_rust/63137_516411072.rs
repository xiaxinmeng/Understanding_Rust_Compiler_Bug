rust
fn main() {
    let x = [1];
    unsafe { x.as_ptr().offset(1); }
}
