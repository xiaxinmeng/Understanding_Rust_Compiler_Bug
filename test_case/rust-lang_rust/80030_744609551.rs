rust
fn main() {
    println!("{:#064b}", 0xFFFF_FFFF_FFFF_FFFFu64);
    println!("{:#064b}", 0x7FFF_FFFF_FFFF_FFFFu64);
    println!("{:#064b}", 0x3FFF_FFFF_FFFF_FFFFu64);
    println!("{:#064b}", 0x1FFF_FFFF_FFFF_FFFFu64);
    println!("{:#064b}", 0x0FFF_FFFF_FFFF_FFFFu64);
}
