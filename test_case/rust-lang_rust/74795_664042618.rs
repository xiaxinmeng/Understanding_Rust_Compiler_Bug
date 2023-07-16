rust
#[derive(PartialEq)]
struct Wrap<A, B> { /* doesn't matter */ }
// =>
impl<A, B, ARhs, BRhs> PartialEq<Wrap<ARhs, BRhs>> for Wrap<A, B>
where
    A: PartialEq<ARhs>,
    B: PartialEq<BRhs>,
{ /* ... */ }
