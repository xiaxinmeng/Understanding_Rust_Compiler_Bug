plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `suggest_using_text` in this scope
  --> src/librustdoc/passes/check_code_block_syntax.rs:96:27
   |
96 |                 } else if suggest_using_text && code_block.is_ignore {


error[E0308]: `if` and `else` have incompatible types
  --> src/librustdoc/passes/check_code_block_syntax.rs:85:21
   |
82 |                   let msg = if buffer.has_errors {
   |  ___________________________-
83 | |                     "could not parse code block as Rust code"
84 | |                 } else {
84 | |                 } else {
85 | |                     ("Rust code block is empty", false)
   | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&str`, found tuple
86 | |                 };
   | |_________________- `if` and `else` have incompatible types
   = note: expected type `&str`
   = note: expected type `&str`
             found tuple `(&str, bool)`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
