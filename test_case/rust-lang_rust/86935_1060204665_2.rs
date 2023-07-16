rust
fn takes_generic_closure<F>(f: F)
    where F: for<T> FnOnce(T)
{ ... }
