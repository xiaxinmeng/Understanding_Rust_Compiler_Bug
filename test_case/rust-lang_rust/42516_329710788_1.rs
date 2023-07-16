
note: but, the lifetime must be valid for the anonymous lifetime #1 defined on the function body at 4:1...
 --> test.rs:4:1
  |
4 | / fn make_foo<'a, T>(m: &'a mut Bar<T>) -> Foo<'a, T> {
5 | |     Foo(m)
6 | | }
  | |_^
