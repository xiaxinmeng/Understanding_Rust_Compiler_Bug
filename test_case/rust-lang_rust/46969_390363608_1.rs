rust
trait EmptyTrait {}

trait Const<U> {
    type T;
    const VAL: Self::T;
}

impl<U: EmptyTrait + Sized> Const<U> for () {
    type T = u32;
    const VAL: Self::T = 5;
}
