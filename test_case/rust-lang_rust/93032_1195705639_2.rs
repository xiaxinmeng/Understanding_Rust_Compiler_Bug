rust
fn foobar<T, U>(foo: impl FnOnce() -> T, bar: impl FnOnce() -> U) -> (T, U) {
   (foo(), bar()) 
}
