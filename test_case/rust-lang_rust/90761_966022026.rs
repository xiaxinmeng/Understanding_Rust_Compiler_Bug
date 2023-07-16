plain
i................................................................................................... 6500/12390
..........................................i....i.........................................i.......... 6600/12390
......i.............i....................................................i.......................... 6700/12390
..........................................................................i......................... 6800/12390
.......................................................F..........F....F............................ 6900/12390
................................................ii.................................................. 7100/12390
......i............................................................................................. 7200/12390
..................................................................i................................. 7300/12390
.................................................................................................... 7400/12390
---
To only update this specific test, also pass `--test-args lint/unused/issue-70041.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/issue-70041.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-70041/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/issue-70041/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
- error: unused macro definition: unused
+ error: unused macro definition: `unused`
2   --> $DIR/unused-macro-rules.rs:4:14
3    |
4 LL | macro_rules! unused {
10 LL | #![deny(unused_macros)]
11    |         ^^^^^^^^^^^^^
12 
- error: unused macro definition: m
---
To only update this specific test, also pass `--test-args lint/unused/unused-macro-rules.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-macro-rules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro-rules" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro-rules/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unused macro definition: `unused`
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:4:14
   |
LL | macro_rules! unused { //~ ERROR: unused macro definition
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:1:9
   |
   |
LL | #![deny(unused_macros)]
   |         ^^^^^^^^^^^^^

error: unused macro definition: `m`
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:11:22
   |
LL |         macro_rules! m { //~ ERROR: unused macro definition

error: unused macro definition: `unused`
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:24:18
   |
   |
LL |     macro_rules! unused { //~ ERROR: unused macro definition
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-macro-rules.rs:23:12
   |
---
- error: unused macro definition: unused
+ error: unused macro definition: `unused`
2   --> $DIR/unused-macro.rs:5:7
3    |
4 LL | macro unused {
10 LL | #![deny(unused_macros)]
11    |         ^^^^^^^^^^^^^
12 
- error: unused macro definition: unused
---
- error: unused macro definition: unused
+ error: unused macro definition: `unused`
26   --> $DIR/unused-macro.rs:21:22
27    |
28 LL |     pub(crate) macro unused {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro/unused-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unused/unused-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unused macro definition: `unused`
  --> /checkout/src/test/ui/lint/unused/unused-macro.rs:5:7
   |
LL | macro unused { //~ ERROR: unused macro definition
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-macro.rs:2:9
   |
   |
LL | #![deny(unused_macros)]
   |         ^^^^^^^^^^^^^

error: unused macro definition: `unused`
  --> /checkout/src/test/ui/lint/unused/unused-macro.rs:15:11
   |
LL |     macro unused { //~ ERROR: unused macro definition
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/unused-macro.rs:14:12
   |
   |
LL |     #[deny(unused_macros)]
   |            ^^^^^^^^^^^^^

error: unused macro definition: `unused`
  --> /checkout/src/test/ui/lint/unused/unused-macro.rs:21:22
   |
LL |     pub(crate) macro unused { //~ ERROR: unused macro definition

error: aborting due to 3 previous errors


---
test result: FAILED. 12276 passed; 3 failed; 111 ignored; 0 measured; 0 filtered out; finished in 140.30s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:57
