rust
pub fn sum_f32_big_8(a: [f32; 8], b: [f32; 8]) -> [f32; 8] {
    let mut c = [0.0; 8];
    for i in 0..8 {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn sum_f32_big_16(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
    let mut c = [0.0; 16];
    for i in 0..16 {
        c[i] = a[i] + b[i];
    }
    c
}
