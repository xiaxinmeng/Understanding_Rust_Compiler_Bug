plain

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-1.rmeta
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-2.rmeta
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-3.rmeta
+            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2.nll/auxiliary/libcrateresolve2-1.rmeta
+            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2.nll/auxiliary/libcrateresolve2-2.rmeta
+            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2.nll/auxiliary/libcrateresolve2-3.rmeta
11 
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
13 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2.nll/crateresolve2.nll.stderr
To only update this specific test, also pass `--test-args crate-loading/crateresolve2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve2;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve2`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2.nll/auxiliary/libcrateresolve2-1.rmeta
           crate `crateresolve2`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2.nll/auxiliary/libcrateresolve2-2.rmeta
           crate `crateresolve2`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2.nll/auxiliary/libcrateresolve2-3.rmeta
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.

---

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1.nll/auxiliary/libcrateresolve1-1.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1.nll/auxiliary/libcrateresolve1-2.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1.nll/auxiliary/libcrateresolve1-3.dylib
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1.nll/crateresolve1.nll.stderr
To only update this specific test, also pass `--test-args crate-loading/crateresolve1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1.nll/auxiliary/libcrateresolve1-1.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1.nll/auxiliary/libcrateresolve1-2.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1.nll/auxiliary/libcrateresolve1-3.so
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.

---

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-1.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-2.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-3.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464.nll/auxiliary/libcrateresolve1-1.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464.nll/auxiliary/libcrateresolve1-2.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464.nll/auxiliary/libcrateresolve1-3.dylib
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464.nll/E0464.nll.stderr
To only update this specific test, also pass `--test-args error-codes/E0464.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0464.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464.nll/auxiliary/libcrateresolve1-1.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464.nll/auxiliary/libcrateresolve1-2.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464.nll/auxiliary/libcrateresolve1-3.so
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.

---
test result: FAILED. 12121 passed; 3 failed; 142 ignored; 0 measured; 0 filtered out; finished in 105.66s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always" "--compare-mode" "nll"


Build completed unsuccessfully in 0:18:23
