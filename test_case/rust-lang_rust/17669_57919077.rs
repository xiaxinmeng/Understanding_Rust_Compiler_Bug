 rust
use std::vec::MoveItems;

trait IntoIter<I> {
    fn into_iter_(self) -> I;
}

impl<T, I: Iterator<T>> IntoIter<I> for I {
    fn into_iter_(self) -> I {
        self
    }
}

impl<T> IntoIter<MoveItems<T>> for Vec<T> {
    fn into_iter_(self) -> MoveItems<T> {
        self.into_iter()
    }
}

fn into_iter<I, S: IntoIter<I>>(iterable: S) -> I {
    iterable.into_iter_()
}

fn main() {
    let v = vec![0u8, 1, 2];
    // Pick one of these three lines
    let i: MoveItems<u8> = into_iter(v);  // Works
    //let i = v.into_iter_();  // ~ Err1
    //let i = into_iter(v);  // ~ Err2
}
