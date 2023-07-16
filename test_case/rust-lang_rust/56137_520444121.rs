rust
trait Zero {
    type Repr;
    const ZERO: Self::Repr;
}

impl Zero for char {
    type Repr = u32;
    const ZERO: u32 = 0;
}

const ONE: u32 = char::ZERO + 1;
