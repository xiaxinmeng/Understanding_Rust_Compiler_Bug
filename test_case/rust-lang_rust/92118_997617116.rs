plain
1 error: where clauses are not allowed here
-   --> $DIR/type-alias-where.rs:9:15
+   --> $DIR/type-alias-where.rs:8:15
3    |
4 LL | type Bar = () where u32: Copy;

6 
7 error: where clause not allowed here
-   --> $DIR/type-alias-where.rs:23:38
-   --> $DIR/type-alias-where.rs:23:38
+   --> $DIR/type-alias-where.rs:22:38
9    |
10 LL |     type Assoc2 where u32: Copy = () where i32: Copy;
11    |                                -     ^^^^^^^^^^^^^^^

13    |                                help: move it here: `, i32: Copy`
15 error: where clause not allowed here
-   --> $DIR/type-alias-where.rs:29:21
+   --> $DIR/type-alias-where.rs:28:21
17    |
17    |
18 LL |     type Assoc = () where u32: Copy;
19    |               -     ^^^^^^^^^^^^^^^

21    |               help: move it here: `where u32: Copy`
23 error: where clause not allowed here
-   --> $DIR/type-alias-where.rs:32:22
+   --> $DIR/type-alias-where.rs:31:22
25    |
25    |
26 LL |     type Assoc2 = () where u32: Copy, i32: Copy;
27    |                -     ^^^^^^^^^^^^^^^^^^^^^^^^^^

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/type-alias-where/type-alias-where.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args parser/type-alias-where.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/type-alias-where.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/type-alias-where" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/type-alias-where/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: where clauses are not allowed here
  --> /checkout/src/test/ui/parser/type-alias-where.rs:8:15
   |
LL | type Bar = () where u32: Copy;

error: where clause not allowed here
  --> /checkout/src/test/ui/parser/type-alias-where.rs:22:38
   |
   |
LL |     type Assoc2 where u32: Copy = () where i32: Copy;
   |                                -     ^^^^^^^^^^^^^^^
   |                                |
   |                                help: move it here: `, i32: Copy`
error: where clause not allowed here
  --> /checkout/src/test/ui/parser/type-alias-where.rs:28:21
   |
   |
LL |     type Assoc = () where u32: Copy;
   |               -     ^^^^^^^^^^^^^^^
   |               |
   |               help: move it here: `where u32: Copy`
error: where clause not allowed here
  --> /checkout/src/test/ui/parser/type-alias-where.rs:31:22
   |
   |
LL |     type Assoc2 = () where u32: Copy, i32: Copy;
   |                -     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                |
   |                help: move it here: `where u32: Copy, i32: Copy`
error: aborting due to 4 previous errors


------------------------------------------
---
test result: FAILED. 12390 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 167.28s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:06
