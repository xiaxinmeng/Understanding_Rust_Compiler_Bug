rust
pub fn case_1(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3],
    ]
}

pub fn case_2(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    let mut c = [0.0; 4];
    for i in 0..4 {
        c[i] = a[i] + b[i];
    }
    c
}
