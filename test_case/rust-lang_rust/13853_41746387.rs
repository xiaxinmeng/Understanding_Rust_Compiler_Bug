
trait Graph<N> {
    fn nodes<'a, I: Iterator<&'a N>>(&'a self) -> I;
}

impl<N> Graph<N> for Vec<N> {
    fn nodes<'a, I: Iterator<&'a N>>(&'a self) -> I {
        self.iter()
    }
}

pub fn main() {
}
