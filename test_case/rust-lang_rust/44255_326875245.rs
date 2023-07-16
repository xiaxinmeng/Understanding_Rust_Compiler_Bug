rust
use std::marker::PhantomData;

trait Val {
    const VAL: usize;
}

struct Five;
impl Val for Five {
    const VAL: usize = 5;
}

struct Multiply<T, U> {
    _t: PhantomData<T>,
    _u: PhantomData<U>,
}
impl<T: Val, U: Val> Val for Multiply<T, U> {
    const VAL: usize = T::VAL * U::VAL;
}

fn main() {
    let arr: [usize; 25] = [1; <Multiply<Five, Five>>::VAL];
    println!("{:?}", arr);
}
