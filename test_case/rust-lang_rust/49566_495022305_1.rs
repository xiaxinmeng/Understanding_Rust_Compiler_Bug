
error[E0599]: no variant or associated item named `Baz` found for type `Foo` in the current scope
 --> src/main.rs:6:18
  |
1 | enum Foo {
  | -------- variant or associated item `Baz` not found here
...
6 |     let x = Foo::Baz(());
  |                  ^^^
  |                  |
  |                  variant or associated item not found in `Foo`
  |                  help: there is a variant with a similar name: `Bar`
