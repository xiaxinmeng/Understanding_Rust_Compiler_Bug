rust
fn update1<F>(&self, f: F) where F: FnOnce(T) -> T;
fn update2<F, R>(&self, f: F) -> R where F: FnOnce(T) -> (T, R);
