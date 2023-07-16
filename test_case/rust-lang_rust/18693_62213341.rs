 Rust
pub trait Get<T> {
    fn get(&self) -> Foo; 
}

fn bar<T: Get<One> + Get<Two>>(bar: &T) {
    // there is no way to determine the trait from the method,
    // because both returns type `Foo`.
    let x = bar.get();
    ...
}
