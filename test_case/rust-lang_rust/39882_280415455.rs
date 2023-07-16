rust
fn main () {
    let mut x = (4, ());
    unsafe {
        std::ptr::write(&mut x, (3, ()));
    }
}
