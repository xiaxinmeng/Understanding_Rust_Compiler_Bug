plain
.................................................................................................... 9500/11892
.................................................................................................... 9600/11892
...................................................................................i......i......... 9700/11892
.................................................................................................... 9800/11892
.............................iiiiiii.iiiiii.i....................................................... 9900/11892
.................................................................................................... 10100/11892
.................................................................................................... 10200/11892
.................................................................................................... 10300/11892
.................................................................................................... 10400/11892
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.278 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 131 tests
........................F...............F....F...................................................... 100/131
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] incremental/hashes/extern_mods.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/extern_mods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/extern_mods.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/extern_mods/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `hir_owner_nodes()` should be clean but is not
   |
   |
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
   |
LL | / extern "C" {
LL | |     pub fn make_function_public(c: i32);
LL | | }


error: `hir_owner_nodes()` should be clean but is not
   |
   |
LL | / extern "C" {
LL | |     pub fn add_function1(c: i32);
LL | |     pub fn add_function2();
LL | | }

error: aborting due to 4 previous errors



------------------------------------------


---- [incremental] incremental/hashes/inherent_impls.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/inherent_impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/inherent_impls.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/inherent_impls/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `hir_owner_nodes(Foo)` should be clean but is not
   |
   |
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     pub fn method_name2() { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
   |
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2", except="associated_item,hir_owner,hir_owner_nodes")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     fn method_privacy() { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
   |
LL | / impl Foo {
LL | |     #[rustc_dirty(cfg="cfail2", except="type_of,predicates_of,promoted_mir")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     pub fn method_selfness(&self) { }
LL | | }


error: `hir_owner_nodes(Foo)` should be clean but is not
   |
   |
LL | / impl Foo {
LL | |     #[rustc_clean(cfg="cfail2")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     pub fn add_method_to_impl1(&self) { }
...  |
LL | |     pub fn add_method_to_impl2(&self) { }
LL | | }


error: `hir_owner_nodes(Bar<u64>)` should be clean but is not
   |
   |
LL | / impl Bar<u64> {
LL | |     #[rustc_clean(cfg="cfail2", except="fn_sig,optimized_mir,typeck")]
LL | |     #[rustc_clean(cfg="cfail3")]
LL | |     pub fn change_impl_self_type(&self) { }
LL | | }

error: aborting due to 5 previous errors



------------------------------------------


---- [incremental] incremental/hashes/type_defs.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/type_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs/type_defs.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/type_defs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `hir_owner_nodes(ChangePrimitiveType)` should be clean but is not
   |
   |
LL | type ChangePrimitiveType = i64;


error: `hir_owner_nodes(ChangeMutability)` should be clean but is not
   |
   |
LL | type ChangeMutability = &'static mut i32;


error: `hir_owner_nodes(ChangeTypeStruct)` should be clean but is not
   |
LL | type ChangeTypeStruct = Struct2;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `hir_owner_nodes(ChangeTypeTuple)` should be clean but is not
   |
   |
LL | type ChangeTypeTuple = (u32, i64);


error: `hir_owner_nodes(ChangeTypeEnum)` should be clean but is not
   |
   |
LL | type ChangeTypeEnum = Enum2;


error: `hir_owner_nodes(AddTupleField)` should be clean but is not
   |
   |
LL | type AddTupleField = (i32, i64, i16);


error: `hir_owner_nodes(ChangeNestedTupleField)` should be clean but is not
   |
   |
LL | type ChangeNestedTupleField = (i32, (i64, i8));

error: aborting due to 7 previous errors


---
test result: FAILED. 128 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.56s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "incremental" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:23:54
