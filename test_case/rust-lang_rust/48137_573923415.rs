rust
macro_rules! foo {
    ($name:ident; $method:item) => (
        struct $name();
        impl $name {
            $method
        }
    )
}

foo!(Name, fn foo() { println!("Hello world"); });
