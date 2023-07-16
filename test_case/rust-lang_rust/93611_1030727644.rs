
    match a {
        Foo::A(x) if x => {},
        Foo::A(x) => {},
        Foo::B => {},
        Foo::C => {},
    }
