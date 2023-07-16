plain

---- [ui] ui/const-ptr/out_of_bounds_read.rs stdout ----
diff of stderr:

4 LL |     unsafe { copy_nonoverlapping(src, dst, count) }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6    |              |
6    |              |
-    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
+    |              memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of alloc6 which has size 4
8    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
9    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
10    |              inside `_READ` at $DIR/out_of_bounds_read.rs:13:33

22 LL |     unsafe { copy_nonoverlapping(src, dst, count) }
24    |              |
24    |              |
-    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
+    |              memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of alloc6 which has size 4
26    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
27    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
28    |              inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL

39 LL |     unsafe { copy_nonoverlapping(src, dst, count) }
41    |              |
41    |              |
-    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
+    |              memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of alloc6 which has size 4
43    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
44    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
45    |              inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/out_of_bounds_read.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/out_of_bounds_read.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
   |              |
   |              |
   |              memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of alloc6 which has size 4
   |              inside `copy_nonoverlapping::<u32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   |              inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:698:9
   |              inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:33
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:5
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
   |
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
   |
   |
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
   |              |
   |              |
   |              memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of alloc6 which has size 4
   |              inside `copy_nonoverlapping::<u32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   |              inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:698:9
   |              inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:755:18
   |              inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:39
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:5
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };


error: any use of this value will cause an error
   |
   |
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
   |              |
   |              |
   |              memory access failed: pointer must be in-bounds at offset 12, but is outside bounds of alloc6 which has size 4
   |              inside `copy_nonoverlapping::<u32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   |              inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:698:9
   |              inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:862:18
   |              inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:37
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:5
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };

error: aborting due to 3 previous errors


---
test result: FAILED. 11194 passed; 1 failed; 87 ignored; 0 measured; 0 filtered out; finished in 133.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:18
