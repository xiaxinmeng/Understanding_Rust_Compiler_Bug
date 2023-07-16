rust
#[repr(Swift5)]
#[derive(Copy, Clone)]
struct SwiftTy {
    x: u16,
    y: u8,
}

#[repr(Swift5)]
struct SwiftTyOuter {
    x: SwiftTy,
    y: NonZeroU8,
}

macro_rules! check {
    ($t:ty, $align:expr, $size:expr) => ({
        assert_eq!(mem::align_of::<$t>(), $align);
        assert_eq!(mem::size_of::<$t>(), $size);
    });
}

fn main() {
    check!(SwiftTy, 2, 4);
    check!(SwiftTyOuter, 2, 4);
}
