rust
impl<A, B, ExtendA, ExtendB> Extend<(A, B)> for (ExtendA, ExtendB)
where
    ExtendA: Extend<A>,
    ExtendB: Extend<B>;
