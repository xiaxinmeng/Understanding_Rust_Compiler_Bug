
impl std::ops::DerefMut for B {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
