plain
Successfully built f248b6f90e2c
Successfully tagged rust-ci:latest
Built container sha256:f248b6f90e2c2e361e3f21e9f416fc6a77c1933585ab9f45cb747b5d00d901de
Uploading finished image to https://ci-caches.rust-lang.org/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7
upload failed: - to s3://rust-lang-ci-sccache2/docker/dfd7203a0b015711c96f25420d9cb51dd6d72a416dd27c32932eb6b4d7efea10392bba63f0eaa514ea019391488096f30c8a7ead06c758f8f033ddf38b7029a7 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
.................................................................................................... 2400/12255
.................................................................................................... 2500/12255
.................................................................................................... 2600/12255
.................................................................................................... 2700/12255
.....F..F.........i..i.............................................................................. 2800/12255
...................................iiiii............................................................ 3000/12255
.................................................................................................... 3100/12255
.................................................................................................... 3200/12255
.................................................................................................... 3300/12255
---

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-1.rmeta
9            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-3.rmeta
+            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-1.rmeta
10            crate `crateresolve2`: $TEST_BUILD_DIR/crate-loading/crateresolve2/auxiliary/libcrateresolve2-2.rmeta
12 error: aborting due to previous error
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/crateresolve2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve2;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve2`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/auxiliary/libcrateresolve2-3.rmeta
           crate `crateresolve2`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/auxiliary/libcrateresolve2-1.rmeta
           crate `crateresolve2`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve2/auxiliary/libcrateresolve2-2.rmeta
error: aborting due to previous error


------------------------------------------
---

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = note: candidates:
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.dylib
-            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.dylib
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.so
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.so
+            crate `crateresolve1`: $TEST_BUILD_DIR/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.so
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/crateresolve1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args crate-loading/crateresolve1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/crate-loading/crateresolve1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | extern crate crateresolve1;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: candidates:
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-1.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-3.so
           crate `crateresolve1`: /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/crate-loading/crateresolve1/auxiliary/libcrateresolve1-2.so
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12138 passed; 2 failed; 115 ignored; 0 measured; 0 filtered out; finished in 126.91s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:25
