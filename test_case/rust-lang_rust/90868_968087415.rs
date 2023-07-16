plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2021-11-13 14:16:55.476699552 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2021-11-13 14:57:42.112525306 +0000
@@ -4,6 +4,7 @@
     let mut _0: ();                      // return place in scope 0 at main.rs:8:11: 8:11
     let _1: i32;                         // in scope 0 at main.rs:9:5: 9:10
+    // preds: []
     bb0: {
     bb0: {
         _1 = foo() -> bb1;               // scope 0 at main.rs:9:5: 9:10
                                          // mir::Constant
@@ -11,6 +12,7 @@
                                          // + literal: Const { ty: fn() -> i32 {foo}, val: Value(Scalar(<ZST>)) }
 
 
+    // preds: [bb0]
     bb1: {
         return;                          // scope 0 at main.rs:10:2: 10:2
     }
@@ -19,6 +21,7 @@
 fn foo() -> i32 {
     let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
+    // preds: []
     bb0: {
     bb0: {
         _0 = const 11_i32;               // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
@@ -30,11 +33,13 @@
     let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
     let mut _1: (i32, bool);             // in scope 0 at main.rs:5:5: 5:10
+    // preds: []
     bb0: {
     bb0: {
         _1 = CheckedAdd(const 5_i32, const 6_i32); // scope 0 at main.rs:5:5: 5:10
         assert(!move (_1.1: bool), "attempt to compute `{} + {}`, which would overflow", const 5_i32, const 6_i32) -> bb1; // scope 0 at main.rs:5:5: 5:10
 
 
+    // preds: [bb0]
     bb1: {
         _0 = move (_1.0: i32);           // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
------------------------------------------
stderr:
------------------------------------------
warning: ignoring --out-dir flag due to -o flag
warning: ignoring --out-dir flag due to -o flag

warning: 1 warning emitted

make: *** [Makefile:5: all] Error 1
------------------------------------------




failures:
    [run-make] run-make/const_fn_mir

test result: FAILED. 25 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 5.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/run-make" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-12/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:38:08
