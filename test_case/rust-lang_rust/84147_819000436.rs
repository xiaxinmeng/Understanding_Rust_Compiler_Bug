rust
struct Array([i32; 10]);

impl std::ops::Deref for Array {
    type Target = [i32; 10];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
