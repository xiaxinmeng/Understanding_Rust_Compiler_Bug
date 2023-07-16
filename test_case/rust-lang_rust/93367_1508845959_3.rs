rust
struct Wrapper<'a, G>(dyn Item<G> + 'a);
unsafe impl<'a> Send for Wrapper<'a, ArcGroup> {}
