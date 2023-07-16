plain

Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 308 tests
............i..ii.F..ii.................................................................  88/308
iiiii......i.ii.iiii..................................iii..............................i 264/308
i.i......i...iiiiiii.iii.ii.i...............

failures:
failures:

---- [run-make] tests/run-make/const_fn_mir stdout ----

error: make failed
status: exit status: 2
command: cd "/checkout/tests/run-make/const_fn_mir" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir  main.rs --emit=mir -o "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
diff -u --strip-trailing-cr dump.mir "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir"/dump.mir
--- dump.mir 2023-04-04 17:25:16.393310211 +0000
+++ /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/const_fn_mir/const_fn_mir/dump.mir 2023-04-04 17:56:20.284636882 +0000
@@ -2,9 +2,15 @@
 // and is subject to change without notice. Knock yourself out.
 fn foo() -> i32 {
     let mut _0: i32;                     // return place in scope 0 at main.rs:4:19: 4:22
+    let mut _1: (i32, bool);             // in scope 0 at main.rs:5:5: 5:10
     bb0: {
     bb0: {
-        _0 = const 11_i32;               // scope 0 at main.rs:5:5: 5:10
+        _1 = CheckedAdd(const 5_i32, const 6_i32); // scope 0 at main.rs:5:5: 5:10
+        assert(!move (_1.1: bool), "attempt to compute `{} + {}`, which would overflow", const 5_i32, const 6_i32) -> bb1; // scope 0 at main.rs:5:5: 5:10
+
+    bb1: {
+    bb1: {
+        _0 = move (_1.0: i32);           // scope 0 at main.rs:5:5: 5:10
         return;                          // scope 0 at main.rs:6:2: 6:2
 }
------------------------------------------
--- stderr -------------------------------
warning: ignoring --out-dir flag due to -o flag
