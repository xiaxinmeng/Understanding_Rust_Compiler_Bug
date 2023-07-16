rust
#[test]
fn convert_atypical_nan() {
    let mut loses_info = false;
    let test = Double::from_bits(0x7FF0000000000001);
    assert_eq!(test.category(), Category::NaN);
    let converted: Single = test.convert(&mut loses_info).value;
    // This passes:
    assert_eq!(converted.category(), Category::NaN);
    // This fails w/ `'convert_atypical_nan' panicked at 'inf'`
    let f = converted.to_f32();
    assert!(f.is_nan(), "{}", f);
}
