plain
.................................................................................................... 9000/11196
.................................................................................................... 9100/11196
......................................................................................i......i...... 9200/11196
.................................................................................................... 9300/11196
.........................iiiiii..iiiiii.i........................................................... 9400/11196
.................................................................................................... 9600/11196
.................................................................................................... 9700/11196
.................................................................................................... 9800/11196
............test [ui] ui/issues/issue-74564-if-expr-stack-overflow.rs has been running for over 60 seconds
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.064 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii.....ii....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii...... 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.423 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Winvalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 979 tests
---
test /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0778 (line 15644) ... ok

failures:

---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0492 (line 8569) stdout ----
error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8573:34
  |
6 | static B: &'static AtomicUsize = &A;
  |
  = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
  = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
  = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
Some expected error codes were not found: ["E0492"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0492 (line 8596) stdout ----
error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8600:25
  |
6 | const B: &Cell<usize> = &A;
  |
  = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
  = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
  = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable

error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8608:25
   |
14 | const E: &Cell<usize> = &D.a; // error
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable

error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:8611:15
   |
17 | const F: &C = &D; // error
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
Some expected error codes were not found: ["E0492"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0492 (line 8569)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0492 (line 8596)

