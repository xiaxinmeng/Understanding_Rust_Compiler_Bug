plain

---- [ui] ui/save-analysis/issue-89066.rs stdout ----
diff of stderr:

10 LL | fn bad_infer_fn<_>() {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11    |                 ^ expected identifier, found reserved identifier
12 
- error: in expressions, `_` can only be used on the left-hand side of an assignment
-   --> $DIR/issue-89066.rs:26:15
-    |
- LL |   let v: [u8; _];
-    |               ^ `_` not allowed here
- 
- error: in expressions, `_` can only be used on the left-hand side of an assignment
-   --> $DIR/issue-89066.rs:28:25
-    |
- LL |   let v: [u8; 10] = [0; _];
-    |                         ^ `_` not allowed here
- 
25 error[E0392]: parameter `_` is never used
27    |


45 LL | struct All<'a, T, const N: usize> {
46    |        ^^^     -        -
- error: aborting due to 6 previous errors
+ error: aborting due to 4 previous errors
49 
50 Some errors have detailed explanations: E0107, E0392.
---
To only update this specific test, also pass `--test-args save-analysis/issue-89066.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/save-analysis/issue-89066.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-89066" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zsave-analysis" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/save-analysis/issue-89066/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/save-analysis/issue-89066.rs:12:17
   |
LL | struct BadInfer<_>;
   |                 ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/save-analysis/issue-89066.rs:18:17
   |
   |
LL | fn bad_infer_fn<_>() {}
   |                 ^ expected identifier, found reserved identifier

error[E0392]: parameter `_` is never used
   |
   |
LL | struct BadInfer<_>;
   |                 ^ unused parameter
   |
   = help: consider removing `_`, referring to it in a field, or using a marker such as `PhantomData`
   = help: if you intended `_` to be a const parameter, use `const _: usize` instead

error[E0107]: this struct takes 2 generic arguments but 3 generic arguments were supplied
   |
   |
LL |   let a: All<_, _, _>;
   |          ^^^       - help: remove this generic argument
   |          expected 2 generic arguments
   |
   |
note: struct defined here, with 2 generic parameters: `T`, `N`
   |
   |
LL | struct All<'a, T, const N: usize> {
   |        ^^^     -        -
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0107, E0392.
For more information about an error, try `rustc --explain E0107`.
---
test result: FAILED. 12421 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 152.78s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:30
