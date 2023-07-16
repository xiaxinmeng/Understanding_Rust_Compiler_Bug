
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
 --> issue-55608.rs:3:16
  |
3 | fn server() -> impl FilterBase2 {
  |                ^^^^^^^^^^^^^^^^
  |
  = note: hidden type `Map2<[closure@issue-55608.rs:4:36: 4:41]>` captures an empty lifetime

error: aborting due to previous error
