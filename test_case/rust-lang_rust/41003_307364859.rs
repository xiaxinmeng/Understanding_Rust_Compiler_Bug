rust
fn main() {
    let a = Foo::Bar { }; // occurs here
    let b = Foo::bar();   // doesn't occur here
}

struct Baz {
    bar: Foo::Bar // occurs here
}

struct Foo { }
