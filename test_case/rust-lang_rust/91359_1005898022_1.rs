console
error[E0716]: temporary value dropped while borrowed
  --> src/main.rs:4:22
   |
4  |             (e,) => &[e],
   |                      ^^-
   |                      | |
   |                      | temporary value is freed at the end of this statement
   |                      creates a temporary which is freed while still in use
...
16 |     let _: &'static [i32] = a!(1); // fail
   |            --------------   ----- in this macro invocation
   |            |
   |            type annotation requires that borrow lasts for `'static`
