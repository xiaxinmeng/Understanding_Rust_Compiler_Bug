diff
warning: unused variable: `f1`
 --> src/main.rs:9:15
  |
9 |     let S{ref f1} = s;
- |               ^^ help: tryignoring the field: `f1: _`
+ |           ^^^^^^ help: try ignoring the field: `f1: _`
  |
  = note: #[warn(unused_variables)] on by default
