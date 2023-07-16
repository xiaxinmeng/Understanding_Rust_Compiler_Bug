rust
pub trait Generator {
    type Yield<'a>;
    type Return;
    unsafe fn resume<'a>(self: Pin<'a, Self>) -> GeneratorState<Self::Yield<'a>, Self::Return>;
}
