plain
........................................................................................ii.i........ 1300/11228
.....i.............................................................................................. 1400/11228
.................................................................................................... 1500/11228
.................................................................................................... 1600/11228
...............F.FF...F..F.......................................................................... 1700/11228
.................................................................................................... 1900/11228
.................................................................................................... 2000/11228
.................................................................................................... 2100/11228
.................................................................................................... 2200/11228
---
.................................................................................................... 9000/11228
.................................................................................................... 9100/11228
.................................................................................................... 9200/11228
...............i......i............................................................................. 9300/11228
......................................................iiiiii..iiiiii.i.............................. 9400/11228
.................................................................................................... 9600/11228
.................................................................................................... 9700/11228
.................................................................................................... 9800/11228
.................................................................................................... 9900/11228
---
diff of stderr:

2   --> $DIR/complex-unord-param.rs:8:41
3    |
4 LL | struct NestedArrays<'a, const N: usize, A: 'a, const M: usize, T:'a =u32> {
-    |                    ---------------------^----------------------^--------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, A: 'a, T: 'a, const N: usize, const M: usize>`
+    |                    ---------------------^----------------------^--------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, A: 'a, T = u32: 'a, const N: usize, const M: usize>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/complex-unord-param.min/complex-unord-param.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/complex-unord-param.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/complex-unord-param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/complex-unord-param.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/complex-unord-param.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type parameters must be declared prior to const parameters
  --> /checkout/src/test/ui/const-generics/defaults/complex-unord-param.rs:8:41
   |
LL | struct NestedArrays<'a, const N: usize, A: 'a, const M: usize, T:'a =u32> {
   |                    ---------------------^----------------------^--------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, A: 'a, T = u32: 'a, const N: usize, const M: usize>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/defaults/intermixed-lifetime.rs#min stdout ----
diff of stderr:

2   --> $DIR/intermixed-lifetime.rs:6:28
3    |
4 LL | struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
-    |           -----------------^^---------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, const N: usize>`
+    |           -----------------^^---------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
7 error: type parameters must be declared prior to const parameters
8   --> $DIR/intermixed-lifetime.rs:6:32

9    |
9    |
10 LL | struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
-    |           ---------------------^------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, const N: usize>`
+    |           ---------------------^------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
12 
13 error: lifetime parameters must be declared prior to const parameters

15    |
15    |
16 LL | struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
-    |           --------------------------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, const N: usize>`
+    |           --------------------------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
19 error: type parameters must be declared prior to const parameters
20   --> $DIR/intermixed-lifetime.rs:10:28

21    |
21    |
22 LL | struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
-    |           -----------------^----------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, const N: usize>`
+    |           -----------------^----------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
25 error: aborting due to 4 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/intermixed-lifetime.min/intermixed-lifetime.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/intermixed-lifetime.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/intermixed-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/intermixed-lifetime.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/intermixed-lifetime.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to const parameters
   |
   |
LL | struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
   |           -----------------^^---------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
error: type parameters must be declared prior to const parameters
  --> /checkout/src/test/ui/const-generics/defaults/intermixed-lifetime.rs:6:32
   |
   |
LL | struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
   |           ---------------------^------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`

error: lifetime parameters must be declared prior to const parameters
   |
   |
LL | struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
   |           --------------------------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
error: type parameters must be declared prior to const parameters
  --> /checkout/src/test/ui/const-generics/defaults/intermixed-lifetime.rs:10:28
   |
   |
LL | struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
   |           -----------------^----------- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
error: aborting due to 4 previous errors
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


---
diff of stderr:

2   --> $DIR/intermixed-lifetime.rs:6:28
3    |
4 LL | struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
-    |           -----------------^^---------- help: reorder the parameters: lifetimes, then consts and types: `<'a, const N: usize, T>`
+    |           -----------------^^---------- help: reorder the parameters: lifetimes, then consts and types: `<'a, const N: usize, T = u32>`
6 
7 error: lifetime parameters must be declared prior to type parameters

9    |
9    |
10 LL | struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
-    |           --------------------------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, const N: usize, T>`
+    |           --------------------------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, const N: usize, T = u32>`
13 error: aborting due to 2 previous errors
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/intermixed-lifetime.full/intermixed-lifetime.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/intermixed-lifetime.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/intermixed-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/intermixed-lifetime.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/intermixed-lifetime.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to const parameters
   |
   |
LL | struct Foo<const N: usize, 'a, T = u32>(&'a (), T);
   |           -----------------^^---------- help: reorder the parameters: lifetimes, then consts and types: `<'a, const N: usize, T = u32>`

error: lifetime parameters must be declared prior to type parameters
   |
   |
LL | struct Bar<const N: usize, T = u32, 'a>(&'a (), T);
   |           --------------------------^^- help: reorder the parameters: lifetimes, then consts and types: `<'a, const N: usize, T = u32>`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/defaults/needs-feature.rs#min stdout ----
diff of stderr:

2   --> $DIR/needs-feature.rs:9:26
3    |
4 LL | struct A<const N: usize, T=u32>(T);
-    |         -----------------^----- help: reorder the parameters: lifetimes, then types, then consts: `<T, const N: usize>`
+    |         -----------------^----- help: reorder the parameters: lifetimes, then types, then consts: `<T = u32, const N: usize>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/needs-feature.min/needs-feature.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/needs-feature.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/needs-feature.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/needs-feature.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/needs-feature.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type parameters must be declared prior to const parameters
  --> /checkout/src/test/ui/const-generics/defaults/needs-feature.rs:9:26
   |
LL | struct A<const N: usize, T=u32>(T);
   |         -----------------^----- help: reorder the parameters: lifetimes, then types, then consts: `<T = u32, const N: usize>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/defaults/simple-defaults.rs#min stdout ----
diff of stderr:

2   --> $DIR/simple-defaults.rs:8:40
3    |
4 LL | struct FixedOutput<'a, const N: usize, T=u32> {
-    |                   ---------------------^----- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T, const N: usize>`
+    |                   ---------------------^----- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/simple-defaults.min/simple-defaults.min.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/defaults/simple-defaults.rs`

error in revision `min`: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/simple-defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/simple-defaults.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/simple-defaults.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type parameters must be declared prior to const parameters
  --> /checkout/src/test/ui/const-generics/defaults/simple-defaults.rs:8:40
   |
LL | struct FixedOutput<'a, const N: usize, T=u32> {
   |                   ---------------------^----- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = u32, const N: usize>`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-80512-param-reordering-with-defaults.rs stdout ----

2   --> $DIR/issue-80512-param-reordering-with-defaults.rs:3:18
3    |
3    |
4 LL | struct S<T = (), 'a>(&'a T);
-    |         ---------^^- help: reorder the parameters: lifetimes, then types: `<'a, T = ()>`
+    |         ---------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = ()>`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-80512-param-reordering-with-defaults/issue-80512-param-reordering-with-defaults.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-80512-param-reordering-with-defaults.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-80512-param-reordering-with-defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-80512-param-reordering-with-defaults" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-80512-param-reordering-with-defaults/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime parameters must be declared prior to type parameters
  --> /checkout/src/test/ui/issues/issue-80512-param-reordering-with-defaults.rs:3:18
   |
LL | struct S<T = (), 'a>(&'a T);
   |         ---------^^- help: reorder the parameters: lifetimes, then types, then consts: `<'a, T = ()>`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11137 passed; 6 failed; 85 ignored; 0 measured; 0 filtered out; finished in 140.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:05
