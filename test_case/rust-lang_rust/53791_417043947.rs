
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
  --> src/lib.rs:25:24
   |
25 | fn f2<'a>(s: S<'a>) -> impl T + 'static {
   |                        ^^^^^^^^^^^^^^^^
   |
note: hidden type `impl T` captures the lifetime 'a as defined on the function body at 25:7
  --> src/lib.rs:25:7
   |
25 | fn f2<'a>(s: S<'a>) -> impl T + 'static {
   |       ^^
