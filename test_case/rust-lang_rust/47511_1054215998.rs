Rust
impl ValueOperationWithGat for Box<dyn for<'e> Fn(&'e ExecutionContext<'e>) -> Value<'e>> {
    #[inline]
    // the 'e: 'e works around a compiler bug
    fn execute<'e: 'e>(&self, ctx: ExecutionContextOf<'e, Self>) -> Value<'e> {
        (self)(ctx)
    }
}
