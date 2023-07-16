plain

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.rlib
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.rlib
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.rlib
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/crateresolve1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve1.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.rlib
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.rlib
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.rlib
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.

---

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-1.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-2.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-3.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-1.rlib
+            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-2.rlib
+            crate `crateresolve1`: $TEST_BUILD_DIR/error-codes/E0464/auxiliary/libcrateresolve1-3.rlib
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/E0464.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0464.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0464.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary/libcrateresolve1-1.rlib
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary/libcrateresolve1-2.rlib
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0464/auxiliary/libcrateresolve1-3.rlib
error: aborting due to previous error

For more information about this error, try `rustc --explain E0464`.

---
test result: FAILED. 11630 passed; 2 failed; 637 ignored; 0 measured; 0 filtered out; finished in 95.63s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:18:20
