plain
---- [ui] ui/associated-types/associated-types-project-from-hrtb-in-struct.rs stdout ----
diff of stderr:

18    |
19 help: use a fully qualified path with explicit lifetimes
20    |
- LL | enum SomeEnum<'c, 'b, I: for<'a> Foo<&'a isize>> {
- LL |     TupleVariant(<I as Foo<&'c isize>>::A),
+ LL | enum SomeEnum<'a, 'b, I: for<'a> Foo<&'a isize>> {
+ LL |     TupleVariant(<I as Foo<&'a isize>>::A),
24 
24 
25 error[E0212]: cannot use the associated type of a trait with uninferred generic parameters
30    |
30    |
31 help: use a fully qualified path with explicit lifetimes
32    |
- LL | enum SomeEnum<'c, 'b, I: for<'a> Foo<&'a isize>> {
+ LL | enum SomeEnum<'a, 'b, I: for<'a> Foo<&'a isize>> {
34 LL |     TupleVariant(I::A),
35 LL |
- LL |     StructVariant { field: <I as Foo<&'c isize>>::A },
+ LL |     StructVariant { field: <I as Foo<&'a isize>>::A },
38 
38 
39 error[E0212]: cannot use the associated type of a trait with uninferred generic parameters
44    |
44    |
45 help: use a fully qualified path with explicit lifetimes
46    |
- LL | struct Why<'bb, 'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x,
+ LL | struct Why<'l, 'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x,
48 LL |     'y, 'z, 'aa, I: for<'l, 'm> Foo<&'l &'m isize>> {
- LL |     field: <I as Foo<&'bb &'bb isize>>::A,
+ LL |     field: <I as Foo<&'l &'l isize>>::A,
51 
52 error: aborting due to 4 previous errors



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-struct/associated-types-project-from-hrtb-in-struct.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args associated-types/associated-types-project-from-hrtb-in-struct.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-project-from-hrtb-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-project-from-hrtb-in-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0212]: cannot use the associated type of a trait with uninferred generic parameters
   |
   |
LL |     field: I::A
   |
   |
help: use a fully qualified path with explicit lifetimes
   |
LL | struct SomeStruct<'a, I: for<'x> Foo<&'x isize>> {
LL |     field: <I as Foo<&'a isize>>::A


error[E0212]: cannot use the associated type of a trait with uninferred generic parameters
   |
   |
LL |     TupleVariant(I::A),
   |
   |
help: use a fully qualified path with explicit lifetimes
   |
LL | enum SomeEnum<'a, 'b, I: for<'a> Foo<&'a isize>> {
LL |     TupleVariant(<I as Foo<&'a isize>>::A),


error[E0212]: cannot use the associated type of a trait with uninferred generic parameters
   |
   |
LL |     StructVariant { field: I::A },
   |
   |
help: use a fully qualified path with explicit lifetimes
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | enum SomeEnum<'a, 'b, I: for<'a> Foo<&'a isize>> {
LL |     TupleVariant(I::A),
LL |     //~^ ERROR cannot use the associated type of a trait with uninferred generic parameters
LL |     StructVariant { field: <I as Foo<&'a isize>>::A },


error[E0212]: cannot use the associated type of a trait with uninferred generic parameters
   |
   |
LL |     field: I::A,
   |
   |
help: use a fully qualified path with explicit lifetimes
   |
LL | struct Why<'l, 'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k, 'n, 'o, 'p, 'q, 'r, 's, 't, 'u, 'v, 'w, 'x,
LL |     'y, 'z, 'aa, I: for<'l, 'm> Foo<&'l &'m isize>> {
LL |     field: <I as Foo<&'l &'l isize>>::A,

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0212`.
---
test result: FAILED. 11317 passed; 1 failed; 92 ignored; 0 measured; 0 filtered out; finished in 134.36s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:43
