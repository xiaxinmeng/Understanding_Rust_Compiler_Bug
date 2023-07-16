rust
pub trait Signal {
    type Item;
}

pub trait Signal2 {
    type Item2;
}

// Impl Signal2 for all impls of Signal
impl<B, C> Signal2 for B where B: Signal<Item = C> {
    type Item2 = C;
}

pub struct Switch<B: Signal> {
    pub inner: <B as Signal2>::Item2,
}
