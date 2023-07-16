plain
.................................................................................................... 1500/12215
......................i.....i................i...................................................... 1600/12215
.................................................................................................... 1700/12215
............................................i....................................................... 1800/12215
.......................................................................F.F.......................... 1900/12215
......F.............F...............................................F............................... 2000/12215
.................................................................................................... 2200/12215
.................................................................................................... 2300/12215
.................................................................................................... 2400/12215
.................................................................................................... 2500/12215
---
diff of stderr:

13    |               ^^^^^^^^^^^^ field access is not supported in generic constant
14    |
15    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
17 
18 error: aborting due to 2 previous errors
19 
19 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.full/array-size-in-generic-struct-param.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.full/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs:8:38
   |
LL | struct ArithArrayLen<const N: usize>([u32; 0 + N]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); 0 + N]:`

error: overly complex generic constant
   |
   |
LL |     arr: [u8; CFG.arr_size],
   |               ^^^^^^^^^^^^ field access is not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 2 previous errors


---
diff of stderr:

7    |                                       references are not supported in generic constant
8    |
9    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
11 
12 error: aborting due to previous error
13 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/closures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/closures/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: overly complex generic constant
   |
   |
LL | fn test<const N: usize>() -> [u8; N + (|| 42)()] {}
   |                                       |
   |                                       references are not supported in generic constant
   |
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error



------------------------------------------


---- [ui] ui/const-generics/generic_const_exprs/let-bindings.rs stdout ----
diff of stderr:

5    |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
6    |
7    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
9 
9 
10 error: overly complex generic constant


14    |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
15    |
16    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
18 
19 error: aborting due to 2 previous errors
20 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/let-bindings.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/let-bindings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/let-bindings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: overly complex generic constant
   |
   |
LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
   |                                                                    ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function


error: overly complex generic constant
   |
   |
LL | fn test<const N: usize>() -> [u8; { let x = N; N + 1 }] where [u8; { let x = N; N + 1 }]: Default {
   |                                   ^^^^^^^^^^^^^^^^^^^^ blocks are not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/const-generics/generic_const_exprs/unused_expr.rs stdout ----
diff of stderr:

5    |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
6    |
7    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
9 
9 
10 error: overly complex generic constant
11   --> $DIR/unused_expr.rs:9:34

14    |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
15    |
16    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
18 
18 
19 error: overly complex generic constant
20   --> $DIR/unused_expr.rs:16:38

23    |                                      ^^^^^^^^^^^^^ blocks are not supported in generic constant
24    |
25    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
27 
28 error: aborting due to 3 previous errors
29 
29 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr/unused_expr.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/unused_expr.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/unused_expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/unused_expr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: overly complex generic constant
   |
   |
LL | fn add<const N: usize>() -> [u8; { N + 1; 5 }] {
   |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function


error: overly complex generic constant
   |
   |
LL | fn div<const N: usize>() -> [u8; { N / 1; 5 }] {
   |                                  ^^^^^^^^^^^^ blocks are not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function


error: overly complex generic constant
   |
   |
LL | fn fn_call<const N: usize>() -> [u8; { foo(N); 5 }] {
   |                                      ^^^^^^^^^^^^^ blocks are not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/const-generics/issues/issue-67945-2.rs#full stdout ----
diff of stderr:

11    | |_____^ blocks are not supported in generic constant
12    |
13    = help: consider moving this anonymous constant into a `const` function
+    = note: this operation may be supported in the future
15 
16 error: aborting due to previous error
17 
17 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full/issue-67945-2.full.stderr
To only update this specific test, also pass `--test-args const-generics/issues/issue-67945-2.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67945-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67945-2.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: overly complex generic constant
   |
   |
LL |       A: [(); {
   |  _____________^
LL | |         //[full]~^ ERROR overly complex generic constant
LL | |         let x: Option<Box<Self>> = None;
LL | |         //[min]~^ ERROR generic `Self` types are currently not permitted in anonymous constants
LL | |         0
LL | |     }],
   | |_____^ blocks are not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to previous error


---
test result: FAILED. 12095 passed; 5 failed; 115 ignored; 0 measured; 0 filtered out; finished in 133.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:23
