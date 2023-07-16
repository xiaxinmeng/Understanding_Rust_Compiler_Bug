 rust
extern crate arena;

fn main() {
    struct Foo<'a> { foo: ::std::cell::Cell<Option<&'a Foo<'a>>> }
    let foo = arena::TypedArena::new();
    let bar = foo.alloc(Foo { foo: ::std::cell::Cell::new(None) });
    bar.foo.set(Some(&*bar));
}
