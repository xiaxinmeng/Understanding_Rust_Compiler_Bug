rust
#[repr(u8)]
enum ZeroByte { Zero = 0 }

struct S {
    a: u32,
    b: i128,
}

struct S2 {
    a: u32,
    b: i128,
    c: ZeroByte,
}

fn main() {
    use std::mem::size_of;
    dbg!(size_of::<S>());
    dbg!(size_of::<Option<S>>());
    dbg!(size_of::<S2>());
    dbg!(size_of::<Option<S2>>());
}
