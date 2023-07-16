plain
.................................................................................................... 12300/12307
.......
failures:

---- [ui] ui/generic-associated-types/self-outlives-lint.rs stdout ----


31    |                     help: add the required where clauses: `where U: 'y, T: 'x`
33 error: Missing required bounds on Out
-   --> $DIR/self-outlives-lint.rs:60:5
+   --> $DIR/self-outlives-lint.rs:59:5
35    |
35    |
36 LL |     type Out<'x, D>;


39    |                    help: add the required where clauses: `where D: 'x`
41 error: Missing required bounds on Out
-   --> $DIR/self-outlives-lint.rs:76:5
+   --> $DIR/self-outlives-lint.rs:75:5
43    |
43    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
44 LL |     type Out<'x, D>;


47    |                    help: add the required where clauses: `where D: 'x`
49 error: Missing required bounds on Out
-   --> $DIR/self-outlives-lint.rs:91:5
+   --> $DIR/self-outlives-lint.rs:90:5
51    |
51    |
52 LL |     type Out<'x, D>;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint/self-outlives-lint.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint/self-outlives-lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/self-outlives-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: Missing required bounds on Item
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:7:5
   |
LL |     type Item<'x>;
   |                  |
   |                  |
   |                  help: add the required where clauses: `where Self: 'x`
error: Missing required bounds on Out
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:23:5
   |
   |
LL |     type Out<'x>;
   |                 |
   |                 |
   |                 help: add the required where clauses: `where T: 'x`
error: Missing required bounds on Out
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:37:5
   |
   |
LL |     type Out<'x>;
   |                 |
   |                 |
   |                 help: add the required where clauses: `where T: 'x`
error: Missing required bounds on Out
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:44:5
   |
   |
LL |     type Out<'x, 'y>;
   |                     |
   |                     |
   |                     help: add the required where clauses: `where U: 'y, T: 'x`
error: Missing required bounds on Out
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:59:5
   |
   |
LL |     type Out<'x, D>;
   |                    |
   |                    |
   |                    help: add the required where clauses: `where D: 'x`
error: Missing required bounds on Out
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:75:5
   |
   |
LL |     type Out<'x, D>;
   |                    |
   |                    |
   |                    help: add the required where clauses: `where D: 'x`
error: Missing required bounds on Out
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:90:5
   |
   |
LL |     type Out<'x, D>;
   |                    |
   |                    |
   |                    help: add the required where clauses: `where D: 'x`
error: aborting due to 7 previous errors


------------------------------------------
---
test result: FAILED. 12189 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 127.01s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:29
