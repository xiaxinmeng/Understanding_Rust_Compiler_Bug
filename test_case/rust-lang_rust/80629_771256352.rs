plain
.................................................................................................... 1000/11335
...........................................................................i........................ 1100/11335
.................................................................................................... 1200/11335
.................................................................................................... 1300/11335
F....F..........................................................ii.i.............i.................. 1400/11335
.................................................................................................... 1600/11335
.................................................................................................... 1700/11335
.....................................................................................i.............. 1800/11335
.................................................................................................... 1900/11335
---
.................................................................................................... 9100/11335
.................................................................................................... 9200/11335
.................................................................................................... 9300/11335
................i......i............................................................................ 9400/11335
........................................................iiiiiiiiiiii.i.............................. 9500/11335
.................................................................................................... 9700/11335
.................................................................................................... 9800/11335
.................................................................................................... 9900/11335
.................................................................................................... 10000/11335
---

---- [ui] ui/closures/2229_closure_analysis/migrations/insignificant_drop.rs stdout ----
diff of stderr:

- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
3    |
4 LL |       let c = || {

18    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
18    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
19    = note: drop(&(t, t1, t2));
20 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
23    |
24 LL |       let c = || {

33    |
33    |
34    = note: drop(&(t, t1));
35 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
38    |
39 LL |       let c = || {

47    |
47    |
48    = note: drop(&(t));
49 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
52    |
53 LL |       let c = || {

61    |
61    |
62    = note: drop(&(t));
63 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
66    |
67 LL |       let c = || {

75    |
75    |
76    = note: drop(&(t));
77 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
80    |
80    |
81 LL |       let c = move || {
88    |
88    |
89    = note: drop(&(t1, t));
90 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
93    |
94 LL |       let c = || {



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop/insignificant_drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/insignificant_drop.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: drop order affected for closure because of `capture_disjoint_fields`
   |
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t, t1, t2));
LL | |         let _t = t.0;
LL | |         let _t1 = t1.0;
LL | |         let _t2 = t2.0;
LL | |     };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/insignificant_drop.rs:1:9
   |
   |
LL | #![deny(disjoint_capture_drop_reorder)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: drop(&(t, t1, t2));

error: drop order affected for closure because of `capture_disjoint_fields`
   |
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t, t1));
LL | |         let _t = t.0;
LL | |         let _t1 = t1.0;
LL | |         let _t2 = t2;
LL | |     };
   |
   |
   = note: drop(&(t, t1));

error: drop order affected for closure because of `capture_disjoint_fields`
   |
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |         println!("{}", t1.1);
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
   |
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |         let _t1 = t1.0;
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
   |
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |         let _s = s.0;
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
   |
   |
LL |       let c = move || {
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t1, t));
LL | |         println!("{} {}", t1.1, t.1);
LL | |     };
   |
   |
   = note: drop(&(t1, t));

error: drop order affected for closure because of `capture_disjoint_fields`
   |
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |     };
   |
   |
   = note: drop(&(t));
error: aborting due to 7 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/closures/2229_closure_analysis/migrations/significant_drop.rs stdout ----


- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
3    |
4 LL |       let c = || {

18    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
18    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
19    = note: drop(&(t, t1, t2));
20 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
23    |
24 LL |       let c = || {

33    |
33    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
34    = note: drop(&(t, t1));
35 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
38    |
39 LL |       let c = || {

47    |
47    |
48    = note: drop(&(t));
49 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
52    |
53 LL |       let c = || {

60    |
60    |
61    = note: drop(&(t));
62 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
65    |
66 LL |       let c = || {

73    |
73    |
74    = note: drop(&(t));
75 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
78    |
79 LL |       let c = || {

86    |
86    |
87    = note: drop(&(t));
88 
- error: Drop order affected for closure because of `capture_disjoint_fields`
+ error: drop order affected for closure because of `capture_disjoint_fields`
91    |
91    |
92 LL |       let c = move || {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/significant_drop/significant_drop.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/significant_drop/significant_drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/2229_closure_analysis/migrations/significant_drop.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/significant_drop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/migrations/significant_drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:24:13
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t, t1, t2));
LL | |         let _t = t.0;
LL | |         let _t1 = t1.0;
LL | |         let _t2 = t2.0;
LL | |     };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:1:9
   |
   |
LL | #![deny(disjoint_capture_drop_reorder)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: drop(&(t, t1, t2));

error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:42:13
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t, t1));
LL | |         let _t = t.0;
LL | |         let _t1 = t1.0;
LL | |         let _t2 = t2;
LL | |     };
   |
   |
   = note: drop(&(t, t1));

error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:58:13
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |         println!("{:?}", t1.1);
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:75:13
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:90:13
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.0;
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:105:13
LL |       let c = || {
   |  _____________^
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t));
LL | |         let _t = t.1;
LL | |     };
   |
   |
   = note: drop(&(t));

error: drop order affected for closure because of `capture_disjoint_fields`
  --> /checkout/src/test/ui/closures/2229_closure_analysis/migrations/significant_drop.rs:120:13
   |
LL |       let c = move || {
   |  _____________^
LL | |     //~^ERROR: Drop order affected for closure because of `capture_disjoint_fields`
LL | |     //~| NOTE: drop(&(t1, t));
LL | |         println!("{:?} {:?}", t1.1, t.1);
LL | |     };
   |
   |
   = note: drop(&(t1, t));
error: aborting due to 7 previous errors


------------------------------------------
---
test result: FAILED. 11245 passed; 2 failed; 88 ignored; 0 measured; 0 filtered out; finished in 136.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:45
