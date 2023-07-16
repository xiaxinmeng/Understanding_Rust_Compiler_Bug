rust
fn main() {
    let snan: u32 = 0xff << 23 | 1;

    let round_trip = unsafe {
        let float: f32 = std::mem::transmute(snan);
        std::mem::transmute(float)
    };

    assert_eq!(snan, round_trip);
    // assert_eq!(f32::from_bits(snan).to_bits(), snan);
}
