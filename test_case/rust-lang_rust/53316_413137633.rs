rust
existential type Foo: std::fmt::Debug;

struct Bar(Foo);

fn foo(f: Foo) -> Bar {
    Bar(f)
}
