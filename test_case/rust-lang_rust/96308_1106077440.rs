rust
impl<'a> Person<'a> {
    fn new(name: &'a str, age: usize) -> Self {
        Self {
            name,
            age,
        }
    }

    fn get_name(&mut self) -> &'a str {
        // some mutations
        return self.name;
    }

    fn mutate(&mut self) {
        // some mutations
    }

    fn test(&mut self) {
        let name = self.get_name();
        self.mutate();
        name;
    }
}
