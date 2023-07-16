rust
pub struct InputWrapper<T>(T);
impl<T> TryFrom<InputWrapper<T>> for MyType { /* */ }
