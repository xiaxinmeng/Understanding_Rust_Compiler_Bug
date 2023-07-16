
error[E0308]: mismatched types
  --> src/main.rs:11:21
   |
11 |                     other
   |                     ^^^^^ lifetime mismatch
   |
   = note: expected type `Foo<'a>`
              found type `Foo<'_>`
note: the anonymous lifetime #2 defined on the method body at 7:5...
  --> src/main.rs:7:5
   |
7  | /     fn bar(&self, other: Foo) -> Foo<'a> {
8  | |         match *self {
9  | |             Foo::Bar(s) => {
10 | |                 if s == "test" {
...  |
16 | |         }
17 | |     }
   | |_____^
note: ...does not necessarily outlive the lifetime 'a as defined on the impl at 6:1
  --> src/main.rs:6:1
   |
6  | / impl<'a> Foo<'a> {
7  | |     fn bar(&self, other: Foo) -> Foo<'a> {
8  | |         match *self {
9  | |             Foo::Bar(s) => {
...  |
17 | |     }
18 | | }
   | |_^
