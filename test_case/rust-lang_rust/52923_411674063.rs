rust
#![feature(test)]
mod foo {
    pub mod test {}
    pub use self::test as _test;
    pub use ::test as _test;
}
fn main() {
    use crate::foo::_test;
}
