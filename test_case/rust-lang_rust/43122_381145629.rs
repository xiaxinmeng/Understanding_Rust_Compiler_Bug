rust
pub trait Generator<'a> {
    type Yield;
    type Return;
    unsafe fn resume(self: Pin<'a, Self>) -> GeneratorState<Self::Yield, Self::Return>;
}
