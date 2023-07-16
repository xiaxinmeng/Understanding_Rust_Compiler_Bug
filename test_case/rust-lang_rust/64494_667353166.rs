rust
trait Foo {
    const VAL: usize;
}

trait MyTrait {}

trait True {}
struct Is<const T: bool>;
impl True for Is<{true}> {}

impl<T: Foo> MyTrait for T where Is<{T::VAL == 5}>: True {}
impl<T: Foo> MyTrait for T where Is<{T::VAL == 6}>: True {}
