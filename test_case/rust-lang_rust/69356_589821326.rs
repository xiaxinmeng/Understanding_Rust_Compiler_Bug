rust
centril@centrilnas2:~/programming/rust/rust-gamma$ rustc +nightly foo.rs -Z unpretty=hir
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
pub enum Foo<'a, T> {
    Struct {
    },
    Tuple(),
    Unit,
    Usage(&'a T),
}
pub fn main() {
                  let foo = Foo::Unit::<String>;
                  let foo = Foo::Tuple::<String>();
                  let foo = Foo::Struct::<, String>{};
              }
