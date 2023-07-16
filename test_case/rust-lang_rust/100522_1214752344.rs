rust
fn inner<T>() { inner2<T::Next>() }
fn inner2<T>() { inner<T::Next>() }
