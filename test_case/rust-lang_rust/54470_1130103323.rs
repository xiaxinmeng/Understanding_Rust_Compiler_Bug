rust
fn main() {
    let mut v = Vec::with_capacity(2);
    v.push(0);
    let ptr = &v[0] as *const i32;
    v.push(0);
    unsafe { dbg!(*ptr) };
}
