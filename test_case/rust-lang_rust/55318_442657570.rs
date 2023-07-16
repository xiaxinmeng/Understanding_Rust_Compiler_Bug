rust
pub trait Pattern {
    type Value;
}

pub struct Constrain<A, B = A, C = A>(A, B, C);

impl<A, B, C> Pattern for Constrain<A, B, C>
    where A: Pattern,
          B: Pattern<Value = A::Value>,
          C: Pattern<Value = A::Value>,
{
    type Value = A::Value;
}

pub struct Wrapper<T>(T);

impl<T> Pattern for Wrapper<T> {
    type Value = T;
}

pub struct WriteAndThen<P1>(pub P1::Value, pub <Constrain<P1, Wrapper<P1::Value>> as Pattern>::Value)
    where P1: Pattern;

