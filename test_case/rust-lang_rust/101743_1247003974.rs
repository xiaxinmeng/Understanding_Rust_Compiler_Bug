rust
pub type Word = usize;
pub struct Repr<const B: usize>([i32; B]);
pub struct IBig(usize);

pub const fn base_as_ibig<const B: Word>() -> IBig {
    IBig(B)
}

impl<const B: Word> Repr<B> {
    /// The base of the representation. It's exposed as an [IBig] constant.
    pub const BASE: IBig = base_as_ibig::<B>();

    /// Create a [Repr] instance representing value zero
    pub const fn zero() -> Self { todo!() }
}
