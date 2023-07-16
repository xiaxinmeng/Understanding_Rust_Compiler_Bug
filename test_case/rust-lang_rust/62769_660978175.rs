
error[E0659]: `Foo` is ambiguous (glob import vs macro-expanded name in the same module during import/macro resolution)
 --> src/lib.rs:1:9
  |
1 | pub use Foo::*;
  |         ^^^ ambiguous name
  |
note: `Foo` could refer to the enum defined here
 --> src/lib.rs:3:1
  |
3 | / pub enum Foo {
4 | |     Foo(i32),
5 | | }
  | |_^
note: `Foo` could also refer to the variant imported here
 --> src/lib.rs:1:9
  |
1 | pub use Foo::*;
  |         ^^^^^^
  = help: consider adding an explicit import of `Foo` to disambiguate
