
error[E0424]: expected value, found module `self`
 --> src/lib.rs:2:12
  |
2 |       () => (self)
  |              ^^^^ `self` value is a keyword only available in methods with a `self` parameter
...
7 | /     fn y(&self) {
8 | |         x!()
  | |         ---- in this macro invocation
9 | |     }
  | |_____- this function has a `self` parameter, but a macro invocation can only access identifiers it receives from parameters
  |
  = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
