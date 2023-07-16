rust
struct Box<T, A: Default = ()>(T, A);

impl<T, A: Default> Box<T, A> {
    fn new(x: T) -> Self {
        Box(x, Default::default())
    }
}

fn main() {
    Box::new(1);
}
