text
error[E0769]: tuple variant `Foo::Bar` written as struct variant
 --> src/main.rs:9:11
  |
9 |    if let Foo::Bar{} = f {
  |           ^^^^^^^^^^
  |
help: use the tuple variant pattern syntax instead
  |
9 |    if let Foo::Bar(_) = f {
  |                   ^^^
