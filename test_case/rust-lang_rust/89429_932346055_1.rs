rust
impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.age.hash(state);
        self.cool.hash(state);
    }
}
