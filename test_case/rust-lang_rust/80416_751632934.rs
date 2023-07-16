rust
const LUT: [i16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

pub fn fft(samples: &[i16; 16]) -> (i32, i32) {
    let sin = (0..).map(|i| LUT[i % LUT.len()]);
    let cos = ((LUT.len()/4)..).map(|i| LUT[i % LUT.len()]);
    sin.zip(cos).zip(samples).fold(
        (0, 0), |(mut real, mut imag), ((sin, cos), &sample)| {
            real += sample as i32 * (cos as i32);
            imag += sample as i32 * (sin as i32);
            (real, imag)
    })
}
