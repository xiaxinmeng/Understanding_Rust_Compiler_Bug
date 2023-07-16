
struct Foo { ... }
struct Bar { foo: Option<Foo> }

impl Bar {
    // this function may be called frequently
    fn get_foo<'a>(&'a self) -> &'a Foo {
        if (foo.is_some()) {
            foo.get_ref()
        } else {
            static mut foo_default_instance: *Foo = 0 as ...;
            static once = ...
            once.doit(initialize foo_default_instance);
            cast::transmute(foo_default_instance)
        }
    }
}
