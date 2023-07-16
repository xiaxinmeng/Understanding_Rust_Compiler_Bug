rust
#[derive(Clone, Copy, Default, Hash)]
#[repr(transparent)]
pub struct OrdFloat<T>(pub T);
