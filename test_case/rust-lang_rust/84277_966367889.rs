rust
pub struct PhTaggedControlFlow<Tag, B, C = ()> {
    tag: PhantomData<Tag>,
    inner: ControlFlow<B, C>,
}
