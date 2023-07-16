rust
pub struct Resource {
    internal: Box<u32>,
    ptr: Box<u32>,
}

pub trait Implementation<Meta, Msg> {
    fn receive(&mut self, msg: Msg, meta: Meta);
}

impl<Meta, Msg, F> Implementation<Meta, Msg> for F
where
    F: FnMut(Msg, Meta) + 'static,
{
    fn receive(&mut self, msg: Msg, meta: Meta) {
        (self)(msg, meta)
    }
}

pub fn create<A, B, Impl>(i: Impl) -> Box<Implementation<A, B>>
where
    Impl: Implementation<A, B> + 'static,
{
        Box::new(i)
}

pub enum Empty {}

pub fn lol() -> Box<Implementation<Resource, Empty>> {
    create(
        |_, _| {},
    )
}
