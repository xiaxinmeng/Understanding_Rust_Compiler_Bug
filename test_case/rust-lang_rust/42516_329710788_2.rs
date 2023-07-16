
note: but, the lifetime must be valid for the anonymous lifetime 'anon1 defined on the function body at 4:1...
 --> test.rs:4:1
  |
4 | / fn make_foo<'a, T>(m: &'a mut Bar<T>) -> Foo<'a, T> {
5 | |     Foo(m)
6 | | }
  | |_^
note: fully inferred function definition of test.rs:4:1
  | fn make_foo<'a, 'anon0, T>(m: &'a mut Bar<'anon0, T>) -> Foo<'a, T>
