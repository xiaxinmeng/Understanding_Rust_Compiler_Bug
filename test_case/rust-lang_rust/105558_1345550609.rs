rust
fn debug_fn(f: impl Fn(&mut fmt::Formatter<'_>) -> fmt::Result) -> impl fmt::Debug {
    struct DebugFn<F>(F);
    impl<F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result> fmt::Debug for DebugFn<F> {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            (self.0)(fmt)
        }
    }
    DebugFn(f)
}
