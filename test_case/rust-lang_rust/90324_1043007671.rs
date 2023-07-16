rust
pub struct Foo(*const i8);

pub trait Whatever: Send {}

impl<T: Send + ?Sized> Whatever for T {}
