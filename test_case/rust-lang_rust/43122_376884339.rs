rust
pub enum GeneratorState<S, Y, R> {
    Yielded(S, Y),
    Complete(R),
}

pub trait Generator where Self: std::marker::Sized {
    type Yield;
    type Return;
    fn resume(self) -> GeneratorState<Self, Self::Yield, Self::Return>;
}
