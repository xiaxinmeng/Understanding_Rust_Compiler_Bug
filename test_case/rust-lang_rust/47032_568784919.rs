rust
#[derive(Clone)]
pub struct DummyHolder<T> {
    pub data: T,
}

#[derive(Clone)]
pub struct Node {
    pub children: DummyHolder<Cow<'static, [Node]>>,
}
