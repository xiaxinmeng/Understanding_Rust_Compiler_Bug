rust
trait Foo {} // assume `Foo` is coinductive here
struct Wrapper<T>(T);

impl<T> Foo for Wrapper<Wrapper<T>>
where
    Wrapper<T>: Foo
{} 
