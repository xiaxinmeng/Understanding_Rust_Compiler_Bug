plain

---- [ui] ui/const-generics/mismatched-const-errors.rs stdout ----
diff of stderr:

80               found type `{ N + 1 + 1 }`
82 error: unconstrained generic constant
-   --> $DIR/mismatched-const-errors.rs:28:44
+   --> $DIR/mismatched-const-errors.rs:27:44
84    |
84    |
85 LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {


98    |                        ^^^^^ required by this bound in `HasTrait`
100 error[E0308]: mismatched types
-   --> $DIR/mismatched-const-errors.rs:28:44
+   --> $DIR/mismatched-const-errors.rs:27:44
102    |
102    |
103 LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
104    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N - 1 + 1 }`

107               found type `{ N - 1 + 1 }`
109 error: unconstrained generic constant
-   --> $DIR/mismatched-const-errors.rs:34:30
+   --> $DIR/mismatched-const-errors.rs:33:30
111    |
111    |
112 LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {


125    |                        ^^^^^ required by this bound in `HasTrait`
127 error[E0308]: mismatched types
-   --> $DIR/mismatched-const-errors.rs:34:30
+   --> $DIR/mismatched-const-errors.rs:33:30
129    |
129    |
130 LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
131    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ 2 * N }`, found `{ N + N + 1 }`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/mismatched-const-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/mismatched-const-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/mismatched-const-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:9:30
   |
LL | fn foo1<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 2}, { N }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 2}, N>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:9:30
   |
   |
LL | fn foo1<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 2}, { N }>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N + 2 + 1 }`
   = note: expected type `N`
   = note: expected type `N`
              found type `{ N + 2 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:15:30
   |
   |
LL | fn foo2<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N + 1 }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1}, { N + 1 }>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:15:30
   |
   |
LL | fn foo2<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N + 1 }>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N + 1 }`, found `{ N + 1 + 1 }`
   |
   = note: expected type `{ N + 1 }`
              found type `{ N + 1 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:21:30
   |
   |
LL | fn foo3<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N - 1}>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1}, { N - 1}>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:21:30
   |
   |
LL | fn foo3<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N - 1}>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N - 1}`, found `{ N + 1 + 1 }`
   |
   = note: expected type `{ N - 1}`
              found type `{ N + 1 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:27:44
   |
   |
LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N - 1}, N>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:27:44
   |
   |
LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N - 1 + 1 }`
   = note: expected type `N`
   = note: expected type `N`
              found type `{ N - 1 + 1 }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:33:30
   |
   |
LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + N }, { 2 * N }>`
   |
   |
LL | impl<const M: usize> Trait for HasCastInTraitImpl<M, { M + 1 }> {}
   |                      ^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: required by a bound in `HasTrait`
   |
   |
LL | pub struct HasTrait<T: Trait>(T);
   |                        ^^^^^ required by this bound in `HasTrait`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-errors.rs:33:30
   |
   |
LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ 2 * N }`, found `{ N + N + 1 }`
   |
   = note: expected type `{ 2 * N }`
              found type `{ N + N + 1 }`
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12278 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 136.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:04
