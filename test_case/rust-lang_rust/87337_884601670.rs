plain
diff of stderr:

7    = note: `#[warn(renamed_and_removed_lints)]` on by default
8 
9 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:13:9
11    |
11    |
12 LL | #![deny(clippy_group)]
13    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
14 
14 
15 warning: lint name `test_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:29:9
17    |
17    |
18 LL | #[allow(test_group)]
19    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
20 
20 
21 warning: unknown lint: `this_lint_does_not_exist`
-   --> $DIR/lint-tool-test.rs:33:8
23    |
23    |
24 LL | #[deny(this_lint_does_not_exist)]


33    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
34 
35 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:13:9
37    |
37    |
38 LL | #![deny(clippy_group)]
39    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
40 
40 
41 warning: lint name `test_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:29:9
43    |
43    |
44 LL | #[allow(test_group)]
45    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

59    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
60 
61 warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:13:9
63    |
63    |
64 LL | #![deny(clippy_group)]
65    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
66 
66 
67 error: item is named 'lintme'
-   --> $DIR/lint-tool-test.rs:18:1
69    |
69    |
70 LL | fn lintme() { }

72    |
73 note: the lint level is defined here
-   --> $DIR/lint-tool-test.rs:13:9
-   --> $DIR/lint-tool-test.rs:13:9
+   --> $DIR/lint-tool-test.rs:14:9
75    |
76 LL | #![deny(clippy_group)]


78    = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`
79 
80 error: item is named 'lintmetoo'
-   --> $DIR/lint-tool-test.rs:26:5
82    |
82    |
83 LL |     fn lintmetoo() { }

85    |
86 note: the lint level is defined here
-   --> $DIR/lint-tool-test.rs:13:9
-   --> $DIR/lint-tool-test.rs:13:9
+   --> $DIR/lint-tool-test.rs:14:9
88    |
89 LL | #![deny(clippy_group)]


91    = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`
92 
93 warning: lint name `test_group` is deprecated and may not have an effect in the future.
-   --> $DIR/lint-tool-test.rs:29:9
95    |
95    |
96 LL | #[allow(test_group)]
97    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
98 
- error: aborting due to 2 previous errors; 11 warnings emitted
- error: aborting due to 2 previous errors; 11 warnings emitted
+ warning: lint name `test_lint` is deprecated and may not have an effect in the future.
+    |
+    |
+ LL | #![cfg_attr(foo, warn(test_lint))]
+    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
+ 
+ warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
+    |
+    |
+ LL | #![deny(clippy_group)]
+    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
+ 
+ warning: lint name `test_group` is deprecated and may not have an effect in the future.
+    |
+    |
+ LL | #[allow(test_group)]
+    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
+ error: aborting due to 2 previous errors; 14 warnings emitted
100 
101 

---
To only update this specific test, also pass `--test-args lint-tool-test.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
   = note: `#[warn(renamed_and_removed_lints)]` on by default


warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

warning: unknown lint: `this_lint_does_not_exist`
   |
   |
LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
   |
   = note: `#[warn(unknown_lints)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`

warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See https://github.com/rust-lang/rust/pull/64675
   |
   |
LL | #![plugin(lint_tool_test)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
   = note: `#[warn(deprecated)]` on by default

warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

error: item is named 'lintme'
   |
   |
LL | fn lintme() { } //~ ERROR item is named 'lintme'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:14:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_lint)]` implied by `#[deny(clippy::group)]`

error: item is named 'lintmetoo'
   |
   |
LL |     fn lintmetoo() { } //~ ERROR item is named 'lintmetoo'
   |
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:14:9
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^
   = note: `#[deny(clippy::test_group)]` implied by `#[deny(clippy::group)]`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
warning: lint name `test_lint` is deprecated and may not have an effect in the future.
  --> /checkout/src/test/ui-fulldeps/lint-tool-test.rs:9:23
   |
   |
LL | #![cfg_attr(foo, warn(test_lint))]
   |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`

warning: lint name `clippy_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #![deny(clippy_group)]
   |         ^^^^^^^^^^^^ help: change it to: `clippy::group`

warning: lint name `test_group` is deprecated and may not have an effect in the future.
   |
   |
LL | #[allow(test_group)]
   |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
error: aborting due to 2 previous errors; 14 warnings emitted


------------------------------------------
---
test result: FAILED. 66 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 11.44s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui-fulldeps" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:54
