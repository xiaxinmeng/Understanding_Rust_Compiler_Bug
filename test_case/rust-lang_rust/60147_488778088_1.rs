
error[E0658]: generic associated types are unstable (see issue #44265)
 --> src/lib.rs:2:5
  |
2 |     type Iter<'a>: Iterator<Item=&'a T>;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: src/librustc/ty/subst.rs:426: Region parameter out of range when substituting in region 'a (root type=Some(&'a T)) (index=2)

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:558:9
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
error: aborting due to 2 previous errors
