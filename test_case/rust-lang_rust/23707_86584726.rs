 rust
use std::marker::{PhantomData, PhantomFn};

struct Chan<T> ( PhantomData<T> );

struct Eps;

struct Send<A,R> ( PhantomData<(A, R)> );

trait Dual: PhantomFn<Self> {}

impl Dual for (Eps, Eps) {}

impl <A, T> Dual for (Send<A, T>, Send<A, T>)
    where (T, T): Dual {}

fn request<R, S> (rx: Chan<R>) where (R, S): Dual {
    borrow_request(&rx)
}

fn borrow_request<R, S> (rx: &Chan<R>) where (R, S): Dual {}
