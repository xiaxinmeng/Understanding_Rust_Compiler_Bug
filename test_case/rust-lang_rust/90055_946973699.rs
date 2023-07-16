rust
fn main() {
    let mut a = 0;
    let mut r = &mut a;
    unsafe { std::ptr::replace(&mut r as *mut _, r); }
}
