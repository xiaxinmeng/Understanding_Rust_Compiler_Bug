 rust
fn f64_as_u64(f: f64) -> u64 {
    let (mantissa, exponent, _sign) = f.integer_decode();
    mantissa >> ((-exponent) & 63)
}
