rust
macro_rules! generate_foo {
    () => {
        #[macro_export]
        macro_rules! foo { (1) => {} }
    }
}

generate_foo!();

#[macro_export]
macro_rules! foo { (2) => {} }

foo!(2); // Local use, OK
