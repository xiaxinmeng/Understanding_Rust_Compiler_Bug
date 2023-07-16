plain
.................................................................................................... 9300/11706
.................................................................................................... 9400/11706
.................................................................................................... 9500/11706
...............................................i......i............................................. 9600/11706
.............................................................................................iiiiiii 9700/11706
..iiiiii.i.......................................................................................... 9800/11706
.................................................................................................... 10000/11706
.................................................................................................... 10100/11706
.................................................................................................... 10200/11706
.................................................................................................... 10300/11706
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 31 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.116 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.33s

 finished in 2.406 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 22 tests
Some tests failed in compiletest suite=run-make mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
iiiiiiiiiiii.F........

---- [run-make] run-make/const_fn_mir stdout ----

error: make failed
error: make failed
status: exit code: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2021-03-21 23:01:13.940226871 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2021-03-21 23:34:25.387132655 +0000
@@ -1,42 +1,42 @@
 // WARNING: This output format is intended for human consumers only
 // and is subject to change without notice. Knock yourself out.
 fn main() -> () {
-    let mut _0: ();                      // return place in scope 0 at main.rs:8:11: 8:11
-    let _1: i32;                         // in scope 0 at main.rs:9:5: 9:10
+    let mut _0: ();                      // return place in scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:8:11: 8:11
+    let _1: i32;                         // in scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:9:5: 9:10
     bb0: {
     bb0: {
-        _1 = foo() -> bb1;               // scope 0 at main.rs:9:5: 9:10
+        _1 = foo() -> bb1;               // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:9:5: 9:10
                                          // mir::Constant
-                                         // + span: main.rs:9:5: 9:8
+                                         // + span: /checkout/src/test/run-make/const_fn_mir/main.rs:9:5: 9:8
                                          // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(<ZST>)) }
 
     bb1: {
     bb1: {
-        return;                          // scope 0 at main.rs:10:2: 10:2
+        return;                          // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:10:2: 10:2
 }
 
 fn foo() -> i32 {
 fn foo() -> i32 {
-    let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
+    let mut _0: i32;                     // return place in scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:4:19: 4:22
     bb0: {
     bb0: {
-        _0 = const 11_i32;               // scope 0 at main.rs:5:5: 5:10
-        return;                          // scope 0 at main.rs:6:2: 6:2
+        _0 = const 11_i32;               // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:5:5: 5:10
+        return;                          // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:6:2: 6:2
 }
 
 
 // MIR FOR CTFE
 fn foo() -> i32 {
-    let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
-    let mut _1: (i32, bool);             // in scope 0 at main.rs:5:5: 5:10
+    let mut _0: i32;                     // return place in scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:4:19: 4:22
+    let mut _1: (i32, bool);             // in scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:5:5: 5:10
     bb0: {
     bb0: {
-        _1 = CheckedAdd(const 5_i32, const 6_i32); // scope 0 at main.rs:5:5: 5:10
-        assert(!move (_1.1: bool), "attempt to compute `{} + {}`, which would overflow", const 5_i32, const 6_i32) -> bb1; // scope 0 at main.rs:5:5: 5:10
+        _1 = CheckedAdd(const 5_i32, const 6_i32); // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:5:5: 5:10
+        assert(!move (_1.1: bool), "attempt to compute `{} + {}`, which would overflow", const 5_i32, const 6_i32) -> bb1; // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:5:5: 5:10
 
     bb1: {
     bb1: {
-        _0 = move (_1.0: i32);           // scope 0 at main.rs:5:5: 5:10
-        return;                          // scope 0 at main.rs:6:2: 6:2
+        _0 = move (_1.0: i32);           // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:5:5: 5:10
+        return;                          // scope 0 at /checkout/src/test/run-make/const_fn_mir/main.rs:6:2: 6:2
 }
Makefile:4: recipe for target 'all' failed

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

make: *** [all] Error 1
------------------------------------------




failures:
    [run-make] run-make/const_fn_mir

test result: FAILED. 9 passed; 1 failed; 12 ignored; 0 measured; 0 filtered out; finished in 0.55s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-9/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:30:39
