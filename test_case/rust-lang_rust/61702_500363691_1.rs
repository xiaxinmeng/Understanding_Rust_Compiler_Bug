rust
const fn do_something(x: Foo) -> isize { ... }

enum Foo {
    A = 0,
    B = do_something(Foo::A),
}
