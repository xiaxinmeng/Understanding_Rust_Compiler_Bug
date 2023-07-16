
#![crate_type="lib"]
// Optimized to output result of h() directly into x, but g() could panic.
pub fn f(x: &mut [i32; 100], g: fn() -> i32, h: fn() -> [i32; 100]) {
    let a = h();
    g();
    *x = a;
}
