
pub trait Foo {
    fn load_from() -> Box<Self>;
    fn load() -> Box<Self> {
        Foo::load_from()
    }
}

pub fn load<M: Foo>() -> Box<M> {
    Foo::load()
}
