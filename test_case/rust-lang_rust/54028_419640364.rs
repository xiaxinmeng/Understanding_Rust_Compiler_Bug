rust
#[repr(align(16))]
pub struct Complex {
    real: f64,
    imag: f64,
}

pub fn load_unaligned(z: Complex) -> f64 {
    let Complex { real, imag } = z;
    
    (real * real) + (imag * imag)
}
