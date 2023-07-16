plain

29 note: required by a bound in `std::mem::drop`
30   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
31    |
- LL | pub fn drop<T>(_x: T) {}
-    |             ^ required by this bound in `std::mem::drop`
+ LL | pub const fn drop<T: ~const Drop>(_x: T) {}
+    |                   ^ required by this bound in `std::mem::drop`
35    |
35    |
36 LL | fn main() where <() as Trait<'_>>::Item: Sized {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-60283/issue-60283.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args higher-rank-trait-bounds/issue-60283.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/issue-60283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-60283" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-60283/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0631]: type mismatch in function arguments
  --> /checkout/src/test/ui/higher-rank-trait-bounds/issue-60283.rs:17:13
   |
LL |     foo((), drop)
   |     |       |
   |     |       |
   |     |       expected signature of `for<'a> fn(<() as Trait<'a>>::Item) -> _`
   |     |       found signature of `fn(()) -> _`
   |     required by a bound introduced by this call
note: required by a bound in `foo`
  --> /checkout/src/test/ui/higher-rank-trait-bounds/issue-60283.rs:12:16
   |
   |
LL | pub fn foo<T, F>(_: T, _: F)
   |        --- required by a bound in this
...
LL |     F: for<'a> FnMut(<T as Trait<'a>>::Item),
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `foo`

error[E0277]: the size for values of type `<() as Trait<'_>>::Item` cannot be known at compilation time
   |
   |
LL |     foo((), drop)
   |     |
   |     required by a bound introduced by this call
   |
   |
   = help: the trait `Sized` is not implemented for `<() as Trait<'_>>::Item`
note: required by a bound in `std::mem::drop`
   |
   |
LL | pub const fn drop<T: ~const Drop>(_x: T) {}
   |                   ^ required by this bound in `std::mem::drop`
   |
   |
LL | fn main() where <() as Trait<'_>>::Item: Sized {

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0631.
---

---- [ui] ui/type_length_limit.rs stdout ----
diff of stderr:

1 error: reached the type-length limit while instantiating `std::mem::drop::<Option<((((...,....., ...), ..., ...), ..., ...)>>`
2   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
3    |
- LL | pub fn drop<T>(_x: T) {}
-    | ^^^^^^^^^^^^^^^^^^^^^
+ LL | pub const fn drop<T: ~const Drop>(_x: T) {}
6    |
6    |
7    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
8    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type_length_limit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: reached the type-length limit while instantiating `std::mem::drop::<Option<((((...,....., ...), ..., ...), ..., ...)>>`
   |
   |
LL | pub const fn drop<T: ~const Drop>(_x: T) {}
   |
   |
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.long-type.txt'
   = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12405 passed; 2 failed; 119 ignored; 0 measured; 0 filtered out; finished in 158.66s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:24
