
warning: unreachable expression
 --> src/main.rs:2:5
  |
2 |     assert_eq!((), return ());
  |     ^^^^^^^^^^^^^^^---------^^
  |     |              |
  |     |              any code following this expression is unreachable
  |     unreachable expression
  |
  = note: `#[warn(unreachable_code)]` on by default
  = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

warning: unreachable expression
 --> src/main.rs:2:5
  |
2 |     assert_eq!((), return ());
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     unreachable expression
  |     any code following this expression is unreachable
  |
  = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

warning: unreachable expression
 --> src/main.rs:2:5
  |
2 |     assert_eq!((), return ());
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |     |
  |     unreachable expression
  |     any code following this expression is unreachable
  |
  = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

warning: unreachable expression
 --> src/main.rs:6:20
  |
6 |     println!("{}", return ());
  |                    ^^^^^^^^^
  |                    |
  |                    unreachable expression
  |                    any code following this expression is unreachable
