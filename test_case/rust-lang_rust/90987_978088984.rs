plain
---- [ui] ui/const-generics/mismatched-const-types-precedence.rs stdout ----
diff of stderr:

108    |
109    = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K + L }]:`
110 note: required by a bound in `foo3`
-   --> $DIR/mismatched-const-types-precedence.rs:37:96
112    |
112    |
- LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L]) -> [u32; 3 * M * {K + L}] {}
-    |                                                                                                ^^^^^^^^^^^^^^^ required by this bound in `foo3`
+ LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
+    |    ---- required by a bound in this
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ LL |   -> [u32; 3 * M * {K + L}] {}
+    |            ^^^^^^^^^^^^^^^ required by this bound in `foo3`
116 error: unconstrained generic constant
117   --> $DIR/mismatched-const-types-precedence.rs:30:10

138    |
138    |
139    = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K + L }]:`
140 note: required by a bound in `foo3`
-   --> $DIR/mismatched-const-types-precedence.rs:37:96
142    |
142    |
- LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L]) -> [u32; 3 * M * {K + L}] {}
-    |                                                                                                ^^^^^^^^^^^^^^^ required by this bound in `foo3`
+ LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
+    |    ---- required by a bound in this
+ LL |   -> [u32; 3 * M * {K + L}] {}
+    |            ^^^^^^^^^^^^^^^ required by this bound in `foo3`
146 error[E0308]: mismatched types
-   --> $DIR/mismatched-const-types-precedence.rs:37:90
+   --> $DIR/mismatched-const-types-precedence.rs:38:6
148    |
148    |
- LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L]) -> [u32; 3 * M * {K + L}] {}
-    |    ---- implicitly returns `()` as its body has no tail or `return` expression           ^^^^^^^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
+ LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
+    |    ---- implicitly returns `()` as its body has no tail or `return` expression
+ LL |   -> [u32; 3 * M * {K + L}] {}
+    |      ^^^^^^^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
152 error: aborting due to 15 previous errors
153 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence/mismatched-const-types-precedence.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/mismatched-const-types-precedence.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/mismatched-const-types-precedence/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:33
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N + K } }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * {M + K}] {}
   |                                                                           ^^^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:10
   |
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * 2 + N + K }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:33
   |
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `3 * 2 + N + K`, found `{ 3 * { 2 + N + K } }`
   |
   = note: expected type `3 * 2 + N + K`
              found type `{ 3 * { 2 + N + K } }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:6:33
   |
   |
LL |   let _: [u32; 3 * 2 + N + K] = foo::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N + K } }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * {M + K}] {}
   |                                                                           ^^^^^^^^^^^ required by this bound in `foo`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:13:69
   |
   |
LL | fn foo<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * {M + K}] {}
   |    ---                                                              ^^^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:33
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K }]:`
note: required by a bound in `foo2`
   |
   |
LL | fn foo2<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * M * K] {}
   |                                                                            ^^^^^^^^^ required by this bound in `foo2`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:10
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * 2 + N * K }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:33
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `3 * 2 + N * K`, found `{ 3 * { 2 + N } * K }`
   |
   = note: expected type `3 * 2 + N * K`
              found type `{ 3 * { 2 + N } * K }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:18:33
   |
   |
LL |   let _: [u32; 3 * 2 + N * K] = foo2::<{ 2 + N }, K>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K }]:`
note: required by a bound in `foo2`
   |
   |
LL | fn foo2<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * M * K] {}
   |                                                                            ^^^^^^^^^ required by this bound in `foo2`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:25:70
   |
   |
LL | fn foo2<const M: usize, const K: usize>(a: [u32; M], b: [u32; K]) -> [u32; 3 * M * K] {}
   |    ----                                                              ^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:37
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K + L }]:`
note: required by a bound in `foo3`
   |
   |
LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
   |    ---- required by a bound in this
LL |   -> [u32; 3 * M * {K + L}] {}
   |            ^^^^^^^^^^^^^^^ required by this bound in `foo3`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:10
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * 2 + N * K + L }]:`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:37
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `3 * 2 + N * K + L`, found `{ 3 * { 2 + N } * K + L }`
   |
   = note: expected type `3 * 2 + N * K + L`
              found type `{ 3 * { 2 + N } * K + L }`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:30:37
   |
   |
LL |   let _: [u32; 3 * 2 + N * K + L] = foo3::<{ 2 + N }, K, L>(a, b);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { 3 * { 2 + N } * K + L }]:`
note: required by a bound in `foo3`
   |
   |
LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
   |    ---- required by a bound in this
LL |   -> [u32; 3 * M * {K + L}] {}
   |            ^^^^^^^^^^^^^^^ required by this bound in `foo3`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/mismatched-const-types-precedence.rs:38:6
   |
   |
LL | fn foo3<const M: usize, const K: usize, const L: usize>(a: [u32; M], b: [u32; K + L])
   |    ---- implicitly returns `()` as its body has no tail or `return` expression
LL |   -> [u32; 3 * M * {K + L}] {}
   |      ^^^^^^^^^^^^^^^^^^^^^^ expected array `[u32; _]`, found `()`
error: aborting due to 15 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 12296 passed; 1 failed; 111 ignored; 0 measured; 0 filtered out; finished in 143.51s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:06
