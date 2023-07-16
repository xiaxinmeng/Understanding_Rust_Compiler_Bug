plain
.................................................................................................... 2100/11384
...............................F.................................................................... 2200/11384
.................................................................................................... 2300/11384
.................................................................................................... 2400/11384
...............F....F............................................................................... 2500/11384
...F...........................F...............................................................i..i. 2600/11384
.................................................................................................... 2800/11384
iiiii............................................................................................... 2900/11384
.................................................................................................... 3000/11384
.................................................................................................... 3100/11384
---

---- [ui] ui/consts/const-eval/const_raw_ptr_ops2.rs stdout ----
diff of stderr:

4 LL | const Y2: usize = unsafe { &1 as *const i32 as usize + 1 };
6    |                            |
6    |                            |
-    |                            "pointer-to-integer cast" needs an rfc before being allowed inside constants
+    |                            cannot cast pointer to integer because it was not created by cast from integer
8    |
9    = note: `#[deny(const_err)]` on by default


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops2/const_raw_ptr_ops2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops2/const_raw_ptr_ops2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_raw_ptr_ops2.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops2/auxiliary"
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL | const Y2: usize = unsafe { &1 as *const i32 as usize + 1 }; //~ ERROR any use of this
   |                            |
   |                            |
   |                            cannot cast pointer to integer because it was not created by cast from integer
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL | const Z2: i32 = unsafe { *(42 as *const i32) }; //~ ERROR any use of this value will cause
   |                          |
   |                          |
   |                          unable to turn bytes into a pointer
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL | const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR any use of this value will cause
   |                          |
   |                          |
   |                          unable to turn bytes into a pointer
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 3 previous errors
---

---- [ui] ui/consts/issue-51559.rs stdout ----
diff of stderr:

4 LL | pub const FOO: usize = unsafe { BAR as usize };
6    |                                 |
6    |                                 |
-    |                                 "pointer-to-integer cast" needs an rfc before being allowed inside constants
+    |                                 cannot cast pointer to integer because it was not created by cast from integer
8    |
9    = note: `#[deny(const_err)]` on by default


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559/issue-51559.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559/issue-51559.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-51559.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-51559.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-51559/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL | pub const FOO: usize = unsafe { BAR as usize };
   |                                 |
   |                                 |
   |                                 cannot cast pointer to integer because it was not created by cast from integer
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to previous error

---
diff of stderr:

20   --> $DIR/issue-52432.rs:7:10
21    |
22 LL |     [(); &(static || {}) as *const _ as usize];
-    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
+    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot cast pointer to integer because it was not created by cast from integer
25 error: aborting due to 4 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-52432/issue-52432.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-52432.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-52432.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-52432" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-52432/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0697]: closures cannot be static
  --> /checkout/src/test/ui/consts/issue-52432.rs:4:12
   |
LL |     [(); &(static |x| {}) as *const _ as usize];

error[E0697]: closures cannot be static
  --> /checkout/src/test/ui/consts/issue-52432.rs:7:12
   |
   |
LL |     [(); &(static || {}) as *const _ as usize];

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/consts/issue-52432.rs:4:20
   |
   |
LL |     [(); &(static |x| {}) as *const _ as usize];
   |                    ^ consider giving this closure parameter a type
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/issue-52432.rs:7:10
   |
   |
LL |     [(); &(static || {}) as *const _ as usize];
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot cast pointer to integer because it was not created by cast from integer
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0080, E0282, E0697.
For more information about an error, try `rustc --explain E0080`.
---
diff of stderr:

8   --> $DIR/ptr_arith.rs:16:14
9    |
10 LL |     let _v = x + 0;
-    |              ^^^^^ "pointer-to-integer cast" needs an rfc before being allowed inside constants
+    |              ^^^^^ cannot cast pointer to integer because it was not created by cast from integer
13 warning: skipping const checks
14    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith/ptr_arith.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/miri_unleashed/ptr_arith.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:9:14
   |
LL |     let _v = x == x;
   |              ^^^^^^ "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:16:14
   |
   |
LL |     let _v = x + 0;
   |              ^^^^^ cannot cast pointer to integer because it was not created by cast from integer
warning: skipping const checks
   |
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:9:14
   |
LL |     let _v = x == x;

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
For more information about this error, try `rustc --explain E0080`.

------------------------------------------


---- [ui] ui/consts/ptr_comparisons.rs stdout ----
diff of stderr:

36 LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
38    |                           |
38    |                           |
-    |                           "pointer-to-integer cast" needs an rfc before being allowed inside constants
+    |                           cannot cast pointer to integer because it was not created by cast from integer
41    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
42    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


47 LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
49    |                           |
49    |                           |
-    |                           "pointer-to-integer cast" needs an rfc before being allowed inside constants
+    |                           cannot cast pointer to integer because it was not created by cast from integer
52    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
53    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>



The actual stderr differed from the expected stderr.
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
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL | / const _: *const u8 =
LL | | //~^ NOTE
LL | |     unsafe { std::ptr::addr_of!((*(FOO as *const usize as *const [u8; 1000]))[999]) };
   | |_________________________________^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^___-
   |                                   |
   |                                   memory access failed: pointer must be in-bounds at offset 1000, but is outside bounds of alloc2 which has size 8
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL | const _: usize = unsafe { std::mem::transmute::<*const usize, usize>(FOO) + 4 };
   |                           |
   |                           |
   |                           cannot cast pointer to integer because it was not created by cast from integer
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>


error: any use of this value will cause an error
   |
   |
LL | const _: usize = unsafe { *std::mem::transmute::<&&usize, &usize>(&FOO) + 4 };
   |                           |
   |                           |
   |                           cannot cast pointer to integer because it was not created by cast from integer
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 4 previous errors
---
test result: FAILED. 11287 passed; 5 failed; 92 ignored; 0 measured; 0 filtered out; finished in 141.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:55
