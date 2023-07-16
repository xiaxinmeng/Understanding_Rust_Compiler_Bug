
error[E0424]: expected value, found module `self`
 --> src/lib.rs:2:12
  |
2 |     () => (self)
  |            ^^^^ `self` value is a keyword only available in methods with `self` parameter
...
8 |         x!()
  |         ---- in this macro invocation
