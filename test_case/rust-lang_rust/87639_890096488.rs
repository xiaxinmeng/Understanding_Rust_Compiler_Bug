plain
diff of stderr:

2   --> $DIR/cli-lint-override.rs:12:1
3    |
4 LL | extern fn foo() {}
-    | ^^^^^^^^^^^^^^^ ABI should be specified here
+    | ^^^^^^^^^^^^^^^^^^ ABI should be specified here
6    |
7    = note: requested on the command line with `-F missing-abi`
8    = help: the default ABI is C

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.forbid_warn/cli-lint-override.forbid_warn.stderr
To only update this specific test, also pass `--test-args lint/cli-lint-override.rs`


error in revision `forbid_warn`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/cli-lint-override.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "forbid_warn" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.forbid_warn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--warn" "missing_abi" "--forbid" "missing_abi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.forbid_warn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: extern declarations without an explicit ABI are deprecated
   |
LL | extern fn foo() {}
LL | extern fn foo() {}
   | ^^^^^^^^^^^^^^^^^^ ABI should be specified here
   |
   = note: requested on the command line with `-F missing-abi`
   = help: the default ABI is C
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lint/cli-lint-override.rs#force_warn_deny stdout ----

2   --> $DIR/cli-lint-override.rs:12:1
3    |
4 LL | extern fn foo() {}
4 LL | extern fn foo() {}
-    | ^^^^^^^^^^^^^^^ ABI should be specified here
+    | ^^^^^^^^^^^^^^^^^^ ABI should be specified here
6    |
7    = note: requested on the command line with `--force-warn missing-abi`
8    = help: the default ABI is C

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.force_warn_deny/cli-lint-override.force_warn_deny.stderr
To only update this specific test, also pass `--test-args lint/cli-lint-override.rs`


error in revision `force_warn_deny`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/cli-lint-override.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "force_warn_deny" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.force_warn_deny" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "--force-warn" "missing_abi" "--allow" "missing_abi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.force_warn_deny/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: extern declarations without an explicit ABI are deprecated
   |
LL | extern fn foo() {}
LL | extern fn foo() {}
   | ^^^^^^^^^^^^^^^^^^ ABI should be specified here
   |
   = note: requested on the command line with `--force-warn missing-abi`
   = help: the default ABI is C
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/cli-lint-override.rs#warn_deny stdout ----

2   --> $DIR/cli-lint-override.rs:12:1
3    |
4 LL | extern fn foo() {}
4 LL | extern fn foo() {}
-    | ^^^^^^^^^^^^^^^ ABI should be specified here
+    | ^^^^^^^^^^^^^^^^^^ ABI should be specified here
6    |
7    = note: requested on the command line with `-D missing-abi`
8    = help: the default ABI is C

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.warn_deny/cli-lint-override.warn_deny.stderr
To only update this specific test, also pass `--test-args lint/cli-lint-override.rs`


error in revision `warn_deny`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/cli-lint-override.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "warn_deny" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.warn_deny" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--warn" "missing_abi" "--deny" "missing_abi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/cli-lint-override.warn_deny/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: extern declarations without an explicit ABI are deprecated
   |
LL | extern fn foo() {}
LL | extern fn foo() {}
   | ^^^^^^^^^^^^^^^^^^ ABI should be specified here
   |
   = note: requested on the command line with `-D missing-abi`
   = help: the default ABI is C
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11973 passed; 3 failed; 103 ignored; 0 measured; 0 filtered out; finished in 124.93s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:48
