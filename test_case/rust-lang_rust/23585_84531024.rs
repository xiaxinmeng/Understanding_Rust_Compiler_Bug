 rust
mod B {
    pub use self::A::foo;

    mod A {
        pub struct X;
        pub fn foo(_: X) {}
    }
}
