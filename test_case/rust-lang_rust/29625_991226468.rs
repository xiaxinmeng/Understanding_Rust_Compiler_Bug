rust
trait FnOnce<'lifetimes..> { // variadic lifetime-generics?
    type Args;
    type Output;

    extern "rust-call"
    fn call_once (self, _: Self::Args)
      -> Self::Output
    where
        Self : Sized,
    ;
}
