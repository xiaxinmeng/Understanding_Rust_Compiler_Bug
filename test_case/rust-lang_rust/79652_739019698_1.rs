
error[E0769]: tuple variant `Foo::Bar` written as struct variant
 --> src/main.rs:9:11
  |
9 |    if let Foo::Bar{ foo } = f {
  |           ^^^^^^^^^^^^^^^ help: use the tuple variant pattern syntax instead: `Foo::Bar(foo)`
