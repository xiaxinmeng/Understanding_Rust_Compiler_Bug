
error: meta-variable repeats with different Kleene operator
  --> src/custom_punctuation.rs:89:71
   |
79 |     ($ident:ident, $($tt:tt)+) => {
   |                             - expected repetition
...
89 |             let _validate_len = 0 $(+ custom_punctuation_len!(strict, $tt))*;
   |                                                                       ^^^  - conflicting repetition
   |
   = note: #[deny(meta_variable_misuse)] on by default
