rust
use std::f32;
use std::f64;

fn f32_bug() {
    let masked_nan2 = f32::NAN.to_bits() ^ 0x0055_5555;
    assert_eq!(f32::from_bits(masked_nan2).to_bits(), masked_nan2);
}

fn f64_bug() {
    let masked_nan1 = f64::NAN.to_bits() ^ 0x000A_AAAA_AAAA_AAAA;
    assert_eq!(f64::from_bits(masked_nan1).to_bits(), masked_nan1);
}
