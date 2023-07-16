
error[E0624]: method `foobar` is private
  --> src/lib.rs:16:5
   |
16 |     test::Foo::<test::A>::foobar();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: librustc_typeck/check/mod.rs:4910: instantiate_value_path: (UFCS) test::Foo<test::A> was a subtype of test::Foo<test::B> but now is not?
  --> src/lib.rs:16:5
   |
16 |     test::Foo::<test::A>::foobar();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
