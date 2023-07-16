rust
pub fn x(callback: fn(*const *const f64)) {
    let a = [100.0, 40.0, 22.0];
    let b = [211.0, 20.0, 2.0];
    let c = [32.0, 190.0, 150.0];
    let d = [2.0, 100.0, 100.0];
    let features = [a.as_ptr(), b.as_ptr(), c.as_ptr(), d.as_ptr()];
    callback(features.as_ptr());
}
