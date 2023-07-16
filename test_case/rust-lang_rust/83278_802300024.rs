plain
.................................................................................................... 2000/11700
.................................................................................................... 2100/11700
.................................................................................................... 2200/11700
.................................................................................................... 2300/11700
.........................................................................................F...F.F.... 2400/11700
.................................................................................................... 2600/11700
.........................................................................i..i....................... 2700/11700
.................................................................................................... 2800/11700
.................................................................................iiiii.............. 2900/11700
---
.................................................................................................... 9300/11700
.................................................................................................... 9400/11700
.................................................................................................... 9500/11700
...........................................i......i................................................. 9600/11700
.........................................................................................iiiiiii..ii 9700/11700
.................................................................................................... 9900/11700
.................................................................................................... 10000/11700
.................................................................................................... 10100/11700
.................................................................................................... 10200/11700
---
-   --> $DIR/const_arg_local.rs:10:5
+ error[E0435]: attempt to use a non-constant value in a constant
+   --> $DIR/const_arg_local.rs:10:32
3    |
+ LL |     let imm8 = 3;
+    |     -------- help: consider using `const` instead of `let`: `const imm8`
4 LL |     _mm_clmulepi64_si128(a, b, imm8)
+    |                                ^^^^ non-constant value
6 
7 error: aborting due to previous error
8 
8 

+ For more information about this error, try `rustc --explain E0435`.
9 


The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local/const_arg_local.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_arg_local.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_local.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_local/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/consts/const_arg_local.rs:10:32
   |
LL |     let imm8 = 3;
   |     -------- help: consider using `const` instead of `let`: `const imm8`
LL |     _mm_clmulepi64_si128(a, b, imm8) //~ ERROR argument 3 is required to be a constant
   |                                ^^^^ non-constant value
error: aborting due to previous error

For more information about this error, try `rustc --explain E0435`.

---
-   --> $DIR/const_arg_promotable.rs:9:5
+ error[E0658]: mutable references are not allowed in constants
+   --> $DIR/const_arg_promotable.rs:9:33
3    |
4 LL |     _mm_clmulepi64_si128(a, b, *&mut 42)
+    |                                 ^^^^^^^
+    |
+    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
+    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
---
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable/const_arg_promotable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_arg_promotable.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_promotable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_promotable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: mutable references are not allowed in constants
  --> /checkout/src/test/ui/consts/const_arg_promotable.rs:9:33
   |
LL |     _mm_clmulepi64_si128(a, b, *&mut 42) //~ ERROR argument 3 is required to be a constant
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

---
-   --> $DIR/const_arg_wrapper.rs:9:5
+ error[E0435]: attempt to use a non-constant value in a constant
+   --> $DIR/const_arg_wrapper.rs:9:32
3    |
+ LL | unsafe fn pclmul(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
+    |                                          ---- this would need to be a `const`
4 LL |     _mm_clmulepi64_si128(a, b, imm8)
+    |                                ^^^^
6 
7 error: aborting due to previous error
8 
8 

+ For more information about this error, try `rustc --explain E0435`.
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper/const_arg_wrapper.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_arg_wrapper.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_arg_wrapper.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_arg_wrapper/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/consts/const_arg_wrapper.rs:9:32
   |
LL | unsafe fn pclmul(a: __m128i, b: __m128i, imm8: i32) -> __m128i {
   |                                          ---- this would need to be a `const`
LL |     _mm_clmulepi64_si128(a, b, imm8) //~ ERROR argument 3 is required to be a constant

error: aborting due to previous error

For more information about this error, try `rustc --explain E0435`.
---
test result: FAILED. 11602 passed; 3 failed; 95 ignored; 0 measured; 0 filtered out; finished in 132.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:01
