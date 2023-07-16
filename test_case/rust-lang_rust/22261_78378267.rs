 rust
pub mod x {
    struct Priv;

    mod helper {
        pub fn f(_: Priv) { }
    }

    pub use self::helper::f; // Error here!
}
