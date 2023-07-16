rust
trait Foo {
    type Assoc: Bar;
}

trait Bar {
    type Assoc2;
}

struct Wrapper<T: Foo> {
    inner: <<T as Foo>::Assoc as Bar>::Assoc2,
    _i: (),
}

fn test<T: Foo<Assoc = S>, S>(s: Wrapper<T>) {
    let _ = s.inner;
    //~^ ERROR the trait bound `S: Bar` is not satisfied
}
