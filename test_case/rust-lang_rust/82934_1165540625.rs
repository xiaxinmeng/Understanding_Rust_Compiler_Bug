rust
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

struct ArrayVec<T: Array>(Option<T>);
impl<T: Array> ArrayVec<T> {
    fn new() -> Self {
        Self(None)
    }
    
    fn push(&mut self, v: T::Value) {}
    
    fn into_inner(self) -> Result<T, ()> {
        todo!()
    }
}

trait Array {
    type Value;
}
impl<T> Array for [T; 0] {
    type Value = T;
}
impl<T> Array for [T; 1] {
    type Value = T;
}

fn append<T, const N: usize>(start: [T; N], v: T) -> [T; N + 1] {
    let mut xs = ArrayVec::new();
    for _ in 0..N {
        xs.push(v);
    }
    xs.push(v);
    match xs.into_inner() {
        Ok(xs) => xs,
        _ => unreachable!()
    }
}
