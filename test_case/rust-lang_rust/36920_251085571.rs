 Rust
#[no_mangle]
pub fn dot_mut_s(xs: &mut [u32], ys: &mut [u32]) -> u32 {
    let mut s = 0;
    for (x, y) in xs.iter().zip(ys) {
        s += (*x) * (*y);
    }
    s
}
