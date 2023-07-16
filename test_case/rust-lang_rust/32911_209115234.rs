 rust
#[macro_use] mod foo {
    macro_rules! baz {
        () => ("foo")
    }
}

fn foo() {
    println!("{}", baz!());
}

#[macro_use] mod bar {
    macro_rules! baz {
        () => ("bar")
    }
}

fn main() {
    foo();
    println!("{}", baz!());
}
