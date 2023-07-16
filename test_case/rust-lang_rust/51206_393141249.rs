rust
impl<A, B, C, D, E, F, G, H> PartialEq<(A, B, C, D, E, F, G, H)> for (A, B, C, D, E, F, G, H) where
    A: PartialEq<A>,
    B: PartialEq<B>,
    C: PartialEq<C>,
    D: PartialEq<D>,
    E: PartialEq<E>,
    F: PartialEq<F>,
    G: PartialEq<G>,
    H: PartialEq<H> + ?Sized, 
