rust
pub struct Hello<F>(F);

impl<F> Hello<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

fn main() {
    Hello::new(|_| ());
}
