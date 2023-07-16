plain
4 LL |     unsafe { copy_nonoverlapping(src, dst, count) }

6    |              |
7    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
8    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |              inside `_READ` at $DIR/out_of_bounds_read.rs:13:33
-   ::: $DIR/out_of_bounds_read.rs:13:5
+   ::: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
13    |
13    |
- LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
-    |     ------------------------------------------------------
+ LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
+    |         --------------------------------------------- inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+   ::: $DIR/out_of_bounds_read.rs:13:33
16    |
16    |
-    = note: `#[deny(const_err)]` on by default
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
+    |                                 ----------------------- inside `_READ` at $DIR/out_of_bounds_read.rs:13:33
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
22   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
23    |
23    |
24 LL |     unsafe { copy_nonoverlapping(src, dst, count) }

26    |              |
27    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
28    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-    |              inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |              inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:14:39
-   ::: $DIR/out_of_bounds_read.rs:14:5
+   ::: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
34    |
34    |
- LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
-    |     --------------------------------------------------------
+ LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
+    |         --------------------------------------------- inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+   ::: $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
37    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |         unsafe { read(self) }
+    |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+   ::: $DIR/out_of_bounds_read.rs:14:39
+    |
+    |
+ LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
+    |                                       ------------------- inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:14:39
- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
42   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
43    |
43    |
44 LL |     unsafe { copy_nonoverlapping(src, dst, count) }

46    |              |
47    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
48    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-    |              inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-    |              inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:15:37
-   ::: $DIR/out_of_bounds_read.rs:15:5
+   ::: $SRC_DIR/core/src/ptr/mod.rs:LL:COL
54    |
54    |
- LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
-    |     --------------------------------------------------------------------
+ LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
+    |         --------------------------------------------- inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+   ::: $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
57    |
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |         unsafe { read(self) }
+    |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+   ::: $DIR/out_of_bounds_read.rs:15:37
+    |
+    |
+ LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
+    |                                     --------------------------------- inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:15:37
61 error: aborting due to 3 previous errors
62 

+ For more information about this error, try `rustc --explain E0080`.
---
To only update this specific test, also pass `--test-args const-ptr/out_of_bounds_read.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-ptr/out_of_bounds_read.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-ptr/out_of_bounds_read/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
   |              inside `copy_nonoverlapping::<u32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   | 
  ::: /checkout/library/core/src/ptr/mod.rs:696:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         --------------------------------------------- inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:696:9
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:33
   |
   |
LL |     const _READ: u32 = unsafe { ptr::read(PAST_END_PTR) };
   |                                 ----------------------- inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:33
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/intrinsics.rs:1861:14
   |
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
   |              inside `copy_nonoverlapping::<u32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   | 
  ::: /checkout/library/core/src/ptr/mod.rs:696:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         --------------------------------------------- inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:696:9
  ::: /checkout/library/core/src/ptr/const_ptr.rs:762:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:762:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:39
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };
   |                                       ------------------- inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:39
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/intrinsics.rs:1861:14
   |
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
LL |     unsafe { copy_nonoverlapping(src, dst, count) }
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |              |
   |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
   |              inside `copy_nonoverlapping::<u32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   | 
  ::: /checkout/library/core/src/ptr/mod.rs:696:9
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         --------------------------------------------- inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:696:9
  ::: /checkout/library/core/src/ptr/mut_ptr.rs:868:18
   |
   |
LL |         unsafe { read(self) }
   |                  ---------- inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:868:18
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:37
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };
   |                                     --------------------------------- inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:37
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.

---
test result: FAILED. 11921 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 129.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:51
