
warning[E0621]: explicit lifetime required in the type of `s`
  --> src/lib.rs:16:9
   |
15 |     fn parsed_len(s: &str) -> usize {
   |                      ---- help: add explicit lifetime `'a` to the type of `s`: `&'a str`
16 |         Self::parse(s).len()
   |         ^^^^^^^^^^^^^^ lifetime `'a` required
   |
   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
   = note: for more information, try `rustc --explain E0729`
