 rust
mod inner {
    struct I32;

    trait A {
        fn a(&self) { }
    }

    pub trait B {
        fn b(&self) { }
    }

    pub trait C: A + B {
        fn c(&self) { }
    }

    impl A for I32 {}
    impl B for I32 {}
    impl C for I32 {} // <-- I expected an error here and on the impl of B
}

fn main() {
}
