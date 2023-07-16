rust
struct Foo {
    f: fn(),
    g: fn(),
}
const fn foo_new(f: fn(), g: fn()) -> Foo {
    Foo { f, g }
}
