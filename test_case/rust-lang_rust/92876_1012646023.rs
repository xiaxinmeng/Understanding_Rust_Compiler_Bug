plain
.................................................................................................... 10700/12547
.................................................................................................... 10800/12547
.................................................................................................... 10900/12547
.................................................................................................... 11000/12547
...........................F..F..................................................................... 11100/12547
........i........................................................................................... 11300/12547
.................................................................................................... 11400/12547
.................................................................................................... 11500/12547
.................................................................................................... 11600/12547
---

---- [ui] ui/suggestions/issue-82566-1.rs stdout ----
diff of stderr:

4 LL |     T1<1>::C;
5    |       ^ ^
6    |
- help: use `::<...>` instead of `<...>` to specify type, lifetime, or const arguments
+ help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
8    |
9 LL |     T1::<1>::C;


15 LL |     T2<1, 2>::C;
16    |         ^ expected one of `.`, `;`, `?`, `}`, or an operator
17    |
- help: use `::<...>` instead of `<...>` to specify type, lifetime, or const arguments
+ help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
19    |
20 LL |     T2::<1, 2>::C;


26 LL |     T3<1, 2, 3>::C;
27    |         ^ expected one of `.`, `;`, `?`, `}`, or an operator
28    |
- help: use `::<...>` instead of `<...>` to specify type, lifetime, or const arguments
+ help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
30    |
31 LL |     T3::<1, 2, 3>::C;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-1/issue-82566-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-1/issue-82566-1.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-82566-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-82566-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/suggestions/issue-82566-1.rs:18:7
   |
LL |     T1<1>::C; //~ ERROR: comparison operators cannot be chained
   |       ^ ^
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
LL |     T1::<1>::C; //~ ERROR: comparison operators cannot be chained


error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
   |
   |
LL |     T2<1, 2>::C; //~ ERROR: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
   |         ^ expected one of `.`, `;`, `?`, `}`, or an operator
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
LL |     T2::<1, 2>::C; //~ ERROR: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`


error: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
   |
   |
LL |     T3<1, 2, 3>::C; //~ ERROR: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`
   |         ^ expected one of `.`, `;`, `?`, `}`, or an operator
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
LL |     T3::<1, 2, 3>::C; //~ ERROR: expected one of `.`, `;`, `?`, `}`, or an operator, found `,`

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/suggestions/issue-82566-2.rs stdout ----
diff of stderr:

4 LL | fn foo1() -> [(); Foo1<10>::SUM] {
5    |                       ^  ^
6    |
- help: use `::<...>` instead of `<...>` to specify type, lifetime, or const arguments
+ help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
8    |
9 LL | fn foo1() -> [(); Foo1::<10>::SUM] {


15 LL | fn foo2() -> [(); Foo2<10, 20>::SUM] {
16    |                          ^ expected one of `.`, `?`, `]`, or an operator
17    |
- help: use `::<...>` instead of `<...>` to specify type, lifetime, or const arguments
+ help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
19    |
20 LL | fn foo2() -> [(); Foo2::<10, 20>::SUM] {


26 LL | fn foo3() -> [(); Foo3<10, 20, 30>::SUM] {
27    |                          ^ expected one of `.`, `?`, `]`, or an operator
28    |
- help: use `::<...>` instead of `<...>` to specify type, lifetime, or const arguments
+ help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
30    |
31 LL | fn foo3() -> [(); Foo3::<10, 20, 30>::SUM] {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-2/issue-82566-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-2/issue-82566-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-82566-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-82566-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-82566-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/suggestions/issue-82566-2.rs:17:23
   |
LL | fn foo1() -> [(); Foo1<10>::SUM] { //~ ERROR: comparison operators cannot be chained
   |                       ^  ^
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
LL | fn foo1() -> [(); Foo1::<10>::SUM] { //~ ERROR: comparison operators cannot be chained


error: expected one of `.`, `?`, `]`, or an operator, found `,`
   |
   |
LL | fn foo2() -> [(); Foo2<10, 20>::SUM] {
   |                          ^ expected one of `.`, `?`, `]`, or an operator
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
LL | fn foo2() -> [(); Foo2::<10, 20>::SUM] {


error: expected one of `.`, `?`, `]`, or an operator, found `,`
   |
   |
LL | fn foo3() -> [(); Foo3<10, 20, 30>::SUM] {
   |                          ^ expected one of `.`, `?`, `]`, or an operator
   |
help: use `::<...>` instead of `<...>` to specify lifetime, type, or const arguments
   |
LL | fn foo3() -> [(); Foo3::<10, 20, 30>::SUM] {

error: aborting due to 3 previous errors


---
test result: FAILED. 12426 passed; 2 failed; 119 ignored; 0 measured; 0 filtered out; finished in 122.33s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:10:58
