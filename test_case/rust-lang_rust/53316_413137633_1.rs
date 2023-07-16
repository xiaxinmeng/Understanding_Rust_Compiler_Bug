rust
struct Bar<T>(T);

fn foo(f: Foo) -> Bar<Foo> {
    Bar(f)
}
