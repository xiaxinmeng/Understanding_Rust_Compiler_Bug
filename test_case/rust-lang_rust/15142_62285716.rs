
pub trait IntIterator {
    fn next<'a>(&'a mut self) -> Option<&'a int>;
}
impl<'a, T> IntIterator for T where T: Iterator<&'a int> {
    fn next<'a>(&'a mut self) -> Option<&'a int> {
        Iterator::next(self)
    }
}
fn main() {}
