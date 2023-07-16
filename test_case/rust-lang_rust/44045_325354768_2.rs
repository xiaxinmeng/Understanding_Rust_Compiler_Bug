rust
impl<A, B> Clone for (A, B) where A: Clone, B: Clone { ... }
impl<A, B, C> Clone for (A, B, C) where A: Clone, B: Clone, C: Clone { ... }
// ... and so on up to tuples of 8 elements
