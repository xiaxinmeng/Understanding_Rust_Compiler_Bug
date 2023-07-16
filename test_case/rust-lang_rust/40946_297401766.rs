rust
warning: Quasi-quoting might make incremental compilation very inefficient: NtIdent(..)
  --> derive/src/lib.rs:9:1
   |
9  |   proc_macro_expr_impl! {
   |  _^ starting here...
10 | |     pub fn args_impl(input: &str) -> String {
11 | |         args::Args::new(input)
12 | |             .process().tokens
13 | |             .to_string()
14 | |     }
15 | | }
   | |_^ ...ending here
   |
   = note: this error originates in a macro outside of the current crate
