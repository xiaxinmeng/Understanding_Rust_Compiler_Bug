
error[[E0597]](https://doc.rust-lang.org/stable/error-index.html#E0597): `baz` does not live long enough
  --> src/main.rs:16:10
   |
16 |     foo(&&baz);
   |     -----^^^^-
   |     |    |
   |     |    borrowed value does not live long enough
   |     argument requires that `baz` is borrowed for `'static
