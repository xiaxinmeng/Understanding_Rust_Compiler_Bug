rust
//#![feature(euclidean_division)]
trait DivEuc {
    fn div_euc(self, rhs: u8) -> Self;
}
impl DivEuc for u32 {
    fn div_euc(self, rhs: u8) -> Self { self / rhs as u32 }
}
fn main() {
    println!("{}", 12u32.div_euc(3u8));
}
