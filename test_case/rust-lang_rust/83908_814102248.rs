plain
.................................................................................................... 9400/11737
.................................................................................................... 9500/11737
.............................................................................i......i............... 9600/11737
.................................................................................................... 9700/11737
.......................iiiiiii..iiiiii.i............................................................ 9800/11737
.................................................................................................... 10000/11737
.................................................................................................... 10100/11737
.................................................................................................... 10200/11737
.................................................................................................... 10300/11737
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..
test result: ok. 2 passed; 0 failed; 31 ignored; 0 measured; 0 filtered out; finished in 0.05s

 finished in 0.102 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.21s

 finished in 2.279 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
   Compiling tempfile v3.1.0
   Compiling serde_json v1.0.59
   Compiling lint-docs v0.1.0 (/checkout/src/tools/lint-docs)
    Finished release [optimized] target(s) in 6.38s
warning: the code example in lint `enum_intrinsics_non_enums` in /checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs failed to generate the expected output: did not find lint `enum_intrinsics_non_enums` in output of example, got:
error: an inner attribute is not permitted in this context
 --> lint_example.rs:4:1
  |
4 | #![feature(variant_count)]
4 | #![feature(variant_count)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error[E0658]: use of unstable library feature 'variant_count'
 --> lint_example.rs:5:1
  |
  |
5 | core::mem::variant_count::<&str>();
  |
  = note: see issue #73662 <https://github.com/rust-lang/rust/issues/73662> for more information
  = note: see issue #73662 <https://github.com/rust-lang/rust/issues/73662> for more information
  = help: add `#![feature(variant_count)]` to the crate attributes to enable

error: aborting due to 2 previous errors


---
doc tests for: /checkout/src/doc/rustc/src/what-is-rustc.md
Set({"src/tools/lint-docs"}) not skipped for "bootstrap::test::LintDocs" -- not in ["src/tools/tidy"]
 finished in 0.863 seconds
Generating lint docs (x86_64-unknown-linux-gnu)
error: failed to test example in lint docs for `enum_intrinsics_non_enums` in /checkout/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs:5: did not find lint `enum_intrinsics_non_enums` in output of example, got:
error: an inner attribute is not permitted in this context
 --> lint_example.rs:4:1

  |
  |
4 | #![feature(variant_count)]

  | ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.

error[E0658]: use of unstable library feature 'variant_count'
 --> lint_example.rs:5:1
  |
  |
5 | core::mem::variant_count::<&str>();
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/lint-docs" "--src" "/checkout/compiler" "--out" "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc/rustc/src/lints" "--rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustc-target" "x86_64-unknown-linux-gnu" "--validate"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:27:15
Build completed unsuccessfully in 0:27:15
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: see issue #73662 <https://github.com/rust-lang/rust/issues/73662> for more information
  = help: add `#![feature(variant_count)]` to the crate attributes to enable

error: aborting due to 2 previous errors



For more information about this error, try `rustc --explain E0658`.


This error was generated by the lint-docs tool.
This tool extracts documentation for lints from the source code and places
them in the rustc book. See the declare_lint! documentation
https://doc.rust-lang.org/nightly/nightly-rustc/rustc_lint_defs/macro.declare_lint.html
for an example of the format of documentation this tool expects.

To re-run these tests, run: ./x.py test --keep-stage=0 src/tools/lint-docs
The --keep-stage flag should be used if you have already built the compiler

