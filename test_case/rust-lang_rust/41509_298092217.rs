
unsafe fn float_to_exponential_common_exact<T>(fmt: &mut Formatter, num: &T,
                                               sign: flt2dec::Sign, precision: usize,
                                               upper: bool) -> Result
    where T: flt2dec::DecodableFloat
{
    let mut buf: [u8; 1024] = mem::uninitialized(); // enough for f32 and f64
    let mut parts: [flt2dec::Part; 7] = mem::uninitialized();
    let formatted = flt2dec::to_exact_exp_str(flt2dec::strategy::grisu::format_exact,
                                              *num, sign, precision,
                                              upper, &mut buf, &mut parts);
    fmt.pad_formatted_parts(&formatted)
}
