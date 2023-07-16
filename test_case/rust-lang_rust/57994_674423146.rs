rust
use core::pin::Pin;
struct S;

impl S {
    fn x(self: Pin<&mut Self>) {
    }
}

fn main() {
    S.x();
}
