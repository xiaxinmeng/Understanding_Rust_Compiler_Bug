rust
#[repr(packed)]
struct A {
    x: u8,
    y: u64
}

fn main() {
    let a = A { x: 1, y: 2 };
    let _ = &a.y;
}
