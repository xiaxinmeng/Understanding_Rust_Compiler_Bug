rust
fn main() { unsafe {
    let x: &[u8] = &[1,2,3];
    let x_01 = &x[0..2];
    let raw = x_01.as_ptr();
    let _val = raw.add(2).read(); // pointer is beyond the bounds of the slice it was created from!
} }
