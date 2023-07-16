
error[E0046]: not all trait items implemented, missing: `fmt`
  --> src/main.rs:17:1
   |
17 | impl std::fmt::Display for A {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fmt` in implementation
   |
   = note: `fmt` from trait: `fn(&Self, &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>`

error[E0046]: not all trait items implemented, missing: `Err`, `from_str`
  --> src/main.rs:19:1
   |
19 | impl FromStr for A{}
   | ^^^^^^^^^^^^^^^^^^ missing `Err`, `from_str` in implementation
   |
   = note: `Err` from trait: `type Err;`
   = note: `from_str` from trait: `fn(&str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err>`

error[E0046]: not all trait items implemented, missing: `Foo`, `foo`, `bar`, `bay`
  --> src/main.rs:21:1
   |
6  |       type Foo;
   |       --------- `Foo` from trait
...
9  |       fn foo() -> T;
   |       -------------- `foo` from trait
10 |       fn bar();
   |       --------- `bar` from trait
11 | /     fn    bay<
12 | |         'lifetime,    TypeParameterA
13 | |             >(  a   : usize,
14 | |                 b: u8  );
   | |_________________________- `bay` from trait
...
21 |   impl X<usize> for A {
   |   ^^^^^^^^^^^^^^^^^^^ missing `Foo`, `foo`, `bar`, `bay` in implementation
