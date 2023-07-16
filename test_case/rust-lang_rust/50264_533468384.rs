
impl Deref for Option<T> where T: Deref {
    type Target = <T as Deref>::Target;
    pub fn deref(&self) -> Option<&<T as Deref>::Target> {
        self.as_ref().map(|t| t.deref())
    }
}
impl DerefMut for Option<T> where T: Deref+DerefMut {
    pub fn deref_mut(&self) -> Option<&<T as Deref>::Target> {
        self.as_mut().map(|t| t.deref_mut())
    }
}
