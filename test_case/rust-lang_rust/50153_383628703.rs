rust
pub struct Resource {
    a: u8,
    b: u8,
}

pub fn create<A, B, Impl>(i: Impl) -> Box<FnMut(A, B)>
where
    Impl: FnMut(A, B) + 'static,
{
        Box::new(i)
}

pub enum Empty {}

pub fn lol() -> Box<FnMut(Resource, Empty)> {
    create(
        |_, _| {},
    )
}
