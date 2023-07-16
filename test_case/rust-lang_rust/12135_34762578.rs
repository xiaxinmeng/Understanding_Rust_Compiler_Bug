
impl<T:Eq> Equiv<T> for T {
    fn equiv(&self, other: &T) -> bool { self.eq(other) }
}
