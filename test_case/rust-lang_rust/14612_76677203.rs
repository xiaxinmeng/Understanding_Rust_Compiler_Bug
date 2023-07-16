 rust
impl<T: Eq> ElemEq for T { ... }
impl<F: FnMut(&T) -> bool> ElemEq for F { ... }
