rust
impl From<&str> for Box<dyn Error>;
impl<'a> From<&str> for Box<dyn Error + Send + Sync + 'a>;
