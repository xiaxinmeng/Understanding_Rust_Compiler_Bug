 rust

#[packed]
struct Packed {
    a: u8,
    b: u32,
    c: [u8, ..3],
}

struct NonPacked {
    a: u8,
    b: u32,
    c: [u8, ..3],
}

fn main() {
    let p = Packed {
        a: 1,
        b: 3,
        c: [3, 4, 5],
    };

    println!("{:?}", p);

    let p = NonPacked {
        a: 1,
        b: 3,
        c: [3, 4, 5],
    };

    println!("{:?}", p);
}
