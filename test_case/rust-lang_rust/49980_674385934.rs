
error[E0080]: could not evaluate static initializer
 --> src/lib.rs:1:18
  |
1 | static FOO: () = loop {};
  |                  ^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
