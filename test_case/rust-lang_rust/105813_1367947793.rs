rust
type T = [u64; 32];
#[no_mangle]
pub fn f(a: T, b: fn(_: T, _: T)) {
    b(a, a)
}
