
trait RefIterable<T> {
    fn refs<'a, Iter: Iterator<&'a T>>(&'a self) -> Iter;
}
fn main() {}
