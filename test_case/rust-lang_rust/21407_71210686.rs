 rust
fn main() {
    use foo::Foo;
    let f1 = Foo::new(13, 14);
    let f2 = Foo { x: 23, y: 24, ..f1 };
    println!("f2: {:?}", f2);
}

mod foo {
    pub struct Foo {
        pub x: u8,
        pub y: u8,
        pub private_data: FooPrivate,
    }

    pub struct FooPrivate {
        _other_1: u8,
        _other_2: u8,
        // developer is free to add new fields here
    }

    ...
}
