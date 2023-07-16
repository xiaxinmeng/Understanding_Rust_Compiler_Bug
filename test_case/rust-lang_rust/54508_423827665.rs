rust
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Callback<T: Layout>(pub fn(WindowInfo<T>, usize, usize) -> Dom<T>);
