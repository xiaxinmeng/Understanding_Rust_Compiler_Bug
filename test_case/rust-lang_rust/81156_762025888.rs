plain
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 29 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii

 finished in 0.080 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i........iii.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.35s

 finished in 2.424 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Check compiletest suite=rustdoc mode=rustdoc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
warning: `tidy` is not installed; generated diffs will be harder to read

running 413 tests
F.........F.........................i......F....F.F................................................. 100/413
......................................................................F...............F..F..F....... 200/413
......F.......................F..F.................F....F........................................... 300/413
..................F...............i...................................F...F......................... 400/413
Some tests failed in compiletest suite=rustdoc mode=rustdoc host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [rustdoc] rustdoc/async-move-doctest.rs stdout ----
---- [rustdoc] rustdoc/async-move-doctest.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-move-doctest/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-move-doctest" "/checkout/src/test/rustdoc/async-move-doctest.rs" "--test" "--edition=2018"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/async-move-doctest.rs - (line 8) ... FAILED
---
---- [rustdoc] rustdoc/comment-in-doctest.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/comment-in-doctest/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/comment-in-doctest" "/checkout/src/test/rustdoc/comment-in-doctest.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/comment-in-doctest.rs - (line 10) ... FAILED
test /checkout/src/test/rustdoc/comment-in-doctest.rs - (line 10) ... FAILED

failures:

---- /checkout/src/test/rustdoc/comment-in-doctest.rs - (line 10) stdout ----
error[E0601]: `main` function not found in crate `rust_out`
  |
  |
1 | #![allow(unused)]
  | ^^^^^^^^^^^^^^^^^ consider adding a `main` function at the crate level
error: aborting due to previous error

For more information about this error, try `rustc --explain E0601`.
Couldn't compile the test.
---
---- [rustdoc] rustdoc/doctest-manual-crate-name.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/doctest-manual-crate-name" "/checkout/src/test/rustdoc/doctest-manual-crate-name.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/doctest-manual-crate-name.rs - (line 3) ... FAILED
---
---- [rustdoc] rustdoc/edition-doctest.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-doctest/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-doctest" "/checkout/src/test/rustdoc/edition-doctest.rs" "--test"
------------------------------------------

running 2 tests
test /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 22) ... FAILED
---
---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 22) stdout ----
error: this file contains an unclosed delimiter
 --> /checkout/src/test/rustdoc/edition-doctest.rs:22:15
  |
2 | #![feature(try
  |   -       -   ^
  |   |       |
  |   |       unclosed delimiter
  |   unclosed delimiter
error: aborting due to previous error


Some expected error codes were not found: ["E0574"]
---- /checkout/src/test/rustdoc/edition-doctest.rs - foo (line 3) stdout ----
 --> /checkout/src/test/rustdoc/edition-doctest.rs:3:15
  |
2 | #![feature(try
2 | #![feature(try
  |   -       -   ^
  |   |       |
  |   |       unclosed delimiter
  |   unclosed delimiter
error: expected identifier, found reserved keyword `try`
 --> /checkout/src/test/rustdoc/edition-doctest.rs:3:12
  |
2 | #![feature(try
---
---- [rustdoc] rustdoc/edition-flag.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/edition-flag" "/checkout/src/test/rustdoc/edition-flag.rs" "--test" "--edition=2018"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/edition-flag.rs - main (line 4) ... FAILED
---
---- [rustdoc] rustdoc/issue-18199.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-18199" "/checkout/src/test/rustdoc/issue-18199.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/issue-18199.rs - foo (line 5) ... FAILED
---
  |   -     ^
  |   |
  |   unclosed delimiter

error: cannot find attribute `unsta` in this scope
  |
2 | #![unsta
  |    ^^^^^

---
---- [rustdoc] rustdoc/issue-23106.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23106" "/checkout/src/test/rustdoc/issue-23106.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/issue-23106.rs - main (line 3) ... FAILED
---
---- [rustdoc] rustdoc/issue-23744.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-23744" "/checkout/src/test/rustdoc/issue-23744.rs" "--test"
------------------------------------------

running 2 tests
test /checkout/src/test/rustdoc/issue-23744.rs - foo (line 9) ... FAILED
---
---- [rustdoc] rustdoc/issue-25944.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-25944" "/checkout/src/test/rustdoc/issue-25944.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/issue-25944.rs - main (line 3) ... FAILED
---
---- [rustdoc] rustdoc/issue-30252.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-30252" "/checkout/src/test/rustdoc/issue-30252.rs" "--test" "--cfg" "feature=\"bar\""
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/issue-30252.rs - foo (line 3) ... FAILED
---
---- [rustdoc] rustdoc/issue-38129.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-38129" "/checkout/src/test/rustdoc/issue-38129.rs" "--test"
------------------------------------------

running 5 tests
test /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) ... FAILED
---
---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs (line 47) stdout ----
error: this file contains an unclosed delimiter
 --> /checkout/src/test/rustdoc/issue-38129.rs:47:15
  |
2 | #![feature(cor
  |   -       -   ^
  |   |       |
  |   |       unclosed delimiter
  |   unclosed delimiter
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/src/test/rustdoc/issue-38129.rs - both_attrs_reverse (line 74) stdout ----
---
  |   -           ^
  |   |
  |   unclosed delimiter

error: cannot find attribute `recursion_l` in this scope
  |
2 | #![recursion_l
  |    ^^^^^^^^^^^


error: aborting due to 2 previous errors

Couldn't compile the test.
---- /checkout/src/test/rustdoc/issue-38129.rs - feature_attr (line 41) stdout ----
error: this file contains an unclosed delimiter
 --> /checkout/src/test/rustdoc/issue-38129.rs:41:15
  |
2 | #![feature(cor
  |   -       -   ^
  |   |       |
  |   |       unclosed delimiter
  |   unclosed delimiter
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/src/test/rustdoc/issue-38129.rs - simple (line 10) stdout ----
---
  |   -           ^
  |   |
  |   unclosed delimiter

error: cannot find attribute `recursion_l` in this scope
  |
2 | #![recursion_l
  |    ^^^^^^^^^^^

---
---- [rustdoc] rustdoc/issue-43153.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-43153" "/checkout/src/test/rustdoc/issue-43153.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED
test /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) ... FAILED

failures:

---- /checkout/src/test/rustdoc/issue-43153.rs - Foo (line 6) stdout ----
error[E0765]: unterminated double quote string
  |
  |
2 | include!("auxi

error: aborting due to previous error

For more information about this error, try `rustc --explain E0765`.
---
---- [rustdoc] rustdoc/issue-48377.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-48377" "/checkout/src/test/rustdoc/issue-48377.rs" "--test"
------------------------------------------

running 2 tests
test /checkout/src/test/rustdoc/issue-48377.rs - (line 10) ... FAILED
---
---- [rustdoc] rustdoc/issue-54478-demo-allocator.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-54478-demo-allocator/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/issue-54478-demo-allocator" "/checkout/src/test/rustdoc/issue-54478-demo-allocator.rs" "--test"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc/issue-54478-demo-allocator.rs - (line 19) ... FAILED
test /checkout/src/test/rustdoc/issue-54478-demo-allocator.rs - (line 19) ... FAILED

failures:

---- /checkout/src/test/rustdoc/issue-54478-demo-allocator.rs - (line 19) stdout ----
error: expected one of `::`, `;`, or `as`, found `<eof>`
  |
2 | use std::alloc
2 | use std::alloc
  |          ^^^^^ expected one of `::`, `;`, or `as`
error: aborting due to previous error

Couldn't compile the test.

---
---- [rustdoc] rustdoc/process-termination.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/process-termination" "/checkout/src/test/rustdoc/process-termination.rs" "--test"
------------------------------------------

running 3 tests
test /checkout/src/test/rustdoc/process-termination.rs - check_process_termination (line 20) ... FAILED
---
---- [rustdoc] rustdoc/test_option_check/bar.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/bar" "/checkout/src/test/rustdoc/test_option_check/bar.rs" "--test"
------------------------------------------

running 1 test
running 1 test
test /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6) ... FAILED
failures:


---- /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6) stdout ----
 --> /checkout/src/test/rustdoc/test_option_check/bar.rs:6:15
  |
2 | fn main() { #[
  |           -  -^
---

Couldn't compile the test.

failures:
    /checkout/src/test/rustdoc/test_option_check/bar.rs - foooo (line 6)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


------------------------------------------
---
---- [rustdoc] rustdoc/test_option_check/test.rs stdout ----

error: rustdoc failed!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/test_option_check/test" "/checkout/src/test/rustdoc/test_option_check/test.rs" "--test"
------------------------------------------

running 3 tests
running 3 tests
test /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) ... FAILED
test /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15) ... FAILED
test /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8) ... FAILED
failures:


---- /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6) stdout ----
 --> /checkout/src/test/rustdoc/test_option_check/bar.rs:6:15
  |
2 | fn main() { #[
  |           -  -^
---

Couldn't compile the test.

failures:
    /checkout/src/test/rustdoc/test_option_check/bar.rs - bar::foooo (line 6)
    /checkout/src/test/rustdoc/test_option_check/test.rs - Bar (line 15)
    /checkout/src/test/rustdoc/test_option_check/test.rs - Foo (line 8)
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s


------------------------------------------
---
test result: FAILED. 394 passed; 17 failed; 2 ignored; 0 measured; 0 filtered out; finished in 37.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc" "--mode" "rustdoc" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:19:37
