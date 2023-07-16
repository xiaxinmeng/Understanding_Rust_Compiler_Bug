plain
---- [ui] ui/type/issue-91268.rs stdout ----
diff of stderr:

1 error: this file contains an unclosed delimiter
-   --> $DIR/issue-xxxxx.rs:9:12
3    |
4 LL | fn main() {
5    |           - unclosed delimiter


9    |          unclosed delimiter
10 
11 error: this file contains an unclosed delimiter
-   --> $DIR/issue-xxxxx.rs:9:12
13    |
14 LL | fn main() {
15    |           - unclosed delimiter


19    |          unclosed delimiter
20 
21 error[E0412]: cannot find type `ţ` in this scope
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/issue-xxxxx.rs:9:11
23    |
24 LL |     0: u8(ţ
24 LL |     0: u8(ţ
25    |           ^ expecting a type here because of type ascription
26 
26 
27 error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
-   --> $DIR/issue-xxxxx.rs:9:8
29    |
30 LL |     0: u8(ţ
30 LL |     0: u8(ţ
31    |        ^^^^ only `Fn` traits may use parentheses
32 
33 error[E0109]: type arguments are not allowed for this type
33 error[E0109]: type arguments are not allowed for this type
-   --> $DIR/issue-xxxxx.rs:9:11
35    |
36 LL |     0: u8(ţ
37    |           ^ type argument not allowed


38 
39 error[E0308]: mismatched types
-   --> $DIR/issue-xxxxx.rs:9:5
41    |
42 LL | fn main() {
42 LL | fn main() {
43    |           - expected `()` because of default return type

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268/issue-91268.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/issue-91268.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/issue-91268.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/issue-91268/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
error[E0412]: cannot find type `ţ` in this scope
  --> /checkout/src/test/ui/type/issue-91268.rs:9:11
   |
LL |     0: u8(ţ
   |           ^ expecting a type here because of type ascription

error[E0214]: parenthesized type parameters may only be used with a `Fn` trait
   |
LL |     0: u8(ţ
LL |     0: u8(ţ
   |        ^^^^ only `Fn` traits may use parentheses
error[E0109]: type arguments are not allowed for this type
  --> /checkout/src/test/ui/type/issue-91268.rs:9:11
   |
LL |     0: u8(ţ
LL |     0: u8(ţ
   |           ^ type argument not allowed

error[E0308]: mismatched types
  --> /checkout/src/test/ui/type/issue-91268.rs:9:5
   |
LL | fn main() {
   |           - expected `()` because of default return type
   |     ^^^^^^^ expected `()`, found `u8`

error: aborting due to 6 previous errors

---
test result: FAILED. 12304 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 142.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:47
