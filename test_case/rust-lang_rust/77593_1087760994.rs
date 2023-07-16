rust
impl<I, O, F> FnOnce<(I,)> for F
where
    F: FnOnce(&I) -> O,
{
    type Output = O;
    extern "rust-call" fn call_once(self, args: (I,)) -> O {
        self(&args.0)
    }
}
