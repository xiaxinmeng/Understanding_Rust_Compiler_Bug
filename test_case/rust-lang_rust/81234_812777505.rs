plain
.................................................................................................... 9400/11735
.................................................................................................... 9500/11735
............................................................................i......i................ 9600/11735
.................................................................................................... 9700/11735
......................iiiiiii..iiiiii.i............................................................. 9800/11735
.................................................................................................... 10000/11735
.................................................................................................... 10100/11735
.................................................................................................... 10200/11735
.................................................................................................... 10300/11735
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.105 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 9.989 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.27s

 finished in 2.338 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 994 tests
---
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0552 (line 10782) stdout ----
error: internal compiler error: unrecognized representation hint
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:10783:8
  |
3 | #[repr(D)] // error: unrecognized representation hint
  |
  = note: delayed at compiler/rustc_passes/src/check_attr.rs:1222:35


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1010:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (81128a65a 2021-04-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z normalize-docs -C codegen-units=1 --crate-type bin
query stack during panic:
end of query stack
end of query stack
Some expected error codes were not found: ["E0552"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0552 (line 10782)

test result: FAILED. 972 passed; 1 failed; 21 ignored; 0 measured; 0 filtered out; finished in 7.87s
