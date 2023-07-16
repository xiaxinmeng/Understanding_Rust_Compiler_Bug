text
error[E0283]: type annotations required: cannot resolve `&'a (): Foo`
 --> src/lib.rs:3:1
  |
1 |   trait Foo {}
  |   --------- required by `Foo`
2 |
3 | / fn foo<'a, 'b>()
4 | | where
5 | |     &'a (): Foo,
6 | |     &'b (): Foo,
7 | | {}
  | |__^

error: aborting due to previous error
