
fn main() {
    let mut x = ();
    unsafe {
        std::ptr::read(&mut x);
        std::ptr::read_unaligned(&mut x);
        std::ptr::write(&mut x, ());
        std::ptr::write_unaligned(&mut x, ());
    }
}
