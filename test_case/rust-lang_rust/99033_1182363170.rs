
Generating lint docs (x86_64-unknown-linux-gnu)
Building stage0 tool lint-docs (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.13s
warning: the code example in lint `unfulfilled_lint_expectations` in /home/jess/src/rust/compiler/rustc_lint_defs/src/builtin.rs failed to generate the expected output: did not find lint `unfulfilled_lint_expectations` in output of example, got:

error[E0554]: `#![feature]` may not be used on the beta release channel
 --> lint_example.rs:1:1
  |
1 | #![feature(lint_reasons)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to previous error


For more information about this error, try `rustc --explain E0554`.

warning: the code example in lint `unused_allocation` in /home/jess/src/rust/compiler/rustc_lint/src/unused.rs failed to generate the expected output: did not find lint `unused_allocation` in output of example, got:

error[E0554]: `#![feature]` may not be used on the beta release channel
 --> lint_example.rs:1:1
  |
1 | #![feature(box_syntax)]
  | ^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to previous error


For more information about this error, try `rustc --explain E0554`.

Rustbook (x86_64-unknown-linux-gnu) - rustc
Documenting stage0 clippy (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.18s
Documenting stage0 miri (x86_64-unknown-linux-gnu)
    Finished release [optimized] target(s) in 0.18s
Build completed successfully in 0:01:45
./x.py doc  406.78s user 29.28s system 413% cpu 1:45.57 total
