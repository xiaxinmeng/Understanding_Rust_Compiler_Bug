rust
#[allow(dead_code)]
#[repr(align(8))]
enum Aligned {
    Zero = 0,
    One = 1,
}

fn id(a: Aligned) -> Aligned { a }

fn main() {
    let aligned = id(Aligned::Zero);
    let _x = match aligned { Aligned::Zero => 0, Aligned::One => 1 };
}
