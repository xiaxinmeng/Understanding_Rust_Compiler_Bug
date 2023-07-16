rust
error: `Foo::A` is being called, but it is not a function
 --> <anon>:7:15
  |
7 |     let foo = Foo::A(string);
  |               ^^^^^^^^^^^^^^
  |
  = help: did you mean to write `Foo::A`?
note: defined here
 --> <anon>:2:5
  |
2 |     A,
  |     ^
