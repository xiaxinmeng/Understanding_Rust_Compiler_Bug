 rust
pub use A::foo;

mod A {
    pub use A::B::foo;
    struct X;

    mod B {
        pub fn foo(_: super::X) {}
    }
}
