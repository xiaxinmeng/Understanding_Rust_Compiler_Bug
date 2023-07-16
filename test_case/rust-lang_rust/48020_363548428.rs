rust
use std::ops::Index;
type Output<I, T> = <I as Index<T>>::Output;
