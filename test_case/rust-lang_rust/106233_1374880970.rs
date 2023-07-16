rust
fn foo<T, U>() { bar::<T>() }
fn bar<T>() { foo::<u8, T>() }
