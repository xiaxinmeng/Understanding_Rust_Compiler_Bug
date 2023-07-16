rust
enum Bar<Foo> {
    X = {
        struct Foo { }
        impl Foo { const Constant: isize = 22; }
        Foo::Constant
    }
}
