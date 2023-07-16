
#![feature(i128_type)]

fn x(i: u128) -> isize {
    (i % 10000) as isize
}

fn main() {
    let y = (0x1u128 << 64);
    println!("{}", x(y));
}
