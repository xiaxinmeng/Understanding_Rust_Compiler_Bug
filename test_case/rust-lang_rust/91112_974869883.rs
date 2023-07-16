plain
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 149 tests
.....................................F........F........FFF.......................................... 100/149
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] incremental/hashes/extern_mods.rs stdout ----
---- [incremental] incremental/hashes/extern_mods.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `hir_owner_nodes()` should be clean but is not
   |
LL | / extern "C" {
LL | / extern "C" {
LL | |     pub fn change_function_name2(c: i64) -> i32;
LL | | }


error: `hir_owner_nodes()` should be clean but is not
   |
   |
LL | / extern "rust-call" {
LL | |     pub fn change_calling_convention(c: i32);
LL | | }


error: `hir_owner_nodes()` should be clean but is not
   |
LL | / extern "C" {
LL | / extern "C" {
LL | |     pub fn add_function1(c: i32);
LL | |     pub fn add_function2();
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [incremental] incremental/hashes/inherent_impls.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn method_name2() { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir,promoted_mir,typeck")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner_nodes,optimized_mir,promoted_mir,typeck")]
LL | |     }
LL | | }
   | |_^


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(
LL | |         cfg="cfail2",
LL | |         except="hir_owner_nodes,optimized_mir,promoted_mir,typeck"
LL | |     }
LL | | }
   | |_^


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(
LL | |         cfg="cfail2",
LL | |         except="hir_owner,hir_owner_nodes,fn_sig,generics_of,typeck,associated_item,optimized_mir",
...  |
LL | |     pub fn method_selfness(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,fn_sig,typeck,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner,hir_owner_nodes,fn_sig,typeck,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn method_selfmutness(&mut self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5")]
...  |
LL | |     pub fn add_method_to_impl2(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,fn_sig,typeck,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner,hir_owner_nodes,fn_sig,typeck,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn add_method_parameter(&self, _: i32) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner_nodes,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn change_method_parameter_name(&self, b: i64) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,fn_sig,optimized_mir,typeck")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner,hir_owner_nodes,fn_sig,optimized_mir,typeck")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn change_method_return_type(&self) -> u32 { 0 }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5")]
...  |
LL | |     pub fn make_method_inline(&self) -> u8 { 0 }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner_nodes,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner_nodes,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn change_method_parameter_order(&self, b: i64, a: i64) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,fn_sig,typeck,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner,hir_owner_nodes,fn_sig,typeck,optimized_mir")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub unsafe fn make_method_unsafe(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,fn_sig,typeck")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner,hir_owner_nodes,fn_sig,typeck")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub extern "C" fn make_method_extern(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="hir_owner,hir_owner_nodes,fn_sig,typeck")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="hir_owner,hir_owner_nodes,fn_sig,typeck")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub extern "system" fn change_method_calling_convention(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     // Warning: Note that `typeck` are coming up clean here.
LL | |     // The addition or removal of lifetime parameters that don't
LL | |     // appear in the arguments or fn body in any way does not, in
...  |
LL | |     pub fn add_lifetime_parameter_to_method<'a>(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     // Warning: Note that `typeck` are coming up clean here.
LL | |     // The addition or removal of type parameters that don't appear in
LL | |     // the arguments or fn body in any way does not, in fact, affect
...  |
LL | |     pub fn add_type_parameter_to_method<T>(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(
LL | |         cfg="cfail2",
LL | |         except="hir_owner,hir_owner_nodes,generics_of,predicates_of,type_of,fn_sig"
...  |
LL | |     pub fn add_lifetime_bound_to_lifetime_param_of_method<'a, 'b: 'a>(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     // Warning: Note that `typeck` are coming up clean here.
LL | |     // The addition or removal of bounds that don't appear in the
LL | |     // arguments or fn body in any way does not, in fact, affect the
...  |
LL | |     pub fn add_lifetime_bound_to_type_param_of_method<'a, T: 'a>(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     // Warning: Note that `typeck` are coming up clean here.
LL | |     // The addition or removal of bounds that don't appear in the
LL | |     // arguments or fn body in any way does not, in fact, affect the
...  |
LL | |     pub fn add_trait_bound_to_type_param_of_method<T: Clone>(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be dirty but is not
   |
LL | / impl Foo {
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5")]
...  |
LL | |     pub fn add_no_mangle_to_method(&self) { }
LL | | }


error: `hir_owner_nodes(Bar<u64>)` should be clean but is not
   |
   |
LL | / impl Bar<u64> {
LL | |     #[rustc_clean(cfg="cfail2", except="fn_sig,optimized_mir,typeck")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail5", except="fn_sig,optimized_mir,typeck")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     pub fn change_impl_self_type(&self) { }
LL | | }

error: aborting due to 21 previous errors



------------------------------------------


---- [incremental] incremental/hashes/trait_defs.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `hir_owner_nodes(TraitVisibility)` should be clean but is not
   |
   |
LL | pub trait TraitVisibility { }


error: `hir_owner_nodes(TraitUnsafety)` should be clean but is not
   |
   |
LL | unsafe trait TraitUnsafety { }


error: `hir_owner_nodes(TraitAddMethod)` should be clean but is not
   |
   |
LL | / pub trait TraitAddMethod {
LL | |     fn method();
LL | | }


error: `hir_owner_nodes(TraitChangeMethodName)` should be clean but is not
   |
   |
LL | / trait TraitChangeMethodName {
LL | |     fn methodChanged();
LL | | }


error: `hir_owner_nodes(TraitAddReturnType::method)` should be clean but is not
   |
   |
LL |     fn method() -> u32;


error: `hir_owner_nodes(TraitAddParameterToMethod::method)` should be clean but is not
   |
   |
LL |     fn method(a: u32);


error: `hir_owner_nodes(TraitChangeMethodParameterName::method)` should be clean but is not
   |
   |
LL |     fn method(b: u32);


error: `hir_owner_nodes(TraitChangeMethodParameterType::method)` should be clean but is not
   |
   |
LL |     fn method(a: i64);


error: `hir_owner_nodes(TraitChangeMethodParameterTypeRef::method)` should be clean but is not
   |
   |
LL |     fn method(a: &mut i32);


error: `hir_owner_nodes(TraitChangeMethodParametersOrder::method)` should be clean but is not
   |
   |
LL |     fn method(b: i64, a: i32);


error: `hir_owner_nodes(TraitAddMethodAutoImplementation)` should be clean but is not
   |
   |
LL | / trait TraitAddMethodAutoImplementation {
LL | |     #[rustc_clean(except="hir_owner,associated_item", cfg="cfail2")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(except="hir_owner,associated_item", cfg="cfail5")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     fn method() { }
LL | | }


error: `hir_owner_nodes(TraitAddMethodAutoImplementation::method)` should be clean but is not
   |
LL |     fn method() { }
   |     ^^^^^^^^^^^^^^^


error: `hir_owner_nodes(TraitChangeOrderOfMethods)` should be clean but is not
   |
   |
LL | / trait TraitChangeOrderOfMethods {
LL | |     fn method1();
LL | |     fn method0();
LL | | }


error: `hir_owner_nodes(TraitChangeModeSelfRefToMut::method)` should be clean but is not
   |
LL |     fn method(&mut self);
   |     ^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(TraitChangeModeSelfOwnToRef::method)` should be clean but is not
   |
   |
LL |     fn method(&self);


error: `hir_owner_nodes(TraitAddUnsafeModifier::method)` should be clean but is not
   |
   |
LL |     unsafe fn method();


error: `hir_owner_nodes(TraitAddExternModifier::method)` should be clean but is not
   |
   |
LL |     extern "C" fn method();


error: `hir_owner_nodes(TraitChangeExternCToRustIntrinsic::method)` should be clean but is not
   |
   |
LL |     extern "stdcall" fn method();


error: `hir_owner_nodes(TraitAddTypeParameterToMethod::method)` should be clean but is not
   |
   |
LL |     fn method<T>();


error: `hir_owner_nodes(TraitAddLifetimeParameterToMethod::method)` should be clean but is not
   |
   |
LL |     fn method<'a>();


error: `hir_owner_nodes(TraitAddAssociatedType)` should be clean but is not
   |
   |
LL | / trait TraitAddAssociatedType {
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(cfg="cfail6")]
LL | |     type Associated;
...  |
LL | |     fn method();
LL | | }


error: `hir_owner_nodes(TraitAddTraitBoundToAssociatedType::Associated)` should be clean but is not
   |
LL |     type Associated: ReferencedTrait0;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(TraitAddLifetimeBoundToAssociatedType::Associated)` should be clean but is not
   |
LL |     type Associated: 'a;
   |     ^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(TraitAddDefaultToAssociatedType)` should be clean but is not
   |
   |
LL | / trait TraitAddDefaultToAssociatedType {
LL | |     #[rustc_clean(except="hir_owner,associated_item", cfg="cfail2")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     #[rustc_clean(except="hir_owner,associated_item", cfg="cfail5")]
...  |
LL | |     fn method();
LL | | }


error: `hir_owner_nodes(TraitAddDefaultToAssociatedType::Associated)` should be clean but is not
   |
---
test result: FAILED. 144 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 13.68s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:24
