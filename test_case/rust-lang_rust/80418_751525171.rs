rust
fn fst_ref<T, U>(x: &(T, U)) -> &T { &x.0 }
