rust
<A, B>
    A: Gat,
    B: for<'x> Gat<Domain<'x> = A::Domain<'x>>,
