plain
..............................................i.i................................................... 12200/12258
..........................................................
failures:

---- [ui] ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs stdout ----

1 error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
-   --> $DIR/feature-gate-non_exhaustive_omitted_patterns.rs:1:1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:1:1
3    |
4 LL | #![deny(non_exhaustive_omitted_patterns)]
5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

8    = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
10 error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
-   --> $DIR/feature-gate-non_exhaustive_omitted_patterns.rs:4:1
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:4:1
12    |
12    |
13 LL | #![allow(non_exhaustive_omitted_patterns)]
14    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

17    = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
19 error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
-   --> $DIR/feature-gate-non_exhaustive_omitted_patterns.rs:1:1
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:1:1
21    |
21    |
22 LL | #![deny(non_exhaustive_omitted_patterns)]
23    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

26    = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
28 error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
-   --> $DIR/feature-gate-non_exhaustive_omitted_patterns.rs:4:1
+   --> $DIR/feature-gate-non_exhaustive_omitted_patterns_lint.rs:4:1
30    |
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL | #![deny(non_exhaustive_omitted_patterns)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #89549 <https://github.com/rust-lang/rust/issues/89549> for more information
   = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs:4:1
   |
LL | #![allow(non_exhaustive_omitted_patterns)]
LL | #![allow(non_exhaustive_omitted_patterns)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #89549 <https://github.com/rust-lang/rust/issues/89549> for more information
   = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs:1:1
   |
LL | #![deny(non_exhaustive_omitted_patterns)]
LL | #![deny(non_exhaustive_omitted_patterns)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #89549 <https://github.com/rust-lang/rust/issues/89549> for more information
   = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
error[E0658]: the `non_exhaustive_omitted_patterns` lint is unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-non_exhaustive_omitted_patterns_lint.rs:4:1
   |
LL | #![allow(non_exhaustive_omitted_patterns)]
LL | #![allow(non_exhaustive_omitted_patterns)]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: see issue #89549 <https://github.com/rust-lang/rust/issues/89549> for more information
   = help: add `#![feature(non_exhaustive_omitted_patterns_lint)]` to the crate attributes to enable
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 12142 passed; 1 failed; 115 ignored; 0 measured; 0 filtered out; finished in 102.88s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:34
