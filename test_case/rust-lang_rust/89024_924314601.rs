plain

---- [ui] ui/type-alias-impl-trait/bound_reduction2.rs stdout ----
diff of stderr:

+ error: type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
+    |
+    |
+ LL |   fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
+ LL | |
+ LL | |
+ LL | |
+ LL | |     ()
+ LL | |     ()
+ LL | | }
+    | |_^
+ 
+ error: type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
+    |
+    |
+ LL |   fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
+ LL | |
+ LL | |
+ LL | |
+ LL | |     ()
---
-   --> $DIR/bound_reduction2.rs:16:46
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/bound_reduction2.rs:16:1
3    |
4 LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
6    |
7 note: used non-generic type `<T as TraitWithAssoc>::Assoc` for generic parameter

11    |          ^
12 
13 error: non-defining opaque type use in defining scope
13 error: non-defining opaque type use in defining scope
-   --> $DIR/bound_reduction2.rs:16:46
+   --> $DIR/bound_reduction2.rs:16:1
15    |
16 LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
18    |
18    |
19 note: used non-generic type `_` for generic parameter

23    |          ^
24 
25 error: non-defining opaque type use in defining scope
25 error: non-defining opaque type use in defining scope
-   --> $DIR/bound_reduction2.rs:16:46
+   --> $DIR/bound_reduction2.rs:16:1
27    |
28 LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
+    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
30    |
30    |
31 note: used non-generic type `_` for generic parameter


40 LL | type Foo<V> = impl Trait<V>;
42 
- error: aborting due to 4 previous errors
+ error: aborting due to 6 previous errors
44 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/bound_reduction2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
   |
   |
LL |   fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
   |  ____________________________________________________________^
LL | |     //~^ ERROR non-defining opaque type use in defining scope
LL | |     //~| ERROR non-defining opaque type use in defining scope
LL | |     //~| ERROR non-defining opaque type use in defining scope
LL | |     ()
LL | | }


error: type parameter `T` is part of concrete type but not used in parameter list for the `impl Trait` type alias
   |
   |
LL |   fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
   |  ____________________________________________________________^
LL | |     //~^ ERROR non-defining opaque type use in defining scope
LL | |     //~| ERROR non-defining opaque type use in defining scope
LL | |     //~| ERROR non-defining opaque type use in defining scope
LL | |     ()
LL | | }

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs:16:1
   |
   |
LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
   |
   |
note: used non-generic type `<T as TraitWithAssoc>::Assoc` for generic parameter
   |
   |
LL | type Foo<V> = impl Trait<V>;

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs:16:1
   |
   |
LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
   |
   |
note: used non-generic type `_` for generic parameter
   |
   |
LL | type Foo<V> = impl Trait<V>;

error: non-defining opaque type use in defining scope
  --> /checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs:16:1
   |
   |
LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
   |
   |
note: used non-generic type `_` for generic parameter
   |
   |
LL | type Foo<V> = impl Trait<V>;

error: could not find defining uses
  --> /checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs:9:15
   |
   |
LL | type Foo<V> = impl Trait<V>;

error: aborting due to 6 previous errors


---
test result: FAILED. 12050 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 124.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:55
