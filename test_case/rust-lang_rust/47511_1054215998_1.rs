
error[[E0581]](https://doc.rust-lang.org/stable/error-index.html#E0581): return type references lifetime `'e`, which is not constrained by the fn input types
  [--> src/main.rs:73:65
](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=a050229041bd8924c1891ab4756827d7#)   |
73 |     fn execute<'e>(&self, ctx: ExecutionContextOf<'e, Self>) -> Value<'e> {
   |                                                                 ^^^^^^^^^

For more information about this error, try `rustc --explain E0581`.
