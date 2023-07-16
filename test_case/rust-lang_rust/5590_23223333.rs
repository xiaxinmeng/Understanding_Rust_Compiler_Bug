 rust
impl<T: TotalEq> Eq for T {
    fn eq(&self, other: &T) -> bool { self.equals(other) }
}

impl<T: TotalOrd> Ord for T {
    fn lt(&self, other: &T) -> bool { self.cmp(other) == Less }
}
