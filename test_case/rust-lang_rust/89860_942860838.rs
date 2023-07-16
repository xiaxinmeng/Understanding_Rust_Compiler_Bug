plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 68 tests
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
.........F.......................F..................................

---- [ui] ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs stdout ----
diff of stderr:


18    |              ^^^^^^^^
19 ...
20 LL | custom_lint_pass_macro!();
+    | ------------------------- in this macro invocation
22    |
22    |
23    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
24    = note: this error originates in the macro `custom_lint_pass_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: implementing `LintPass` by hand
   |
   |
LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
   |
   |
LL | #![deny(rustc::lint_pass_impl_without_macro)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead

error: implementing `LintPass` by hand
   |
   |
LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
...
...
LL | custom_lint_pass_macro!();
   |
   |
   = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
   = note: this error originates in the macro `custom_lint_pass_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui-fulldeps/issue-76270-panic-in-libproc-macro.rs stdout ----
diff of stderr:

2   --> $DIR/issue-76270-panic-in-libproc-macro.rs:15:1
3    |
4 LL | proc_macro_panic::panic_in_libproc_macro!();
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7    = help: message: `""` is not a valid identifier


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro/issue-76270-panic-in-libproc-macro.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro/issue-76270-panic-in-libproc-macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issue-76270-panic-in-libproc-macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-76270-panic-in-libproc-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: proc macro panicked
  --> /checkout/src/test/ui-fulldeps/issue-76270-panic-in-libproc-macro.rs:15:1
   |
LL | proc_macro_panic::panic_in_libproc_macro!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: `""` is not a valid identifier
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 66 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.19s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:38
