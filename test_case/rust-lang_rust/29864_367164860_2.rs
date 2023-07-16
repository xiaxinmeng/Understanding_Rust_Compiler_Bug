rust
pub trait F {}
impl<T> F for T where T: 'static {}
impl<T> F for T where T: Copy {}
