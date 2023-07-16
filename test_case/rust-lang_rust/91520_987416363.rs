rust
struct Inner<T>(T);
impl Clone for Inner<()> {
    fn clone(&self) -> Self { todo!() }
}

#[derive(Clone)]
struct Outer<T>(Inner<T>);
