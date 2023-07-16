plain
.........i.i..ii.................................................................................... 7100/11198
.................................................................................................... 7200/11198
.................................................................................................... 7300/11198
.................................................................................................... 7400/11198
...............................F.i..ii...........................................................ii. 7500/11198
..................i................................................................................. 7700/11198
...........................................................................................i........ 7800/11198
.................................................................................................... 7900/11198
.................................................................................................... 8000/11198
---
...................................................F................................................ 9000/11198
.................................................................................................... 9100/11198
........................................................................................i......i.... 9200/11198
.................................................................................................... 9300/11198
...........................iiiiii..iiiiii.i......................................................... 9400/11198
.................................................................................................... 9600/11198
.................................................................................................... 9700/11198
.................................................................................................... 9800/11198
.................................................................................................... 9900/11198
---
diff of stderr:

2   --> $DIR/E0435.rs:3:17
3    |
4 LL |     let _: [u8; foo];
-    |                 ^^^ non-constant value
+    |                 |
+    |                 non-constant value
+    |                 help: consider using `const` instead of `let`
6 
6 
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0435/E0435.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0435.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0435.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0435" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0435/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/error-codes/E0435.rs:3:17
   |
LL |     let _: [u8; foo]; //~ ERROR E0435
   |                 |
   |                 non-constant value
   |                 help: consider using `const` instead of `let`

---
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3521/issue-3521.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-3521.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3521.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3521" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3521/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
diff of stderr:

2   --> $DIR/issue-42060.rs:3:23
3    |
4 LL |     let other: typeof(thing) = thing;
-    |                       ^^^^^ non-constant value
+    |                       |
+    |                       non-constant value
+    |                       help: consider using `const` instead of `let`
6 
6 
7 error[E0435]: attempt to use a non-constant value in a constant
8   --> $DIR/issue-42060.rs:9:13

9    |
10 LL |     <typeof(q)>::N
-    |             ^ non-constant value
+    |             |
+    |             non-constant value
+    |             help: consider using `const` instead of `let`
12 
12 
13 error[E0516]: `typeof` is a reserved keyword but unimplemented


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/issue-42060.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/issue-42060.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-42060.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-42060.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-42060/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/issues/issue-42060.rs:3:23
   |
LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
   |                       |
   |                       non-constant value
   |                       help: consider using `const` instead of `let`


error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/issues/issue-42060.rs:9:13
   |
LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
   |             |
   |             non-constant value
   |             help: consider using `const` instead of `let`


error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let other: typeof(thing) = thing; //~ ERROR attempt to use a non-constant value in a constant
   |                ^^^^^^^^^^^^^ reserved keyword

error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     <typeof(q)>::N //~ ERROR attempt to use a non-constant value in a constant
   |      ^^^^^^^^^ reserved keyword
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0435, E0516.
For more information about an error, try `rustc --explain E0435`.
---
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44239/issue-44239.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-44239.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-44239.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44239" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-44239/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
diff of stderr:

2   --> $DIR/non-constant-expr-for-arr-len.rs:5:22
3    |
4 LL |         let _x = [0; n];
-    |                      ^ non-constant value
+    |                      |
+    |                      non-constant value
+    |                      help: consider using `const` instead of `let`
6 
6 
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-constant-expr-for-arr-len/non-constant-expr-for-arr-len.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args non-constant-expr-for-arr-len.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/non-constant-expr-for-arr-len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-constant-expr-for-arr-len" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/non-constant-expr-for-arr-len/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/non-constant-expr-for-arr-len.rs:5:22
   |
LL |         let _x = [0; n];
   |                      |
   |                      non-constant value
   |                      help: consider using `const` instead of `let`

---
diff of stderr:

2   --> $DIR/repeat_count.rs:5:17
3    |
4 LL |     let a = [0; n];
-    |                 ^ non-constant value
+    |                 |
+    |                 non-constant value
+    |                 help: consider using `const` instead of `let`
6 
6 
7 error[E0308]: mismatched types
8   --> $DIR/repeat_count.rs:7:17


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count/repeat_count.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args repeat_count.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/repeat_count.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/repeat_count/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/repeat_count.rs:5:17
   |
LL |     let a = [0; n];
   |                 |
   |                 non-constant value
   |                 help: consider using `const` instead of `let`


error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:7:17
   |
LL |     let b = [0; ()];
   |                 ^^ expected `usize`, found `()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:10:17
   |
   |
LL |     let c = [0; true];
   |                 ^^^^ expected `usize`, found `bool`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:13:17
   |
   |
LL |     let d = [0; 0.5];
   |                 ^^^ expected `usize`, found floating-point number
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:16:17
   |
   |
LL |     let e = [0; "foo"];
   |                 ^^^^^ expected `usize`, found `&str`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:31:17
   |
   |
LL |     let g = [0; G { g: () }];
   |                 ^^^^^^^^^^^ expected `usize`, found struct `G`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:19:17
   |
   |
LL |     let f = [0; -4_isize];
   |                 ^^^^^^^^ expected `usize`, found `isize`
   |
   = note: `-4_isize` cannot fit into type `usize`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/repeat_count.rs:22:23
   |
   |
LL |     let f = [0_usize; -1_isize];
   |                       ^^^^^^^^ expected `usize`, found `isize`
   |
   = note: `-1_isize` cannot fit into type `usize`
error[E0308]: mismatched types
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  --> /checkout/src/test/ui/repeat_count.rs:25:17
   |
   |
LL |     let f = [0; 4u8];
   |                 ^^^ expected `usize`, found `u8`
   |
help: change the type of the numeric literal from `u8` to `usize`
   |
LL |     let f = [0; 4usize];

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0308, E0435.
---
diff of stderr:

2   --> $DIR/type-dependent-def-issue-49241.rs:3:22
3    |
4 LL |     const l: usize = v.count();
-    |           - constant ^ non-constant value
+    |           -          ^ non-constant value
+    |           |
+    |           help: consider using `let` instead of `const`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/type-dependent-def-issue-49241.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type/type-dependent-def-issue-49241.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-dependent-def-issue-49241.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-dependent-def-issue-49241/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0435]: attempt to use a non-constant value in a constant
  --> /checkout/src/test/ui/type/type-dependent-def-issue-49241.rs:3:22
   |
LL |     const l: usize = v.count(); //~ ERROR attempt to use a non-constant value in a constant
   |           -          ^ non-constant value
   |           |
   |           help: consider using `let` instead of `const`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0435`.

---
test result: FAILED. 11107 passed; 7 failed; 84 ignored; 0 measured; 0 filtered out; finished in 127.12s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:14
