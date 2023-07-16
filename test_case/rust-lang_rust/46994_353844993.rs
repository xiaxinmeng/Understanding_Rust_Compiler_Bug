
#![feature(nll)]
fn main() {
    let mut a = [0u8; 1];
    a[0] = 0.to_string().as_bytes()[0];
}
