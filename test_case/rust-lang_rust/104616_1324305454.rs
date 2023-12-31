plain

error[E0080]: evaluation of constant value failed
 --> lint_example.rs:5:5
  |
5 |     *y; // the address of a `u8` array is unknown and thus we don't know if
  |     ^^ accessing memory with alignment 1, but alignment 4 is required

warning: unused unary operation that must be used
 --> lint_example.rs:5:5
  |
  |
5 |     *y; // the address of a `u8` array is unknown and thus we don't know if
  |     ^^ the unary operation produces a value
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
  |
5 |     let _ = *y; // the address of a `u8` array is unknown and thus we don't know if


error: aborting due to previous error; 2 warnings emitted

---
doc tests for: /checkout/src/doc/rustc/src/tests/index.md
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
 finished in 1.950 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
error: failed to test example in lint docs for `invalid_alignment` in /checkout/compiler/rustc_lint_defs/src/builtin.rs:1019: did not find lint `invalid_alignment` in output of example, got:
warning: constant `FOO` is never used
 --> lint_example.rs:2:7
  |
Build completed unsuccessfully in 0:30:16
---

error[E0080]: evaluation of constant value failed
 --> lint_example.rs:5:5
  |
5 |     *y; // the address of a `u8` array is unknown and thus we don't know if
  |     ^^ accessing memory with alignment 1, but alignment 4 is required

warning: unused unary operation that must be used
 --> lint_example.rs:5:5
  |
  |
5 |     *y; // the address of a `u8` array is unknown and thus we don't know if
  |     ^^ the unary operation produces a value
  = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
  |
  |
5 |     let _ = *y; // the address of a `u8` array is unknown and thus we don't know if


error: aborting due to previous error; 2 warnings emitted



For more information about this error, try `rustc --explain E0080`.


This error was generated by the lint-docs tool.
This tool extracts documentation for lints from the source code and places
them in the rustc book. See the declare_lint! documentation
https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint_defs/macro.declare_lint.html


To re-run these tests, run: ./x.py test --keep-stage=0 src/tools/lint-docs
The --keep-stage flag should be used if you have already built the compiler

