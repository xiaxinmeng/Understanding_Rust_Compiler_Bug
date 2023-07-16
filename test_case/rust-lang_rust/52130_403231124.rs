rust
// original code
impl Struct {
    fn method(&self) { ... }
}

// adding an argument after some refactoring...
impl Struct {
    fn method(&self, &other: Struct) { ... }  // oops! Should be `other: &Struct`
}
