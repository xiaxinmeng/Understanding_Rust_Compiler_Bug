rust
trait EmptyTrait {}

trait Const {
    type T;
    const VAL: Self::T;
}

impl<U: EmptyTrait + std::marker::Sized> Const for U
{
    type T = u32;
    const VAL: u32 = 5;
}
