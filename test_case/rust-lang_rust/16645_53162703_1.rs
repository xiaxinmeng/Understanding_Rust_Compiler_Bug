 rust
// my_crate
pub use self::pub_mod::Foo;
pub mod pub_mod {
     pub use self::priv_mod::Foo;
     mod priv_mod {
         pub struct Foo;
    }
}
