rust
pub struct Service {
    field: Option<...>,
}

impl Service {
    /// Precondition: xyz has not been frobbed yet.
    pub fn f(&self) {
        self.field.expect_none("...");
        ...
    }
}
