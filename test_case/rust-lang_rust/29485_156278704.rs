
#![crate_type="lib"]
// Optimized to memset before loop, but g() could panic.
pub fn f(x: &mut [i32; 1000], g: fn() -> i32) {
    for i in 0..1000 {
        x[i] = 0;
        g();
    }
}
