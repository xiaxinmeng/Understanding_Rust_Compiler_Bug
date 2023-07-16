rust
pub struct GenX<S> {
    inner: S,
}

impl<S> Into<S> for GenX<S> {
    fn into(self) -> S {
        self.inner
    }
}

fn main() {
    println!("HI");
}
