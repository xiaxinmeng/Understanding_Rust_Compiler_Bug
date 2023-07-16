rust
mod serde;

mod bar {
    extern crate serde;
    use self::serde::Foo;
}
