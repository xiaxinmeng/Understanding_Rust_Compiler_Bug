rust
impl<A, B> !Send for (A, B) where A: Copy { }
