 rust
trait Add<Rhs=Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
    fn add_assign(&mut self, Rhs);
}

// the `default` qualifier here means (1) not all items are impled
// and (2) those that are can be further specialized
default impl<T: Clone, Rhs> Add<Rhs> for T {
    fn add_assign(&mut self, rhs: R) {
        let tmp = self.clone() + rhs;
        *self = tmp;
    }
}
