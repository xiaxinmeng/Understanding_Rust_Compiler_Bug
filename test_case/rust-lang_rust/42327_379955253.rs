rust
trait Try<Other: TryContinue = Self>: TryContinue + Sized {
    fn check(x: Other) -> ControlFlow<Other::Continue, Self>;
}

enum ControlFlow<C, B> {
    Continue(C),
    Break(B),
}
