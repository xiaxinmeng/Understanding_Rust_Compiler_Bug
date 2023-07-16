text
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/lib.rs:16:5
   |
16 |     A::foo()(x);
   |     ^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 15:8...
  --> src/lib.rs:15:8
   |
15 | fn bar<'a>(x: &'a i32, _: A) {
   |        ^^
note: ...so that reference does not outlive borrowed content
  --> src/lib.rs:16:14
   |
16 |     A::foo()(x);
   |              ^
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the reference type `&'static fn(&i32)` does not outlive the data it points at
  --> src/lib.rs:16:5
   |
16 |     A::foo()(x);
   |     ^^^^^^^^
