
error[E0312]: lifetime of reference outlives lifetime of borrowed content...
 --> file.rs:7:31
  |
7 |         let _: &'static Foo = self;
  |                               ^^^^
  |
  = note: ...the reference is valid for the static lifetime...
note: ...but the borrowed content is only valid for the anonymous lifetime #1 defined on the method body at 6:5
 --> file.rs:6:5
  |
6 | /     fn foo(&self) -> impl Debug {
7 | |         let _: &'static Foo = self;
8 | |         ()
9 | |     }
  | |_____^
