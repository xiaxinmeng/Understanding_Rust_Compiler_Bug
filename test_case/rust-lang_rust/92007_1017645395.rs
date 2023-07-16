plain
Suite("src/test/mir-opt") not skipped for "bootstrap::test::MirOpt" -- not in ["src/tools/tidy"]
Check compiletest suite=mir-opt mode=mir-opt (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 164 tests
...................................i......................................F......i.....F............ 100/164
Some tests failed in compiletest suite=mir-opt mode=mir-opt host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [mir-opt] mir-opt/inline/inline-generator.rs stdout ----


error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/inline/inline-generator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Copt-level=1" "-Zdump-mir=all" "-Zmir-opt-level=4" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-generator" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-generator" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/inline-generator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `g::{opaque#0}`
   |
   |
13 | pub fn g() -> impl Generator<bool> {
   |
   |
note: ...which requires borrow-checking `g`...
   |
   |
13 | pub fn g() -> impl Generator<bool> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `g`...
   |
   |
13 | pub fn g() -> impl Generator<bool> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `g`...
   |
   |
13 | pub fn g() -> impl Generator<bool> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `impl core::ops::generator::Generator<bool>`...
   = note: ...which again requires computing type of `g::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
   |
2  | / #![feature(generators, generator_trait)]
4  | | use std::ops::Generator;
5  | | use std::pin::Pin;
...  |
...  |
15 | |     |a| { yield if a { 7 } else { 13 } }
   | |_^

error: aborting due to previous error

---
---- [mir-opt] mir-opt/inline/issue-78442.rs stdout ----

error: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/inline/issue-78442.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Copt-level=1" "-Zdump-mir=all" "-Zmir-opt-level=4" "-Zvalidate-mir" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/issue-78442" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/issue-78442" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=3" "-Z" "inline-mir" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/inline/issue-78442/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0391]: cycle detected when computing type of `hide_foo::{opaque#0}`
   |
   |
14 | fn hide_foo() -> impl Fn() {
   |
   |
note: ...which requires borrow-checking `hide_foo`...
   |
   |
14 | fn hide_foo() -> impl Fn() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `hide_foo`...
   |
   |
14 | fn hide_foo() -> impl Fn() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `hide_foo`...
   |
   |
14 | fn hide_foo() -> impl Fn() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which requires normalizing `impl Fn()`...
   = note: ...which again requires computing type of `hide_foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
   |
3  | / #![crate_type = "lib"]
4  | |
4  | |
5  | | // EMIT_MIR issue_78442.bar.RevealAll.diff
6  | | // EMIT_MIR issue_78442.bar.Inline.diff
...  |
19 | | fn foo() { // Error won't happen if "foo" isn't used in "iterate" or has generics
   | |_^

error: aborting due to previous error

---
test result: FAILED. 159 passed; 2 failed; 3 ignored; 0 measured; 0 filtered out; finished in 4.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/mir-opt" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "mir-opt" "--mode" "mir-opt" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:47
