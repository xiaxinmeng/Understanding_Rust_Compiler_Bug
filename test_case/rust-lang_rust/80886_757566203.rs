plain
.................................................................................................... 9000/11247
.................................................................................................... 9100/11247
.................................................................................................... 9200/11247
...........................................i......i................................................. 9300/11247
..................................................................................iiiiii..iiiiii.i.. 9400/11247
.................................................................................................... 9600/11247
.................................................................................................... 9700/11247
.................................................................................................... 9800/11247
.................................................................................................... 9900/11247
---
---- [ui] ui/consts/ptr_comparisons.rs stdout ----
diff of stderr:

6    |                  |
7    |                  inbounds test failed: pointer must be in-bounds at offset $TWO_WORDS, but is outside bounds of alloc2 which has size $WORD
8    |                  inside `ptr::const_ptr::<impl *const usize>::offset` at $SRC_DIR/core/src/ptr/const_ptr.rs:LL:COL
-    |                  inside `_` at $DIR/ptr_comparisons.rs:62:34
+    |                  inside `_` at $DIR/ptr_comparisons.rs:61:34
-   ::: $DIR/ptr_comparisons.rs:62:1
+   ::: $DIR/ptr_comparisons.rs:61:1
12    |
12    |
13 LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };


16    = note: `#[deny(const_err)]` on by default
17 
18 error: any use of this value will cause an error
-   --> $DIR/ptr_comparisons.rs:67:14
20    |
20    |
21 LL | / const _: *const u8 =
22 LL | |
28    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
29 
29 
30 error: any use of this value will cause an error
-   --> $DIR/ptr_comparisons.rs:70:27
32    |
32    |
33 LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };


36    |                           "pointer-to-integer cast" needs an rfc before being allowed inside constants
37 
38 error: any use of this value will cause an error
-   --> $DIR/ptr_comparisons.rs:75:27
40    |
40    |
41 LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/ptr_comparisons.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/ptr_comparisons.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/ptr_comparisons.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |         unsafe { intrinsics::offset(self, count) }
   |                  |
   |                  |
   |                  inbounds test failed: pointer must be in-bounds at offset 16, but is outside bounds of alloc2 which has size 8
   |                  inside `ptr::const_ptr::<impl *const usize>::offset` at /checkout/library/core/src/ptr/const_ptr.rs:225:18
   |                  inside `_` at /checkout/src/test/ui/consts/ptr_comparisons.rs:61:34
  ::: /checkout/src/test/ui/consts/ptr_comparisons.rs:61:1
   |
   |
LL | const _: *const usize = unsafe { (FOO as *const usize).offset(2) };
   |
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL | / const _: *const u8 =
LL | | //~^ NOTE
LL | |     unsafe { std::ptr::const_addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
   | |______________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^__-
   |                |
   |                memory access failed: pointer must be in-bounds at offset 1000, but is outside bounds of alloc2 which has size 8
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)


error: any use of this value will cause an error
   |
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           |
   |                           |
   |                           "pointer-to-integer cast" needs an rfc before being allowed inside constants

error: any use of this value will cause an error
   |
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           |
   |                           |
   |                           "pointer-to-integer cast" needs an rfc before being allowed inside constants
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 11160 passed; 1 failed; 86 ignored; 0 measured; 0 filtered out; finished in 136.07s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:29
