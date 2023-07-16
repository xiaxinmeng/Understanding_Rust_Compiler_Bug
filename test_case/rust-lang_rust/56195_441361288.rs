rust
#[deprecated(note="Use std .step_by() instead", since="0.8")]
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Step<I> {
    iter: Fuse<I>,
    skip: usize,
}
