rust
trait NotFoo {}

pub trait Foo: NotFoo {
    type OnlyFoo;
}

pub trait Service {
    type AssocType;
}

pub trait ThriftService<Bug: NotFoo>:
    Service<AssocType = <Bug as Foo>::OnlyFoo>
{
    fn get_service(
        &self,
    ) -> Self::AssocType;
}

fn with_factory<H>(factory: ThriftService<()>) {}
