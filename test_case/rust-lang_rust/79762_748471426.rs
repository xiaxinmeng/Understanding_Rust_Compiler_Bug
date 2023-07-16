plain
.................................................................................................... 9000/11182
.................................................................................................... 9100/11182
..........................................................................i......i.................. 9200/11182
.................................................................................................... 9300/11182
.............iiiiii..iiiiii.i....................................................................... 9400/11182
.................................................................................................... 9600/11182
.................................................................................................... 9700/11182
.................................................................................................... 9800/11182
.................................................................................................... 9900/11182
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 27 tests
iiiiiiiiiiiiiiiiiiiiiiiiiii

Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
 finished in 0.071 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 11.357 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii...........iiii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.34s

 finished in 2.420 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/rustdoc-ui") not skipped for "bootstrap::test::RustdocUi" -- not in ["src/tools/tidy"]
Check compiletest suite=rustdoc-ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 85 tests
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
....................................F...F.F.................i........................

---- [ui] rustdoc-ui/failed-doctest-compile-fail.rs stdout ----
diff of stdout:


5 failures:
6 
7 ---- $DIR/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
+ warning: function `_doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_compile_fail_rs_9_0` should have a snake case name
+    |
+    |
+ LL | fn main() { fn _doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_compile_fail_rs_9_0() {
+    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_doctest_main_checkout_src_test_rustdoc_ui_failed_doctest_compile_fail_rs_9_0`
+    |
+    = note: `#[warn(non_snake_case)]` on by default
+ warning: 1 warning emitted
+ 
+ 
8 Test compiled successfully, but it's marked `compile_fail`.
10 failures:


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-compile-fail/failed-doctest-compile-fail.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args failed-doctest-compile-fail.rs`
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-compile-fail" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-compile-fail/auxiliary"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9) ... FAILED
test /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9) ... FAILED

failures:

---- /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9) stdout ----
warning: function `_doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_compile_fail_rs_9_0` should have a snake case name
   |
   |
LL | fn main() { fn _doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_compile_fail_rs_9_0() {
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_doctest_main_checkout_src_test_rustdoc_ui_failed_doctest_compile_fail_rs_9_0`
   |
   = note: `#[warn(non_snake_case)]` on by default
warning: 1 warning emitted


Test compiled successfully, but it's marked `compile_fail`.
failures:
    /checkout/src/test/rustdoc-ui/failed-doctest-compile-fail.rs - Foo (line 9)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s
---
diff of stdout:

5 failures:
6 
7 ---- $DIR/failed-doctest-should-panic.rs - Foo (line 9) stdout ----
+ warning: function `_doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_should_panic_rs_9_0` should have a snake case name
+    |
+    |
+ LL | fn main() { fn _doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_should_panic_rs_9_0() {
+    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_doctest_main_checkout_src_test_rustdoc_ui_failed_doctest_should_panic_rs_9_0`
+    |
+    = note: `#[warn(non_snake_case)]` on by default
+ warning: 1 warning emitted
+ 
+ 
8 Test executable succeeded, but it's marked `should_panic`.
10 failures:


The actual stdout differed from the expected stdout.
The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-should-panic/failed-doctest-should-panic.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args failed-doctest-should-panic.rs`
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-should-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-should-panic" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-should-panic/auxiliary"
------------------------------------------

running 1 test
test /checkout/src/test/rustdoc-ui/failed-doctest-should-panic.rs - Foo (line 9) ... FAILED
test /checkout/src/test/rustdoc-ui/failed-doctest-should-panic.rs - Foo (line 9) ... FAILED

failures:

---- /checkout/src/test/rustdoc-ui/failed-doctest-should-panic.rs - Foo (line 9) stdout ----
warning: function `_doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_should_panic_rs_9_0` should have a snake case name
   |
   |
LL | fn main() { fn _doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_should_panic_rs_9_0() {
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_doctest_main_checkout_src_test_rustdoc_ui_failed_doctest_should_panic_rs_9_0`
   |
   = note: `#[warn(non_snake_case)]` on by default
warning: 1 warning emitted


Test executable succeeded, but it's marked `should_panic`.
failures:
    /checkout/src/test/rustdoc-ui/failed-doctest-should-panic.rs - Foo (line 9)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s
---
diff of stdout:

17 For more information about this error, try `rustc --explain E0425`.
18 Couldn't compile the test.
19 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 12) stdout ----
+ warning: function `_doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_output_rs_12_0` should have a snake case name
+    |
+    |
+ LL | fn main() { fn _doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_output_rs_12_0() {
+    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_doctest_main_checkout_src_test_rustdoc_ui_failed_doctest_output_rs_12_0`
+    |
+    = note: `#[warn(non_snake_case)]` on by default
+ warning: 1 warning emitted
+ 
20 Test executable failed (exit code 101).
21 
21 
22 stdout:


The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args failed-doctest-output.rs`
error: 1 errors occurred comparing output.
status: exit code: 101
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/failed-doctest-output.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "--test-args" "--test-threads=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/auxiliary"
------------------------------------------

running 2 tests
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 22) ... FAILED
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 22) ... FAILED
test /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 12) ... FAILED

failures:

---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - OtherStruct (line 22) stdout ----
error[E0425]: cannot find value `no` in this scope
   |
LL | no
   | ^^ not found in this scope


error: aborting due to previous error

For more information about this error, try `rustc --explain E0425`.
Couldn't compile the test.
---- /checkout/src/test/rustdoc-ui/failed-doctest-output.rs - SomeStruct (line 12) stdout ----
warning: function `_doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_output_rs_12_0` should have a snake case name
   |
   |
LL | fn main() { fn _doctest_main__checkout_src_test_rustdoc_ui_failed_doctest_output_rs_12_0() {
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: convert the identifier to snake case: `_doctest_main_checkout_src_test_rustdoc_ui_failed_doctest_output_rs_12_0`
   |
   = note: `#[warn(non_snake_case)]` on by default
warning: 1 warning emitted

Test executable failed (exit code 101).


stdout:
stdout 1
stdout 2

stderr:
stderr 1
stderr 2
thread 'main' panicked at 'oh no', /checkout/src/test/rustdoc-ui/failed-doctest-output.rs:7:1



failures:
---
test result: FAILED. 81 passed; 3 failed; 1 ignored; 0 measured; 0 filtered out; finished in 5.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:33:46
