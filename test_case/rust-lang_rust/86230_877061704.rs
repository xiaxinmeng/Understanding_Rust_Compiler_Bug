plain
Some tests failed in compiletest suite=rustdoc-ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.......................
failures:

---- [ui] rustdoc-ui/nocapture-fail.rs stdout ----


1 error: struct literal body without path
-   --> src/test/rustdoc-ui/nocapture-fail.rs:6:99
+   --> $DIR/nocapture-fail.rs:6:109
3    |
- LL |   fn main() { #[allow(non_snake_case)] fn _doctest_main_src_test_rustdoc_ui_nocapture_fail_rs_6_0() {
-    |  ___________________________________________________________________________________________________^
+ LL |   fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() {
6 LL | | Input: 123
6 LL | | Input: 123
- LL | | } _doctest_main_src_test_rustdoc_ui_nocapture_fail_rs_6_0() }
+ LL | | } _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() }
9    |
9    |
10 help: you might have forgotten to add the struct literal inside the block
11    |
11    |
- LL | fn main() { #[allow(non_snake_case)] fn _doctest_main_src_test_rustdoc_ui_nocapture_fail_rs_6_0() { SomeStruct {
+ LL | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() { SomeStruct {
13 LL | Input: 123
- LL | } } _doctest_main_src_test_rustdoc_ui_nocapture_fail_rs_6_0() }
+ LL | } } _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() }
16 
17 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/nocapture-fail/nocapture-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nocapture-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/nocapture-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/nocapture-fail" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Zunstable-options" "--nocapture" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/nocapture-fail/auxiliary"
------------------------------------------

running 1 test
running 1 test
test /checkout/src/test/rustdoc-ui/nocapture-fail.rs - Foo (line 6) - compile fail ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s


------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: struct literal body without path
  --> /checkout/src/test/rustdoc-ui/nocapture-fail.rs:6:109
   |
LL |   fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() {
LL | | Input: 123
LL | | Input: 123
LL | | } _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() }
   |
   |
help: you might have forgotten to add the struct literal inside the block
   |
LL | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() { SomeStruct {
LL | Input: 123
LL | } } _doctest_main__checkout_src_test_rustdoc_ui_nocapture_fail_rs_6_0() }

error: aborting due to previous error


---
test result: FAILED. 122 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:25:18
