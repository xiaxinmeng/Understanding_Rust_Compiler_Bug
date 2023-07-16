plain

10 LL |     mac!();
11    |     ^^^ ambiguous name
12    |
-    = note: ambiguous because of macro name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
14 note: `mac` could refer to the macro defined here
16    |


Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/issue-53269.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-53269.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-53269.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-53269/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `nonexistent_module`
  --> /checkout/src/test/ui/imports/issue-53269.rs:6:9
   |
LL |     use nonexistent_module::mac; //~ ERROR unresolved import `nonexistent_module`
   |         ^^^^^^^^^^^^^^^^^^ maybe a missing crate `nonexistent_module`?

error[E0659]: `mac` is ambiguous
   |
   |
LL |     mac!(); //~ ERROR `mac` is ambiguous
   |     ^^^ ambiguous name
   |
   = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
note: `mac` could refer to the macro defined here
   |
   |
LL | macro_rules! mac { () => () }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `mac` could also refer to the unresolved item imported here
   |
   |
LL |     use nonexistent_module::mac; //~ ERROR unresolved import `nonexistent_module`
   |         ^^^^^^^^^^^^^^^^^^^^^^^
   = help: use `self::mac` to refer to this unresolved item unambiguously
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0432, E0659.
For more information about an error, try `rustc --explain E0432`.
---

4 LL |         m!()
5    |         ^ ambiguous name
6    |
-    = note: ambiguous because of macro name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
8 note: `m` could refer to the macro defined here
9   --> $DIR/ambiguity-legacy-vs-modern.rs:26:5

22 LL |     m!()
23    |     ^ ambiguous name
24    |
24    |
-    = note: ambiguous because of macro name conflicts with non-`macro_rules` name from another module
+    = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
26 note: `m` could refer to the macro defined here
27   --> $DIR/ambiguity-legacy-vs-modern.rs:40:9


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern/ambiguity-legacy-vs-modern.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern/ambiguity-legacy-vs-modern.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/ambiguity-legacy-vs-modern.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/ambiguity-legacy-vs-modern.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/ambiguity-legacy-vs-modern/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0659]: `m` is ambiguous
   |
   |
LL |         m!() //~ ERROR `m` is ambiguous
   |         ^ ambiguous name
   |
   = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
note: `m` could refer to the macro defined here
   |
   |
LL |     macro_rules! m { () => (()) }
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `m` could also refer to the macro defined here
   |
   |
LL |         macro m() { 0 }


error[E0659]: `m` is ambiguous
   |
   |
LL |     m!() //~ ERROR `m` is ambiguous
   |     ^ ambiguous name
   |
   = note: ambiguous because of `macro_rules` name conflicts with non-`macro_rules` name from another module
note: `m` could refer to the macro defined here
   |
   |
LL |         macro_rules! m { () => (()) }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: `m` could also refer to the macro defined here
   |
   |
LL |     macro m() { 0 }

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0659`.
---
test result: FAILED. 12226 passed; 2 failed; 110 ignored; 0 measured; 0 filtered out; finished in 147.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:53
