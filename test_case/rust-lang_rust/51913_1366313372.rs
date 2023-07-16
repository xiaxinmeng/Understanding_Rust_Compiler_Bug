
error[E0107]: this struct takes 3 generic arguments but 0 generic arguments were supplied
 --> src/main.rs:4:12
  |
4 | type Bar = Foo<C = u32, B = bool, A = String>;
  |            ^^^ expected 3 generic arguments
  |
note: struct defined here, with 3 generic parameters: `A`, `B`, `C`
 --> src/main.rs:6:8
  |
6 | struct Foo<A, B, C> {
  |        ^^^ -  -  -
help: add missing generic arguments
  |
4 | type Bar = Foo<A, B, C, C = u32, B = bool, A = String>;
  |                ++++++++

error[E0229]: associated type bindings are not allowed here
 --> src/main.rs:4:16
  |
4 | type Bar = Foo<C = u32, B = bool, A = String>;
  |                ^^^^^^^ associated type not allowed here
