
error[E0308]: mismatched types
  --> tests/ui/associated-inherent-types/issue-109789.rs:17:1
   |
17 | fn bar(_: Foo<for<'a> fn(&'a ())>::Assoc) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected struct `Foo<fn(&'static ())>`
              found struct `Foo<for<'a> fn(&'a ())>`

error[E0308]: mismatched types
  --> tests/ui/associated-inherent-types/issue-109789.rs:17:11
   |
17 | fn bar(_: Foo<for<'a> fn(&'a ())>::Assoc) {}
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected struct `Foo<fn(&'static ())>`
              found struct `Foo<for<'a> fn(&'a ())>`
