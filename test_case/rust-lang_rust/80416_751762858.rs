rust
pub fn fft<N>(samples: &GenericArray<i16, Prod<N, LutLen>>) -> (I16F16, I16F16)
where
    N: Mul<LutLen> + Unsigned,
    Prod<N, LutLen>: ArrayLength<i16>,
{
    const ZERO: I16F16 = I16F16::from_bits(0);
    let sin = LUT.iter().cycle();
    let mut cos = LUT.iter().cycle();
    cos.nth(LUT.len() / 4 - 1); // Use nth not skip, so iterator is optimized
    sin.zip(cos).zip(samples).fold(
        (ZERO, ZERO), |(mut real, mut imag), ((&sin, &cos), &sample)| {
            let sample = I1F15::from_bits(sample);
            real += I16F16::from_num(sample.wide_mul(cos));
            imag += I16F16::from_num(sample.wide_mul(sin));
            (real, imag)
    })
}
