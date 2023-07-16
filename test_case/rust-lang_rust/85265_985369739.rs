rust
pub fn case_2(a: [f32; 4], b: [f32; 4], out: &mut [f32; 4]) {
    for i in 0..4 {
        out[i] = a[i] + b[i];
    }
}
