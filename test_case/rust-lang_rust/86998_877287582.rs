plain
.............................................................i.i.................................... 12000/12070
......................................................................
failures:

---- [ui] ui/consts/const-eval/const_panic_2021.rs stdout ----

1 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:5:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | const A: () = std::panic!("blåhaj");
-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'blåhaj', $DIR/const_panic_2021.rs:5:15
+ LL |         panic_str(msg);
+    |         |
+    |         |
+    |         the evaluated program panicked at 'blåhaj', $DIR/const_panic_2021.rs:5:15
+    |         inside `panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
+    | 
+   ::: $DIR/const_panic_2021.rs:5:15
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+ LL | const A: () = std::panic!("blåhaj");
+    |               --------------------- inside `A` at $SRC_DIR/core/src/panic.rs:LL:COL
9 error[E0080]: evaluation of constant value failed
10   --> $DIR/const_panic_2021.rs:8:15

31    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
31    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
33 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:17:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
35    |
- LL | const E: () = core::panic!("shark");
-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'shark', $DIR/const_panic_2021.rs:17:15
+ LL |         panic_str(msg);
+    |         |
+    |         |
+    |         the evaluated program panicked at 'shark', $DIR/const_panic_2021.rs:17:15
+    |         inside `panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
+    | 
+   ::: $DIR/const_panic_2021.rs:17:15
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+ LL | const E: () = core::panic!("shark");
+    |               --------------------- inside `E` at $SRC_DIR/core/src/panic.rs:LL:COL
41 error[E0080]: evaluation of constant value failed
42   --> $DIR/const_panic_2021.rs:20:15



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/const_panic_2021.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_2021.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:101:9
   |
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'blåhaj', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:5:15
   |         inside `panic_fmt` at /checkout/library/core/src/panicking.rs:101:9
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:5:15
   |
   |
LL | const A: () = std::panic!("blåhaj");
   |               --------------------- inside `A` at /checkout/library/core/src/panic.rs:38:9
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:8:15
   |
   |
LL | const B: () = std::panic!();
   |               ^^^^^^^^^^^^^ the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:8:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:11:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:11:15
   |
LL | const C: () = std::unreachable!();
   |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:11:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:14:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:14:15
   |
LL | const D: () = std::unimplemented!();
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:14:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:101:9
  --> /checkout/library/core/src/panicking.rs:101:9
   |
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'shark', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:17:15
   |         inside `panic_fmt` at /checkout/library/core/src/panicking.rs:101:9
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:17:15
   |
   |
LL | const E: () = core::panic!("shark");
   |               --------------------- inside `E` at /checkout/library/core/src/panic.rs:38:9
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:20:15
   |
   |
LL | const F: () = core::panic!();
   |               ^^^^^^^^^^^^^^ the evaluated program panicked at 'explicit panic', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:20:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:23:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:23:15
   |
LL | const G: () = core::unreachable!();
   |               ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:23:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:26:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:26:15
   |
LL | const H: () = core::unimplemented!();
   |               ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:26:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 8 previous errors

---
test result: FAILED. 11969 passed; 1 failed; 100 ignored; 0 measured; 0 filtered out; finished in 126.25s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:34
