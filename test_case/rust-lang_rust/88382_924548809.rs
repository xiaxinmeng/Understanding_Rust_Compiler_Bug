rust
#![feature(generic_associated_types)]
use std::marker::PhantomData;

trait ScalarRef<'a> {}
trait Scalar {
    type Ref<'a>: ScalarRef<'a>;
}

struct Func<I1: Scalar, I2: Scalar, O: Scalar, F: Fn(I1::Ref<'_>, I2::Ref<'_>) -> O> {
    func: F,
    _phantom: PhantomData<(I1, I2, O)>,
}

impl<I1: Scalar, I2: Scalar, O: Scalar, F: Fn(I1::Ref<'_>, I2::Ref<'_>) -> O> Func<I1, I2, O, F> {
    fn new(func: F) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }
}

impl ScalarRef<'_> for &i32 {}
impl Scalar for i32 {
    type Ref<'a> = &'a i32;
}

fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}

fn main() {
    let expr1 = Func::<i32, i32, i32, _> {
        func: add,
        _phantom: PhantomData,
    }; // works

    let expr2 = Func::<i32, i32, i32, _>::new(add); // compile error
}
