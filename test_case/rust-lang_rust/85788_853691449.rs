plain
............................i....................................................................... 6200/11949
.i.................................................................................................. 6300/11949
.................................................................ii.ii.......i...i.................. 6400/11949
.................................................................................................... 6500/11949
.........i....i..................................i...........................i................FFF.FF 6600/11949
FF.F.F.............................................................................................. 6700/11949
.................................................................................................... 6900/11949
.......................ii...............................................i........................... 7000/11949
.................................................................................................... 7100/11949
.................................................................................................... 7200/11949
---
........................................i.i......................................................... 11900/11949
.................................................
failures:

---- [ui] ui/lint/force-warn/force-allowed-deny-by-default-lint.rs stdout ----

6    |                |
6    |                |
7    |                attempt to divide `1_i32` by zero
8    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-deny-by-default-lint/force-allowed-deny-by-default-lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-allowed-deny-by-default-lint.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-allowed-deny-by-default-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-deny-by-default-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "const_err" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-deny-by-default-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/lint/force-warn/force-allowed-deny-by-default-lint.rs:5:16
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: warning forced by `force-warns` commandline option
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/lint/force-warn/force-allowed-by-default-lint.rs stdout ----


4 LL | fn foo(x: &Foo) {}
5    |            ^^^- help: indicate the anonymous lifetime: `<'_>`
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9 warning: 1 warning emitted
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-by-default-lint/force-allowed-by-default-lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-allowed-by-default-lint.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-allowed-by-default-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-by-default-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "elided_lifetimes_in_paths" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-by-default-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: hidden lifetime parameters in types are deprecated
  --> /checkout/src/test/ui/lint/force-warn/force-allowed-by-default-lint.rs:8:12
   |
LL | fn foo(x: &Foo) {}
   |            ^^^- help: indicate the anonymous lifetime: `<'_>`
   |
   = note: warning forced by `force-warns` commandline option
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/force-warn/force-lint-allow-all-warnings.rs stdout ----

4 LL | fn dead_function() {}
5    |    ^^^^^^^^^^^^^
6    |
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9 warning: 1 warning emitted
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-allow-all-warnings/force-lint-allow-all-warnings.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-lint-allow-all-warnings.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-lint-allow-all-warnings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-allow-all-warnings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "dead_code" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-allow-all-warnings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: function is never used: `dead_function`
  --> /checkout/src/test/ui/lint/force-warn/force-lint-allow-all-warnings.rs:6:4
LL | fn dead_function() {}
   |    ^^^^^^^^^^^^^
   |
   |
   = note: warning forced by `force-warns` commandline option
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/force-warn/force-lint-group-allow-all-warnings.rs stdout ----

4 LL | pub fn FUNCTION() {}
4 LL | pub fn FUNCTION() {}
5    |        ^^^^^^^^ help: convert the identifier to snake case: `function`
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9 warning: 1 warning emitted
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-group-allow-all-warnings/force-lint-group-allow-all-warnings.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-lint-group-allow-all-warnings.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-lint-group-allow-all-warnings.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-group-allow-all-warnings" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "nonstandard_style" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-group-allow-all-warnings/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: function `FUNCTION` should have a snake case name
  --> /checkout/src/test/ui/lint/force-warn/force-lint-group-allow-all-warnings.rs:6:8
LL | pub fn FUNCTION() {}
LL | pub fn FUNCTION() {}
   |        ^^^^^^^^ help: convert the identifier to snake case: `function`
   |
   = note: warning forced by `force-warns` commandline option
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/force-warn/force-deny-by-default-lint.rs stdout ----

6    |                |
6    |                |
7    |                attempt to divide `1_i32` by zero
8    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
11    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-deny-by-default-lint/force-deny-by-default-lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-deny-by-default-lint.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-deny-by-default-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-deny-by-default-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "const_err" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-deny-by-default-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: any use of this value will cause an error
  --> /checkout/src/test/ui/lint/force-warn/force-deny-by-default-lint.rs:4:16
   |
LL | const C: i32 = 1 / 0;
   |                |
   |                |
   |                attempt to divide `1_i32` by zero
   |
   = note: warning forced by `force-warns` commandline option
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/lint/force-warn/force-allowed-warning.rs stdout ----

4 LL | fn dead_function() {}
5    |    ^^^^^^^^^^^^^
6    |
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9 warning: 1 warning emitted
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-warning/force-allowed-warning.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-allowed-warning.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-allowed-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-warning" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "dead_code" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-allowed-warning/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: function is never used: `dead_function`
  --> /checkout/src/test/ui/lint/force-warn/force-allowed-warning.rs:6:4
   |
LL | fn dead_function() {}
   |    ^^^^^^^^^^^^^
   |
   = note: warning forced by `force-warns` commandline option
warning: 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/lint/force-warn/force-lint-in-allowed-group.rs stdout ----


4 LL | pub fn function(_x: Box<SomeTrait>) {}
5    |                         ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9    = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-in-allowed-group/force-lint-in-allowed-group.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-lint-in-allowed-group.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-lint-in-allowed-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-in-allowed-group" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "bare_trait_objects" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-lint-in-allowed-group/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/lint/force-warn/force-lint-in-allowed-group.rs:8:25
   |
LL | pub fn function(_x: Box<SomeTrait>) {}
   |                         ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
   |
   = note: warning forced by `force-warns` commandline option
   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/lint/force-warn/force-warn-group-allow-warning.rs stdout ----


4 LL | pub fn function(_x: Box<SomeTrait>) {}
5    |                         ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9    = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-warn-group-allow-warning/force-warn-group-allow-warning.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-warn-group-allow-warning.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-warn-group-allow-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-warn-group-allow-warning" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "rust_2018_idioms" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-warn-group-allow-warning/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/lint/force-warn/force-warn-group-allow-warning.rs:8:25
   |
LL | pub fn function(_x: Box<SomeTrait>) {}
   |                         ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
   |
   = note: warning forced by `force-warns` commandline option
   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/lint/force-warn/force-warn-group.rs stdout ----


4 LL | pub fn function(_x: Box<SomeTrait>) {}
5    |                         ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
6    |
-    = note: Warning forced by `force-warns` commandline option
+    = note: warning forced by `force-warns` commandline option
9    = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-warn-group/force-warn-group.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/force-warn/force-warn-group.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/force-warn/force-warn-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-warn-group" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--force-warns" "rust_2018_idioms" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/force-warn/force-warn-group/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
  --> /checkout/src/test/ui/lint/force-warn/force-warn-group.rs:8:25
   |
LL | pub fn function(_x: Box<SomeTrait>) {}
   |                         ^^^^^^^^^ help: use `dyn`: `dyn SomeTrait`
   |
   = note: warning forced by `force-warns` commandline option
   = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

warning: 1 warning emitted



------------------------------------------



failures:
    [ui] ui/lint/force-warn/force-allowed-by-default-lint.rs
    [ui] ui/lint/force-warn/force-allowed-deny-by-default-lint.rs
    [ui] ui/lint/force-warn/force-allowed-warning.rs
    [ui] ui/lint/force-warn/force-deny-by-default-lint.rs
    [ui] ui/lint/force-warn/force-lint-allow-all-warnings.rs
    [ui] ui/lint/force-warn/force-lint-group-allow-all-warnings.rs
    [ui] ui/lint/force-warn/force-lint-in-allowed-group.rs
    [ui] ui/lint/force-warn/force-warn-group-allow-warning.rs
    [ui] ui/lint/force-warn/force-warn-group.rs
test result: FAILED. 11842 passed; 9 failed; 98 ignored; 0 measured; 0 filtered out; finished in 116.18s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:22
