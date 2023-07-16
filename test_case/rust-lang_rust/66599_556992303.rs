rust
fn main() {
    for _ in 0..20 {
        let mut coords = 0;

        let x5sm = (-0.4 / 0.05) as u32;
        let y5sm = (-0.7 / 0.05) as u32;
        let z5sm = (-0.8 / 0.05) as u32;

        coords |= x5sm << 21;
        coords |= y5sm << 10 & 0x1ffc00;
        coords |= z5sm & 0x3ff;

        println!("{}", coords);
    }
}
