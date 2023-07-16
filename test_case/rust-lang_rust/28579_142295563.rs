 Rust
fn check_eq<T: Eq>(t: T) { t == t; }
fn main() {
    let x: *mut [u8] = &mut [0,1];
    check_eq(x);
}
