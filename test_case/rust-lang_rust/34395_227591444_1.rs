
pub fn main() {
    const X: i8 = ---128;
    const Z: [u8; X as u8 as usize] = [0; X as u8 as usize];
    println!("{}", Z[0]);
    println!("{}", X);
}
