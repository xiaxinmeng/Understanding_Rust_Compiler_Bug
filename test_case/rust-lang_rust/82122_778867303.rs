plain
.................................................................................................... 9200/11445
.................................................................................................... 9300/11445
.................................................................................................... 9400/11445
....i......i........................................................................................ 9500/11445
..........................................iiiiiii..iiiiii.i......................................... 9600/11445
.................................................................................................... 9800/11445
.................................................................................................... 9900/11445
.................................................................................................... 10000/11445
.................................................................................................... 10100/11445
---
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    | |
7    | calling non-const function `<Vec<i32> as Drop>::drop`
-    | inside `drop_in_place::<Vec<i32>> - shim(Some(Vec<i32>))` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
+    | inside `std::ptr::drop_in_place::<Vec<i32>> - shim(Some(Vec<i32>))` at $SRC_DIR/core/src/ptr/mod.rs:LL:COL
10   ::: $DIR/drop.rs:18:1
11    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/drop/drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/drop.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/drop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/drop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/library/core/src/ptr/mod.rs:179:1
   |
LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
   | |
   | |
   | calling non-const function `<Vec<i32> as Drop>::drop`
   | inside `std::ptr::drop_in_place::<Vec<i32>> - shim(Some(Vec<i32>))` at /checkout/library/core/src/ptr/mod.rs:179:1
  ::: /checkout/src/test/ui/consts/miri_unleashed/drop.rs:18:1
   |
LL | };
LL | };
   | - inside `TEST_BAD` at /checkout/src/test/ui/consts/miri_unleashed/drop.rs:18:1
warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
   |
   |
LL |     let _v: Vec<i32> = Vec::new();

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/recursion/issue-38591-non-regular-dropck-recursion.rs stdout ----
diff of stderr:

- error: reached the recursion limit while instantiating `drop_in_place::<S<fn(fn(fn(fn(fn...)))))))))))))))))))))))))))))>))`
+ error: reached the recursion limit while instantiating `std::ptr::drop_in_place::<S<fn(f...)))))))))))))))))))))))))))))>))`
2   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
3    |
4 LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
- note: `drop_in_place` defined here
+ note: `std::ptr::drop_in_place` defined here
8   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
9    |
10 LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/issue-38591-non-regular-dropck-recursion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/issue-38591-non-regular-dropck-recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/issue-38591-non-regular-dropck-recursion.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/issue-38591-non-regular-dropck-recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: reached the recursion limit while instantiating `std::ptr::drop_in_place::<S<fn(f...)))))))))))))))))))))))))))))>))`
   |
   |
LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
   |
note: `std::ptr::drop_in_place` defined here
  --> /checkout/library/core/src/ptr/mod.rs:179:1
   |
   |
LL | pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/issue-38591-non-regular-dropck-recursion/issue-38591-non-regular-dropck-recursion.long-type.txt'
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11350 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 133.52s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:49
