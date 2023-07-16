rust
use std::borrow::Cow;

struct X;

impl Clone for X {
    fn clone(&self) -> Self {
        panic!("clone!");
    }
}

fn main() {
    let mut dst = Cow::<X>::Owned(X);
    let src = Cow::Borrowed(&X);

    dst.clone_from(&src);
}
