rust
#[simd]
fn funnything(a: &[f32], b: &[f32], out: &mut [f32]) {
    foreach i in 0..a.len() {
        if a[i] > b[i] {
            out = a[i] - b[i];
        } else {
            out = a[i] + b[i];
        }
    }
}
