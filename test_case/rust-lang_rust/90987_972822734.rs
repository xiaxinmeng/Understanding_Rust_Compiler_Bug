plain

---- [ui] ui/const-generics/defaults/generic-expr-default.rs stdout ----
diff of stderr:

4 LL | pub fn needs_evaluatable_bound<const N1: usize>() -> Foo<N1> {
6    |
6    |
-    = help: try adding a `where` bound using this expression: `where [(); { N1 + 1 }]:`
+    = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
9 error: unconstrained generic constant
10   --> $DIR/generic-expr-default.rs:14:58

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args const-generics/defaults/generic-expr-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/generic-expr-default.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/generic-expr-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/defaults/generic-expr-default.rs:5:54
   |
LL | pub fn needs_evaluatable_bound<const N1: usize>() -> Foo<N1> {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/defaults/generic-expr-default.rs:14:58
   |
   |
LL | fn needs_evaluatable_bound_alias<T, const N: usize>() -> FooAlias<N>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs#full stdout ----
diff of stderr:

4 LL | struct ArithArrayLen<const N: usize>([u32; 0 + N]);
6    |
6    |
-    = help: try adding a `where` bound using this expression: `where [(); { 0 + N }]:`
+    = help: try adding a `where` bound using this expression: `where [(); 0 + N]:`
8 
9 error: overly complex generic constant


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.full/array-size-in-generic-struct-param.full.stderr
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs`


error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/array-size-in-generic-struct-param.rs:8:38
   |
LL | struct ArithArrayLen<const N: usize>([u32; 0 + N]);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); 0 + N]:`

error: overly complex generic constant
   |
   |
LL |     arr: [u8; CFG.arr_size],
   |               ^^^^^^^^^^^^ field access is not supported in generic constant
   |
   = help: consider moving this anonymous constant into a `const` function

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/const-generics/mismatched-const-errors.rs stdout ----
diff of stderr:

4 LL | fn foo1<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 2}, { N }>> {
6    |
6    |
-    = help: try adding a `where` bound using this expression: `where [(); { N + 2 + 1 }]:`
+    = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
8 note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 2}, N>`
10    |

20   --> $DIR/mismatched-const-errors.rs:9:30
21    |
21    |
22 LL | fn foo1<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 2}, { N }>> {
-    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N + 2 + 1 }`
+    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ M + 1 }`
25    = note: expected type `N`
25    = note: expected type `N`
-               found type `{ N + 2 + 1 }`
+               found type `{ M + 1 }`
28 error: unconstrained generic constant
29   --> $DIR/mismatched-const-errors.rs:15:30


31 LL | fn foo2<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N + 1 }>> {
33    |
33    |
-    = help: try adding a `where` bound using this expression: `where [(); { N + 1 + 1 }]:`
+    = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
35 note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1}, { N + 1 }>`
37    |

47   --> $DIR/mismatched-const-errors.rs:15:30
48    |
48    |
49 LL | fn foo2<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N + 1 }>> {
-    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N + 1 }`, found `{ N + 1 + 1 }`
+    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N + 1 }`, found `{ M + 1 }`
51    |
52    = note: expected type `{ N + 1 }`
-               found type `{ N + 1 + 1 }`
+               found type `{ M + 1 }`
55 error: unconstrained generic constant
56   --> $DIR/mismatched-const-errors.rs:21:30


58 LL | fn foo3<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N - 1}>> {
60    |
60    |
-    = help: try adding a `where` bound using this expression: `where [(); { N + 1 + 1 }]:`
+    = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
62 note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + 1}, { N - 1}>`
64    |

74   --> $DIR/mismatched-const-errors.rs:21:30
75    |
75    |
76 LL | fn foo3<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + 1}, { N - 1}>> {
-    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N - 1}`, found `{ N + 1 + 1 }`
+    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N - 1}`, found `{ M + 1 }`
78    |
79    = note: expected type `{ N - 1}`
-               found type `{ N + 1 + 1 }`
+               found type `{ M + 1 }`
82 error: unconstrained generic constant
83   --> $DIR/mismatched-const-errors.rs:27:44


85 LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
87    |
87    |
-    = help: try adding a `where` bound using this expression: `where [(); { N - 1 + 1 }]:`
+    = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
89 note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N - 1}, N>`
91    |

101   --> $DIR/mismatched-const-errors.rs:27:44
102    |
102    |
103 LL | fn foo4<const N: usize>(c : [usize; N]) -> HasTrait<HasCastInTraitImpl<{ N - 1}, { N }>> {
-    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ N - 1 + 1 }`
+    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ M + 1 }`
106    = note: expected type `N`
106    = note: expected type `N`
-               found type `{ N - 1 + 1 }`
+               found type `{ M + 1 }`
109 error: unconstrained generic constant
110   --> $DIR/mismatched-const-errors.rs:33:30


112 LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
114    |
114    |
-    = help: try adding a `where` bound using this expression: `where [(); { N + N + 1 }]:`
+    = help: try adding a `where` bound using this expression: `where [(); { M + 1 }]:`
116 note: required because of the requirements on the impl of `Trait` for `HasCastInTraitImpl<{ N + N }, { 2 * N }>`
118    |

128   --> $DIR/mismatched-const-errors.rs:33:30
129    |
129    |
130 LL | fn foo5<const N: usize>() -> HasTrait<HasCastInTraitImpl<{ N + N }, { 2 * N }>> {
-    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ 2 * N }`, found `{ N + N + 1 }`
+    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ 2 * N }`, found `{ M + 1 }`
132    |
133    = note: expected type `{ 2 * N }`
-               found type `{ N + N + 1 }`
+               found type `{ M + 1 }`
136 error: aborting due to 10 previous errors
137 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/mismatched-const-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/mismatched-const-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/mismatched-const-errors.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-errors/auxiliary"
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
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ M + 1 }`
   = note: expected type `N`
   = note: expected type `N`
              found type `{ M + 1 }`
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
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N + 1 }`, found `{ M + 1 }`
   |
   = note: expected type `{ N + 1 }`
              found type `{ M + 1 }`
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
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ N - 1}`, found `{ M + 1 }`
   |
   = note: expected type `{ N - 1}`
              found type `{ M + 1 }`
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
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `N`, found `{ M + 1 }`
   = note: expected type `N`
   = note: expected type `N`
              found type `{ M + 1 }`
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
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `{ 2 * N }`, found `{ M + 1 }`
   |
   = note: expected type `{ 2 * N }`
              found type `{ M + 1 }`
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12221 passed; 3 failed; 166 ignored; 0 measured; 0 filtered out; finished in 87.34s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:29
