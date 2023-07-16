rust

trait Bar{ 
    type Foo<'a>: 'a;
}

struct What<'a, T: Bar> {
    foo: T::Foo<'a>,
}
