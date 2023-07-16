
trait Contains<T> {
    fn contains<U>(&self, x: &U) -> bool where T: PartialEq<U>;
}

impl<T> Contains<T> for [T] {
    fn contains<U>(&self, x: &U) -> bool where T: PartialEq<U> {
        self.iter().any(|e| e == x)
    }
}
