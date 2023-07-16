 rust
pub trait Module<R: Ring>: AdditiveAbelianGroup + ScalarMultiplication<R> {}

pub trait Vector<F: Field>: Module<F> {}
