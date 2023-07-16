rust
use std::marker::PhantomData;

pub struct One<T>(PhantomData<T>);
pub struct Two<T, U>(PhantomData<(T, U)>);

unsafe impl<T, U> Send for Two<T, U> where U: IsOne<T> {}

pub trait IsOne<T> {}
impl<T> IsOne<T> for One<T> {}

fn main() {
    fn assert_send(_: impl Send) {}
    assert_send(async {
        //struct T(Box<dyn Send + Sync + 'static>);   // <-- WORKS
        type T = Box<dyn Send + Sync + 'static>;      // <-- FAILS
        let _value = Two::<T, One<T>>(PhantomData);
        async {}.await;
    });
}
