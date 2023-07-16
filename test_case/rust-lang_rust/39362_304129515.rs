
fn test(f: Foo) {
    match f {
        //~^ ERROR non-exhaustive patterns
        //~| patterns
        Foo::Bar { bar: Bar::A, .. } => (),
        Foo::Bar { bar: Bar::B, .. } => (),
    }
}
