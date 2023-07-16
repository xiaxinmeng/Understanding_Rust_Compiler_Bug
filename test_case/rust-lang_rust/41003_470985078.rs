
error: no variant `Baz` on enum `Foo`
 --> src/main.rs:6:18
  |
6 |     let x: Foo = Foo::Baz {n: 15};
  |                  ^^^^^^^^ help: did you mean: `Foo::Bar`
