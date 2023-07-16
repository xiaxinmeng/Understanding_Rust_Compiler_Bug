plain
diff of stderr:

22   --> $DIR/extern-prelude-from-opaque-fail.rs:11:18
23    |
24 LL |         fn f() { my_core::mem::drop(0); }
-    |                  ^^^^^^^ use of undeclared crate or module `my_core`
+    |                  |
+    |                  use of undeclared crate or module `my_core`
+    |                  use of undeclared crate or module `my_core`
+    |                  help: a similar crate or module exists: `my_core`
26 ...
27 LL | a!();

33   --> $DIR/extern-prelude-from-opaque-fail.rs:24:14
34    |
34    |
35 LL |     fn f() { my_core::mem::drop(0); }
-    |              ^^^^^^^ use of undeclared crate or module `my_core`
+    |              |
+    |              use of undeclared crate or module `my_core`
+    |              use of undeclared crate or module `my_core`
+    |              help: a similar crate or module exists: `my_core`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
38 error: aborting due to 4 previous errors
39 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail/extern-prelude-from-opaque-fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/extern-prelude-from-opaque-fail.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/extern-prelude-from-opaque-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/extern-prelude-from-opaque-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0432]: unresolved import `my_core`
   |
   |
LL |     use my_core; //~ ERROR unresolved import `my_core`
   |         |
   |         |
   |         no `my_core` in the root
   |         help: a similar name exists in the module: `my_core`

error[E0432]: unresolved import `my_core`
   |
   |
LL |         use my_core; //~ ERROR unresolved import `my_core`
   |             ^^^^^^^ no `my_core` in the root
...
LL | a!();
   |
   |
   = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0433]: failed to resolve: use of undeclared crate or module `my_core`
  --> /checkout/src/test/ui/hygiene/extern-prelude-from-opaque-fail.rs:11:18
   |
   |
LL |         fn f() { my_core::mem::drop(0); }
   |                  |
   |                  use of undeclared crate or module `my_core`
   |                  use of undeclared crate or module `my_core`
   |                  help: a similar crate or module exists: `my_core`
...
LL | a!();
   |
   |
   = note: this error originates in the macro `a` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0433]: failed to resolve: use of undeclared crate or module `my_core`
  --> /checkout/src/test/ui/hygiene/extern-prelude-from-opaque-fail.rs:24:14
   |
   |
LL |     fn f() { my_core::mem::drop(0); }
   |              |
   |              use of undeclared crate or module `my_core`
   |              use of undeclared crate or module `my_core`
   |              help: a similar crate or module exists: `my_core`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0432, E0433.
For more information about an error, try `rustc --explain E0432`.
---
test result: FAILED. 12102 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 130.29s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:20
