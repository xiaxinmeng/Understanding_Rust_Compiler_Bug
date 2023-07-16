rust
use std::task::Poll;
struct A<'a>(&'a i32);
impl<'a> Drop for A<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let mut x = 10;
    let r: Poll<_> = Poll::Ready(A(&x));
    match r {
        Poll::Ready(a) => {
            drop(a);
            x = 11; // Ok if A wasn't Drop
        }
        Poll::Pending => {}
    }
}
