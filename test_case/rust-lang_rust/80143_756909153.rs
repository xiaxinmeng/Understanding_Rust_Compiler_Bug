rust
#[should_panic]
#[test_utils::matches_image("tests/data/blue_pixel.png")]
fn fails_on_mismatching_data() -> Box<[u32]> {
    Box::new([0; 1])
}
