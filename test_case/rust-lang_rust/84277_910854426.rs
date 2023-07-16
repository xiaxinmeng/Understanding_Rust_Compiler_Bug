rust

impl FromResidual<Option<Viewport>> for MyResult {
    fn from_residual(_: Option<Viewport>) -> Self {
        Self(Err(SomeError::ViewportNotFound))
    }
}
impl FromResidual<Option<Item>> for MyResult {
    fn from_residual(_: Option<Item>) -> Self {
        Self(Err(SomeError::ItemNotFound))
    }
}
