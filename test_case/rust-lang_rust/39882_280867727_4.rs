 rust
fn main() {
    let mut x = (24, ());
    unsafe {
        ptr::write(&mut x, (42, ()));
    }
}
