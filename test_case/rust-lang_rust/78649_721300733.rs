rust
fn boxed<'a>(self) -> Pin<Box<dyn Future<Output = Self::Output> + 'a + Send>>
