rust
// rustc one.rs --crate-type=lib
pub trait Future {
    type Output;
}

pub trait IntoFuture {
    type Output;
    type IntoFuture: Future<Output = Self::Output>;
}
