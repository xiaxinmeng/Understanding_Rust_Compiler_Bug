
warning: unused variable: `s`
 --> src/lib.rs:1:8
  |
1 | fn foo(s: &i32) -> &i32 {
  |        ^ help: if this is intentional, prefix it with an underscore: `_s`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0381]: use of possibly-uninitialized variable: `xs`
 --> src/lib.rs:3:5
  |
3 |     xs
  |     ^^ use of possibly-uninitialized `xs`

error: aborting due to previous error; 1 warning emitted
