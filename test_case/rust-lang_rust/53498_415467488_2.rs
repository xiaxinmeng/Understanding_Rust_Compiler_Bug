
error[E0599]: no function or associated item named `foobar` found for type `test::Foo<test::A>` in the current scope
  --> src/lib.rs:16:5
   |
4  |     pub struct Foo<T>(T);
   |     --------------------- function or associated item `foobar` not found for this
...
16 |     test::Foo::<test::A>::foobar();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function or associated item not found in `test::Foo<test::A>`
