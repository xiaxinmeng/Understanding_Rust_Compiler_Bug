
error: no variant `Baz` in enum `Foo`
 --> src/main.rs:6:18
  |
1 | enum Foo {
  | -------- variant `Baz` not found here
...
6 |     let x = Foo::Baz { id: 1 };
  |                  ^^^ help: there is a variant with a similar name: `Bar`
