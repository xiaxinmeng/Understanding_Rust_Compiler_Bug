
trait Foo {}
struct Bar<H>(std::marker::PhantomData<H>);

pub trait Baz {
    type Out<'a>;
}

impl<H: Foo> Baz for Bar<H> {
    type Out<'a> = Bar<&'a H>;
}
