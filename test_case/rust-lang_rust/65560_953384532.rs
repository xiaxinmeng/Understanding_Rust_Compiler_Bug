
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
  --> src/main.rs:24:10
   |
24 | #[derive(Clone)]
   |          ^^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined here...
  --> src/main.rs:25:19
   |
25 | pub struct RmbMsg<'a> {
   |                   ^^
note: ...so that the types are compatible
  --> src/main.rs:24:10
   |
24 | #[derive(Clone)]
   |          ^^^^^
   = note: expected `RmbMsg<'a>`
              found `RmbMsg<'_>`
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the types are compatible
  --> src/main.rs:27:5
   |
24 | #[derive(Clone)]
   |          ----- in this derive macro expansion
...
27 |     msg: Box<dyn Msg + 'a>,
   |     ^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `<Box<dyn Msg> as Clone>`
              found `<Box<(dyn Msg + 'static)> as Clone>`
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
