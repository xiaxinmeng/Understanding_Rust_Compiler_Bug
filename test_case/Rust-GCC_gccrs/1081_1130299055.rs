rust
match (f, g) {
        (Foo::A, 1) => { return 5; }
        (Foo::A, 2) => { return 10; }
        (Foo::B, 2) => { return 15; }
        _ => {}
    }
