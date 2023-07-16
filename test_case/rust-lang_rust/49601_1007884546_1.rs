rust
pub trait Helper<I, X>: FnOnce(I) -> <Self as Helper<I, X>>::Intermediate {
    type Intermediate: Iterator<Item=X>;
}

impl<F, I, O, X> Helper<I, X> for F where F: FnOnce(I) -> O, O: Iterator<Item=X> {
    type Intermediate = O;
}

pub fn foo<F>(_f: F) where for<'a> F: Helper<&'a i32, &'a i64> {}
