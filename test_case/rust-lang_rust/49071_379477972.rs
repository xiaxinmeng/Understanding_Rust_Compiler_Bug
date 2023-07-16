rust
pub struct BED<T>(pub T);

impl<T> Debug for BED<T> { .. }

impl<T> Debug for BED<T> where specialize(T: Debug) { .. }
