plain
.................................................................................................... 4100/11494
.................................................................................................... 4200/11494
.................................................................................................... 4300/11494
...................................................................ii............................... 4400/11494
....................................................i..F............................................ 4500/11494
.................................................................................................... 4700/11494
.................................................................................................... 4800/11494
.................................................................................................... 4900/11494
.................................................................................................... 5000/11494
---
.................................................................................................... 9200/11494
.................................................................................................... 9300/11494
.................................................................................................... 9400/11494
..................................................i......i.......................................... 9500/11494
.........................................................................................iiiiiii..ii 9600/11494
.................................................................................................... 9800/11494
.................................................................................................... 9900/11494
.................................................................................................... 10000/11494
.................................................................................................... 10100/11494
---
.....................................................................................i.i............ 11400/11494
..............................................................................................
failures:

---- [ui] ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs stdout ----


+ error[E0428]: the name `foo3` is defined multiple times
+   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:30:1
+    |
+ LL | fn foo3<const X: usize>(_: u8) {}
+    | ------------------------------ previous definition of the value `foo3` here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL | fn foo3<X>() {}
+    | ^^^^^^^^^^^^ `foo3` redefined here
+    |
+    = note: `foo3` must be defined only once in the value namespace of this module
+ 
1 error: suffixed literals are not allowed in attributes
2   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:21:31


7    = help: instead of using a suffixed literal (`1u8`, `1.0f32`, etc.), use an unsuffixed version (`1`, `1.0`, etc.)
8 
9 error: malformed `rustc_legacy_const_generics` attribute input
-   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:29:1
+   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:32:1
11    |
12 LL | #[rustc_legacy_const_generics]
13    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_legacy_const_generics(N)]`
14 
14 
15 error: malformed `rustc_legacy_const_generics` attribute input
-   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:32:1
+   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:35:1
17    |
18 LL | #[rustc_legacy_const_generics = 1]
19    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_legacy_const_generics(N)]`
56 LL | struct S;
57    | --------- not a function
58 
58 
+ error: #[rustc_legacy_const_generics] functions must only have const generics
+   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:29:31
+    |
+ LL | #[rustc_legacy_const_generics(0)]
+    |                               ^
+ LL | fn foo3<X>() {}
+    |         - non-const generic parameter
+ 
59 error: index exceeds number of arguments
60   --> $DIR/invalid-rustc_legacy_const_generics-arguments.rs:25:35


62 LL |     #[rustc_legacy_const_generics(1)]
63    |                                   ^ there is only 1 argument
- error: aborting due to 10 previous errors
+ error: aborting due to 12 previous errors
66 
+ For more information about this error, try `rustc --explain E0428`.
+ For more information about this error, try `rustc --explain E0428`.
67 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments/invalid-rustc_legacy_const_generics-arguments.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args invalid/invalid-rustc_legacy_const_generics-arguments.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0428]: the name `foo3` is defined multiple times
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:30:1
   |
LL | fn foo3<const X: usize>(_: u8) {}
   | ------------------------------ previous definition of the value `foo3` here
...
LL | fn foo3<X>() {}
   | ^^^^^^^^^^^^ `foo3` redefined here
   |
   = note: `foo3` must be defined only once in the value namespace of this module

error: suffixed literals are not allowed in attributes
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:21:31
   |
LL | #[rustc_legacy_const_generics(0usize)] //~ ERROR suffixed literals are not allowed in attributes
   |
   |
   = help: instead of using a suffixed literal (`1u8`, `1.0f32`, etc.), use an unsuffixed version (`1`, `1.0`, etc.)

error: malformed `rustc_legacy_const_generics` attribute input
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:32:1
   |
LL | #[rustc_legacy_const_generics] //~ ERROR malformed `rustc_legacy_const_generics` attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_legacy_const_generics(N)]`

error: malformed `rustc_legacy_const_generics` attribute input
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:35:1
   |
LL | #[rustc_legacy_const_generics = 1] //~ ERROR malformed `rustc_legacy_const_generics` attribute
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: must be of the form: `#[rustc_legacy_const_generics(N)]`

error: index exceeds number of arguments
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:3:31
   |
LL | #[rustc_legacy_const_generics(0)] //~ ERROR index exceeds number of arguments
   |                               ^ there are only 0 arguments

error: index exceeds number of arguments
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:6:31
   |
LL | #[rustc_legacy_const_generics(1)] //~ ERROR index exceeds number of arguments
   |                               ^ there is only 1 argument

error: index exceeds number of arguments
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:9:31
   |
LL | #[rustc_legacy_const_generics(2)] //~ ERROR index exceeds number of arguments
   |                               ^ there are only 2 arguments
error: arguments should be non-negative integers
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:12:31
   |
   |
LL | #[rustc_legacy_const_generics(a)] //~ ERROR arguments should be non-negative integers

error: arguments should be non-negative integers
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:15:34
   |
   |
LL | #[rustc_legacy_const_generics(1, a, 2, b)] //~ ERROR arguments should be non-negative integers
   |                                  ^     ^
error: attribute should be applied to a function
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:18:1
   |
   |
LL | #[rustc_legacy_const_generics(0)] //~ ERROR attribute should be applied to a function
LL | struct S;
   | --------- not a function


error: #[rustc_legacy_const_generics] functions must only have const generics
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:29:31
   |
LL | #[rustc_legacy_const_generics(0)] //~ ERROR #[rustc_legacy_const_generics] functions must only have
   |                               ^
LL | fn foo3<X>() {}
   |         - non-const generic parameter

error: index exceeds number of arguments
  --> /checkout/src/test/ui/invalid/invalid-rustc_legacy_const_generics-arguments.rs:25:35
   |
LL |     #[rustc_legacy_const_generics(1)] //~ ERROR index exceeds number of arguments
   |                                   ^ there is only 1 argument
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0428`.

---
test result: FAILED. 11400 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 133.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:50
