rust
fn assoc(_: impl exists<X: Clone> Iterator<Item = X>) {}
fn param(_: impl forall<X: Clone> AsRef<X>) {} // unimplementable w/o `for<X>`
