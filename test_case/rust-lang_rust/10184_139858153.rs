 rust
#[inline(never)]
pub fn f(ary: &[u8; 5]) -> &[u8] {
    let idx = 1e100f64 as usize;
    &ary[idx..]
}

fn main() {
    println!("{}", f(&[1; 5])[0xdeadbeef]);
}
