text
warning: trait objects without an explicit `dyn` are deprecated
 --> src/lib.rs:6:26
  |
6 |     fn get_bars() -> Box<Iterator<Item = Self::FooBaz>>;
  |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Iterator<Item = Self::FooBaz>`
  |
  = note: `#[warn(bare_trait_objects)]` on by default

error[E0277]: the trait bound `<<<Self as Foo>::FooBaz as Bar>::BarBaz as std::borrow::ToOwned>::Owned: std::io::Write` is not satisfied
 --> src/lib.rs:3:1
  |
3 |   pub trait Foo {
  |   ^            - help: consider further restricting the associated type: `where <<<Self as Foo>::FooBaz as Bar>::BarBaz as std::borrow::ToOwned>::Owned: std::io::Write`
  |  _|
  | |
4 | |     type FooBaz: Bar;
5 | |     
6 | |     fn get_bars() -> Box<Iterator<Item = Self::FooBaz>>;
7 | | }
  | |_^ the trait `std::io::Write` is not implemented for `<<<Self as Foo>::FooBaz as Bar>::BarBaz as std::borrow::ToOwned>::Owned`
8 | 
9 |   pub trait Bar where <Self::BarBaz as ToOwned>::Owned: Write {
  |   ----------------------------------------------------------- required by `Bar`

error[E0277]: the trait bound `<<<Self as Foo>::FooBaz as Bar>::BarBaz as std::borrow::ToOwned>::Owned: std::io::Write` is not satisfied
 --> src/lib.rs:6:5
  |
6 |     fn get_bars() -> Box<Iterator<Item = Self::FooBaz>>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
  |     |                                                  |
  |     |                                                  help: consider further restricting the associated type: `where <<<Self as Foo>::FooBaz as Bar>::BarBaz as std::borrow::ToOwned>::Owned: std::io::Write`
  |     the trait `std::io::Write` is not implemented for `<<<Self as Foo>::FooBaz as Bar>::BarBaz as std::borrow::ToOwned>::Owned`
...
9 | pub trait Bar where <Self::BarBaz as ToOwned>::Owned: Write {
  | ----------------------------------------------------------- required by `Bar`

