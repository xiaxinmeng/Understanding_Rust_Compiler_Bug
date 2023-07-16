plain
.................................................................................................... 9300/11703
.................................................................................................... 9400/11703
.................................................................................................... 9500/11703
..............................................i......i.............................................. 9600/11703
............................................................................................iiiiiii. 9700/11703
.iiiiii.i........................................................................................... 9800/11703
.................................................................................................... 10000/11703
.................................................................................................... 10100/11703
.................................................................................................... 10200/11703
.................................................................................................... 10300/11703
---

---- [ui] ui/consts/copy-intrinsic.rs stdout ----
diff of stderr:

1 error: any use of this value will cause an error
-   --> $DIR/copy-intrinsic.rs:16:5
+   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
3    |
+ LL |       unsafe { copy_nonoverlapping(src, dst, count) }
+    |                |
+    |                memory access failed: pointer must be in-bounds at offset 40, but is outside bounds of alloc4 which has size 4
+    |                memory access failed: pointer must be in-bounds at offset 40, but is outside bounds of alloc4 which has size 4
+    |                inside `copy_nonoverlapping::<i32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+    |                inside `COPY_OOB_1` at $DIR/copy-intrinsic.rs:16:5
+   ::: $DIR/copy-intrinsic.rs:12:1
+    |
+    |
4 LL | / const COPY_OOB_1: () = unsafe {
5 LL | |     let mut x = 0i32;
6 LL | |     let dangle = (&mut x as *mut i32).wrapping_add(10);

7 LL | |     // Even if the first ptr is an int ptr and this is a ZST copy, we should detect dangling 2nd ptrs.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | |     ptr::copy_nonoverlapping(0x100 as *const i32, dangle, 0);
+ ...  |
10 LL | |
- LL | |
12 LL | | };
12 LL | | };
13    | |__-
14    |

17    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
18 
19 error: any use of this value will cause an error
-   --> $DIR/copy-intrinsic.rs:24:5
+   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
21    |
+ LL |       unsafe { copy_nonoverlapping(src, dst, count) }
+    |                |
+    |                memory access failed: pointer must be in-bounds at offset 40, but is outside bounds of alloc6 which has size 4
+    |                memory access failed: pointer must be in-bounds at offset 40, but is outside bounds of alloc6 which has size 4
+    |                inside `copy_nonoverlapping::<i32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+    |                inside `COPY_OOB_2` at $DIR/copy-intrinsic.rs:24:5
+   ::: $DIR/copy-intrinsic.rs:20:1
+    |
+    |
22 LL | / const COPY_OOB_2: () = unsafe {
23 LL | |     let x = 0i32;
24 LL | |     let dangle = (&x as *const i32).wrapping_add(10);

25 LL | |     // Even if the second ptr is an int ptr and this is a ZST copy, we should detect dangling 1st ptrs.
- LL | |     ptr::copy_nonoverlapping(dangle, 0x100 as *mut i32, 0);
+ ...  |
28 LL | |
- LL | |
30 LL | | };
30 LL | | };
31    | |__-
32    |

34    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
35 
36 error: any use of this value will cause an error
-   --> $DIR/copy-intrinsic.rs:32:5
+   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
38    |
+ LL |       unsafe { copy(src, dst, count) }
+    |                |
+    |                overflow computing total size of `copy`
+    |                overflow computing total size of `copy`
+    |                inside `std::intrinsics::copy::<i32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+    |                inside `COPY_SIZE_OVERFLOW` at $DIR/copy-intrinsic.rs:32:5
+   ::: $DIR/copy-intrinsic.rs:29:1
+    |
+    |
39 LL | / const COPY_SIZE_OVERFLOW: () = unsafe {
40 LL | |     let x = 0;
41 LL | |     let mut y = 0;

42 LL | |     ptr::copy(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1));
-    | |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow computing total size of `copy`
44 LL | |
45 LL | |
46 LL | | };
50    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
51 
51 
52 error: any use of this value will cause an error
-   --> $DIR/copy-intrinsic.rs:39:5
+   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
54    |
+ LL |       unsafe { copy_nonoverlapping(src, dst, count) }
+    |                |
+    |                overflow computing total size of `copy_nonoverlapping`
+    |                overflow computing total size of `copy_nonoverlapping`
+    |                inside `copy_nonoverlapping::<i32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
+    |                inside `COPY_NONOVERLAPPING_SIZE_OVERFLOW` at $DIR/copy-intrinsic.rs:39:5
+   ::: $DIR/copy-intrinsic.rs:36:1
+    |
+    |
55 LL | / const COPY_NONOVERLAPPING_SIZE_OVERFLOW: () = unsafe {
56 LL | |     let x = 0;
57 LL | |     let mut y = 0;

58 LL | |     ptr::copy_nonoverlapping(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1));
-    | |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overflow computing total size of `copy_nonoverlapping`
60 LL | |
61 LL | |
62 LL | | };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/copy-intrinsic/copy-intrinsic.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/copy-intrinsic.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/copy-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/copy-intrinsic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/copy-intrinsic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
LL |       unsafe { copy_nonoverlapping(src, dst, count) }
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                |
   |                memory access failed: pointer must be in-bounds at offset 40, but is outside bounds of alloc4 which has size 4
   |                inside `copy_nonoverlapping::<i32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   |                inside `COPY_OOB_1` at /checkout/src/test/ui/consts/copy-intrinsic.rs:16:5
  ::: /checkout/src/test/ui/consts/copy-intrinsic.rs:12:1
   |
   |
LL | / const COPY_OOB_1: () = unsafe {
LL | |     let mut x = 0i32;
LL | |     let dangle = (&mut x as *mut i32).wrapping_add(10);
LL | |     // Even if the first ptr is an int ptr and this is a ZST copy, we should detect dangling 2nd ptrs.
...  |
LL | |     //~| previously accepted
LL | | };
   |
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
LL |       unsafe { copy_nonoverlapping(src, dst, count) }
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                |
   |                memory access failed: pointer must be in-bounds at offset 40, but is outside bounds of alloc6 which has size 4
   |                inside `copy_nonoverlapping::<i32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   |                inside `COPY_OOB_2` at /checkout/src/test/ui/consts/copy-intrinsic.rs:24:5
  ::: /checkout/src/test/ui/consts/copy-intrinsic.rs:20:1
   |
   |
LL | / const COPY_OOB_2: () = unsafe {
LL | |     let x = 0i32;
LL | |     let dangle = (&x as *const i32).wrapping_add(10);
LL | |     // Even if the second ptr is an int ptr and this is a ZST copy, we should detect dangling 1st ptrs.
...  |
LL | |     //~| previously accepted
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
LL |       unsafe { copy(src, dst, count) }
   |                ^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                |
   |                overflow computing total size of `copy`
   |                inside `std::intrinsics::copy::<i32>` at /checkout/library/core/src/intrinsics.rs:1941:14
   |                inside `COPY_SIZE_OVERFLOW` at /checkout/src/test/ui/consts/copy-intrinsic.rs:32:5
   | 
  ::: /checkout/src/test/ui/consts/copy-intrinsic.rs:29:1
   |
LL | / const COPY_SIZE_OVERFLOW: () = unsafe {
LL | |     let x = 0;
LL | |     let mut y = 0;
LL | |     ptr::copy(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1)); //~ ERROR any use of this value will cause an error
LL | |     //~| overflow computing total size of `copy`
LL | |     //~| previously accepted
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
LL |       unsafe { copy_nonoverlapping(src, dst, count) }
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                |
   |                overflow computing total size of `copy_nonoverlapping`
   |                inside `copy_nonoverlapping::<i32>` at /checkout/library/core/src/intrinsics.rs:1861:14
   |                inside `COPY_NONOVERLAPPING_SIZE_OVERFLOW` at /checkout/src/test/ui/consts/copy-intrinsic.rs:39:5
   | 
  ::: /checkout/src/test/ui/consts/copy-intrinsic.rs:36:1
   |
LL | / const COPY_NONOVERLAPPING_SIZE_OVERFLOW: () = unsafe {
LL | |     let x = 0;
LL | |     let mut y = 0;
LL | |     ptr::copy_nonoverlapping(&x, &mut y, 1usize << (mem::size_of::<usize>() * 8 - 1)); //~ ERROR any use of this value will cause an error
LL | |     //~| overflow computing total size of `copy_nonoverlapping`
LL | |     //~| previously accepted
LL | | };
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

---
test result: FAILED. 11607 passed; 1 failed; 95 ignored; 0 measured; 0 filtered out; finished in 133.92s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:05
