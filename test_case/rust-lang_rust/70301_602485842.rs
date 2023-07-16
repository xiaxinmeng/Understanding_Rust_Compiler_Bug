rust
fn main() {
    let mut v = Vec::with_capacity(10);
    v.push(0);
    let v0 = unsafe { &*(&v[0] as *const _) }; // laundering the lifetime -- we take care that `v` does not reallocate, so that's okay.
    v.extend_from_slice(&[1]);
    let _val = *v0;
}
