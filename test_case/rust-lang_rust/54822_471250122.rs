
trait ConstDefault {
    const DEFAULT: Self;
}

trait Foo {
    type T;
}

trait Bar<F: Foo> {
    const T: F::T;
}

impl<T: ConstDefault, F: Foo<T = T>> Bar<F> for () {
    const T: T = T::DEFAULT;
}

fn main() {}
