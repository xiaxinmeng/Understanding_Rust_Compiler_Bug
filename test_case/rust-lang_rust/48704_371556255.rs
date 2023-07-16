
warning: trait objects without an explicit `dyn` are deprecated
 --> test.rs:4:12
  |
4 | fn foo(x: &::Foo) {
  |            ^^^^^ help: use `dyn`: `dyn (::Foo)`
