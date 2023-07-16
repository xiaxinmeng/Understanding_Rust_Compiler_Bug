rust
#![feature(inclusive_range_syntax)]
#![allow(private_no_mangle_fns)]

#[inline(never)]
#[no_mangle]
fn triangle_exc(n: u64) -> u64 {
    let mut count = 0;
    for j in (0 .. n + 1) {
        count += j;
    }
    count
}

#[inline(never)]
#[no_mangle]
fn triangle_inc(n: u64) -> u64 {
    let mut count = 0;
    for j in 0 ..= n {
        count += j;
    }
    count
}

fn main() {
    let n: u64 = std::env::args().nth(1).unwrap().parse().unwrap();

    println!("{}", triangle_exc(n));
    println!("{}", triangle_inc(n));
}
