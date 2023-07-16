rust
impl From<String> for Box<dyn Error + Send> {}
impl<'a> From<&str> for Box<dyn Error + Send + 'a> {}
impl<'a, 'b> From<Cow<'a, str>> for Box<dyn Error + Send + 'b> {}
