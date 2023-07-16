
#![feature(exclusive_range_pattern)]
#![feature(euclidean_division)]

fn foo1(x: u8) {
    match x {
        0 .. 120 => {},
        120 ..= 255 => {},
    }
}

fn foo2(x: u32) {
    match x % 3 {
        0 => {},
        1 => {},
        2 => {},
    }
}

fn foo3(x: i32) {
    match x.mod_euc(3) - 1 {
        -1 => {},
        0 => {},
        1 => {},
    }
}

fn main() {}
