 rust
#[no_mangle]
pub fn dot_ref_s(xs: &&[u32], ys: &&[u32]) -> u32 {
    let mut s = 0;
    for (x, y) in xs.iter().zip(*ys) {
        s += x * y;
    }
    s
}
