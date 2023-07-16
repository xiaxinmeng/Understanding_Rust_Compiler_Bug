
pub fn fft(samples: &[i16; 16]) -> (i32, i32) {
    let sin = LUT.iter().cycle();
    let mut cos = LUT.iter().cycle();
    cos.nth(LUT.len() / 4);
    ...
}
