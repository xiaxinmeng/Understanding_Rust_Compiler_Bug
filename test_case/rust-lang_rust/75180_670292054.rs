rust
// in core
impl<T> From<T> for T;

// in std
impl<'a, E: Error + 'a> From<E> for Box<dyn Error + 'a>;
