plain
  libcilkrts5-dbg libmpx2-dbg libquadmath0-dbg gdb-doc gettext-base
  git-daemon-run | git-daemon-sysvinit git-doc git-el git-email git-gui gitk
  gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc gnupg | gnupg2 bzr
  gdbm-l10n krb5-doc krb5-user ncurses-doc libssl-doc libstdc++-7-doc
  llvm-9-doc make-doc ed diffutils-doc perl-doc libterm-readline-gnu-perl
  | libterm-readline-perl-perl python-doc python-tk ttf-bitstream-vera
  python3.6-doc readline-doc
Recommended packages:
  build-essential fakeroot gnupg | gnupg2 libalgorithm-merge-perl libc-dbg
  gdbserver less ssh-client manpages manpages-dev libfile-fcntllock-perl
---
Successfully built 31385ab06b01
Successfully tagged rust-ci:latest
Built container sha256:31385ab06b0161e08aabc641854a9e10b13912a809a6998d31f357f6ce997d74
Uploading finished image to https://ci-caches.rust-lang.org/docker/1d8776fc6611839c0fe1104032e04cb412cf84a696fcac0418b82e4341b3688a883196f99e2f55a0df0b3c67d154c62dd0c4630956d70292523c69687b63cce6
upload failed: - to s3://rust-lang-ci-sccache2/docker/1d8776fc6611839c0fe1104032e04cb412cf84a696fcac0418b82e4341b3688a883196f99e2f55a0df0b3c67d154c62dd0c4630956d70292523c69687b63cce6 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-9]
---

---- [ui] ui/const-ptr/out_of_bounds_read.rs stdout ----
diff of stderr:

1 error: any use of this value will cause an error
-   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
3    |
- LL |     unsafe { copy_nonoverlapping(src, dst, count) }
-    |              |
-    |              |
-    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
-    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-    |              inside `_READ` at $DIR/out_of_bounds_read.rs:13:33
+ LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
+    |         |
+    |         |
+    |         memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
+    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+    |         inside `_READ` at $DIR/out_of_bounds_read.rs:13:33
12   ::: $DIR/out_of_bounds_read.rs:13:5
13    |


17    = note: `#[deny(const_err)]` on by default
18 
19 error: any use of this value will cause an error
-   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
21    |
- LL |     unsafe { copy_nonoverlapping(src, dst, count) }
-    |              |
-    |              |
-    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
-    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-    |              inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |              inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:14:39
+ LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
+    |         |
+    |         |
+    |         memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
+    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+    |         inside `ptr::const_ptr::<impl *const u32>::read` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
+    |         inside `_CONST_READ` at $DIR/out_of_bounds_read.rs:14:39
31   ::: $DIR/out_of_bounds_read.rs:14:5
32    |

34    |     --------------------------------------------------------
34    |     --------------------------------------------------------
35 
36 error: any use of this value will cause an error
-   --> $SRC_DIR/core/src/intrinsics.rs:LL:COL
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
38    |
- LL |     unsafe { copy_nonoverlapping(src, dst, count) }
-    |              |
-    |              |
-    |              memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
-    |              inside `copy_nonoverlapping::<u32>` at $SRC_DIR/core/src/intrinsics.rs:LL:COL
-    |              inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
-    |              inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
-    |              inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:15:37
+ LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
+    |         |
+    |         |
+    |         memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
+    |         inside `std::ptr::read::<u32>` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+    |         inside `ptr::mut_ptr::<impl *mut u32>::read` at $SRC_DIR/core/src/ptr/mut_ptr.rs:LL:COL
+    |         inside `_MUT_READ` at $DIR/out_of_bounds_read.rs:15:37
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
48   ::: $DIR/out_of_bounds_read.rs:15:5
49    |



The actual stderr differed from the expected stderr.
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
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         |
   |         |
   |         memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:698:9
   |         inside `_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:13:33
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
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         |
   |         |
   |         memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:698:9
   |         inside `ptr::const_ptr::<impl *const u32>::read` at /checkout/library/core/src/ptr/const_ptr.rs:755:18
   |         inside `_CONST_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:39
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:14:5
   |
   |
LL |     const _CONST_READ: u32 = unsafe { PAST_END_PTR.read() };


error: any use of this value will cause an error
   |
   |
LL |         copy_nonoverlapping(src, tmp.as_mut_ptr(), 1);
   |         |
   |         |
   |         memory access failed: pointer must be in-bounds at offset 8, but is outside bounds of alloc6 which has size 4
   |         inside `std::ptr::read::<u32>` at /checkout/library/core/src/ptr/mod.rs:698:9
   |         inside `ptr::mut_ptr::<impl *mut u32>::read` at /checkout/library/core/src/ptr/mut_ptr.rs:862:18
   |         inside `_MUT_READ` at /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:37
  ::: /checkout/src/test/ui/const-ptr/out_of_bounds_read.rs:15:5
   |
   |
LL |     const _MUT_READ: u32 = unsafe { (PAST_END_PTR as *mut u32).read() };

error: aborting due to 3 previous errors


---
test result: FAILED. 11183 passed; 1 failed; 87 ignored; 0 measured; 0 filtered out; finished in 134.37s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:04
