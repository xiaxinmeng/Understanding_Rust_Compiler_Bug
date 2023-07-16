
#![feature(i128_type)]
fn x(i: u128) -> u128 {
    (i % 10000)

}

fn main() {
    let y = (0x1u128 << 64);
    println!("{:x} {:x}", (x(y) >> 64) as u64, x(y) as u64);
}
