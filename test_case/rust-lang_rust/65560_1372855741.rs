
error[E0521]: borrowed data escapes outside of associated function
  --> src/main.rs:27:5
   |
24 | #[derive(Clone)]
   |          -----
   |          |
   |          `self` is a reference that is only valid in the associated function body
   |          in this derive macro expansion
25 | pub struct RmbMsg<'a> {
   |                   -- lifetime `'a` defined here
26 |     bus: u32,
27 |     msg: Box<dyn Msg + 'a>,
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |     |
   |     `self` escapes the associated function body here
   |     argument requires that `'a` must outlive `'static`
   |
   = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)
