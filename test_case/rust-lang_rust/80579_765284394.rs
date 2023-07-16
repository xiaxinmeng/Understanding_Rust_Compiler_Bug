plain
.......................................i............................................................ 1800/11277
.................................................................................................... 1900/11277
.................................................................................................... 2000/11277
.................................................................................................... 2100/11277
..........................F...FF.................................................................... 2200/11277
.................................................................................................... 2400/11277
.................................................................................................... 2500/11277
..................i..i.............................................................................. 2600/11277
.................................................................................................... 2700/11277
---
.................................................................................................... 9100/11277
.................................................................................................... 9200/11277
.........................................................................i.......i.................. 9300/11277
.................................................................................................... 9400/11277
...........iiiiii...iiiiiii......................................................................... 9500/11277
.................................................................................................... 9700/11277
.................................................................................................... 9800/11277
.................................................................................................... 9900/11277
.................................................................................................... 10000/11277
---

---- [ui] ui/consts/const-eval/promoted_errors.rs#noopt stdout ----
diff of stderr:

- warning: this arithmetic operation will overflow
-   --> $DIR/promoted_errors.rs:13:5
- LL |     0 - 1
- LL |     0 - 1
-    |     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow
- note: the lint level is defined here
-   --> $DIR/promoted_errors.rs:9:20
-    |
-    |
- LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
- 
- 
13 warning: any use of this value will cause an error
15    |

48 LL | | };
49    | |__-
---
53 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/promoted_errors.noopt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`

error in revision `noopt`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "noopt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.noopt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL |       0 - 1 //~WARN arithmetic_overflow
   |       |
   |       |
   |       attempt to compute `0_u32 - 1_u32`, which would overflow
   |       inside `overflow` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:13:5
   |       inside `X` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:31:29
...
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     let _x: &'static i32 = &div_by_zero1();
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:9
   |
   |
LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]


warning: any use of this value will cause an error
   |
   |
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
   | |                            ^^^^^^^^^^^ referenced constant has errors
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     let _x: &'static i32 = &div_by_zero1();
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };

warning: 2 warnings emitted



------------------------------------------


---- [ui] ui/consts/const-eval/promoted_errors.rs#opt_with_overflow_checks stdout ----
diff of stderr:

- warning: this arithmetic operation will overflow
-   --> $DIR/promoted_errors.rs:13:5
- LL |     0 - 1
- LL |     0 - 1
-    |     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow
- note: the lint level is defined here
-   --> $DIR/promoted_errors.rs:9:20
-    |
-    |
- LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
- 
- 
13 warning: any use of this value will cause an error
15    |

48 LL | | };
49    | |__-
---
53 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/promoted_errors.opt_with_overflow_checks.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`

error in revision `opt_with_overflow_checks`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt_with_overflow_checks" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=on" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt_with_overflow_checks/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL |       0 - 1 //~WARN arithmetic_overflow
   |       |
   |       |
   |       attempt to compute `0_u32 - 1_u32`, which would overflow
   |       inside `overflow` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:13:5
   |       inside `X` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:31:29
...
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     let _x: &'static i32 = &div_by_zero1();
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:9
   |
   |
LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]


warning: any use of this value will cause an error
   |
   |
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
   | |                            ^^^^^^^^^^^ referenced constant has errors
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     let _x: &'static i32 = &div_by_zero1();
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };

warning: 2 warnings emitted



------------------------------------------


---- [ui] ui/consts/const-eval/promoted_errors.rs#opt stdout ----
diff of stderr:

- warning: this arithmetic operation will overflow
-   --> $DIR/promoted_errors.rs:13:5
- LL |     0 - 1
- LL |     0 - 1
-    |     ^^^^^ attempt to compute `0_u32 - 1_u32`, which would overflow
- note: the lint level is defined here
-   --> $DIR/promoted_errors.rs:9:20
-    |
-    |
- LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
- 
- warning: this operation will panic at runtime
-   --> $DIR/promoted_errors.rs:17:5
-    |
-    |
- LL |     1 / 0
-    |     ^^^^^ attempt to divide `1_i32` by zero
- note: the lint level is defined here
-   --> $DIR/promoted_errors.rs:9:41
-    |
-    |
- LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]
- 
- 
25 warning: any use of this value will cause an error
27    |

60 LL | | };
61    | |__-
---
65 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/promoted_errors.opt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_errors.rs`

error in revision `opt`: 1 errors occurred comparing output.
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "opt" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_errors.opt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
   |
   |
LL |       1 / 0 //[opt]~WARN unconditional_panic
   |       |
   |       |
   |       attempt to divide `1_i32` by zero
   |       inside `div_by_zero1` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:17:5
   |       inside `X` at /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:33:29
...
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     let _x: &'static i32 = &div_by_zero1();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/promoted_errors.rs:9:9
   |
   |
LL | #![warn(const_err, arithmetic_overflow, unconditional_panic)]


warning: any use of this value will cause an error
   |
   |
LL | / const X: () = {
LL | |     let _x: &'static u32 = &overflow();
LL | |     //[opt_with_overflow_checks,noopt]~^ WARN any use of this value will cause an error
LL | |     let _x: &'static i32 = &div_by_zero1();
   | |                            ^^^^^^^^^^^^^^^ referenced constant has errors
...  |
LL | |     let _x: &'static i32 = &oob();
LL | | };

warning: 2 warnings emitted


---
test result: FAILED. 11187 passed; 3 failed; 87 ignored; 0 measured; 0 filtered out; finished in 113.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:09
