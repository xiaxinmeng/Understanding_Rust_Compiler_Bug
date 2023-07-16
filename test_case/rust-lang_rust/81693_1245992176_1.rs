rust
use std::task::Poll;
use std::mem::ManuallyDrop;
struct A<'a>(&'a i32);
impl<'a> Drop for A<'a> {
    fn drop(&mut self) {}
}

fn main() {
    let mut x = 10;
    let r: Poll<_> = Poll::Ready(ManuallyDrop::new(A(&x)));
    match r {
        Poll::Ready(a) => {
            ManuallyDrop::into_inner(a);
            x = 11;
        }
        Poll::Pending => {}
    }
}
