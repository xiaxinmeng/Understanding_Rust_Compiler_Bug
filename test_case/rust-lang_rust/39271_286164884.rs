
/// bits to floats
///
/// oh, and also, never produces a sNaN
pub fn from_bits(bits: u32) -> f32 {
    if looks_like_nan(bits) { 
        ::std::mem::transmute::<u32, f32>(bits & !SNAN_BIT)
    } else {
        ::std::mem::transmute::<u32, f32>(bits)
    }
}
