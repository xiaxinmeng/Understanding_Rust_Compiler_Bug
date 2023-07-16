plain
.................................................................................................... 9300/11702
.................................................................................................... 9400/11702
.................................................................................................... 9500/11702
.............................................i......i............................................... 9600/11702
............................................................................................iiiiiii. 9700/11702
iiiiii.i............................................................................................ 9800/11702
.................................................................................................... 10000/11702
.................................................................................................... 10100/11702
.................................................................................................... 10200/11702
.................................................................................................... 10300/11702
---
Suite("src/test/codegen-units") not skipped for "bootstrap::test::CodegenUnits" -- not in ["src/tools/tidy"]
Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 41 tests
Some tests failed in compiletest suite=codegen-units mode=codegen-units host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
ii..................i.FFF...F..FFFFFFFF.F

---- [codegen-units] codegen-units/partitioning/local-generic.rs stdout ----


The following items were assigned to wrong codegen units:

fn generic::<&str>
  expected: local_generic.volatile[External] 
  actual:   local_generic-cgu.0[External] 

fn generic::<char>
  expected: local_generic.volatile[External] 
  actual:   local_generic-cgu.0[External] 
fn generic::<u32>
fn generic::<u32>
  expected: local_generic.volatile[External] 
  actual:   local_generic-cgu.0[Internal] 
fn generic::<u64>
fn generic::<u64>
  expected: local_generic.volatile[External] 
  actual:   local_generic-cgu.0[External] 

fn mod1::mod1::user
  expected: local_generic-mod1-mod1[Internal] 
  actual:   local_generic-cgu.2[Internal] 
fn mod1::user
fn mod1::user
  expected: local_generic-mod1[Internal] 
  actual:   local_generic-cgu.1[Internal] 
fn mod2::user
fn mod2::user
  expected: local_generic-mod2[Internal] 
  actual:   local_generic-cgu.3[Internal] 
fn user
fn user
  expected: local_generic[Internal] 
  actual:   local_generic-cgu.0[Internal] 
thread '[codegen-units] codegen-units/partitioning/local-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----
---- [codegen-units] codegen-units/partitioning/local-drop-glue.rs stdout ----

The following items were assigned to wrong codegen units:

fn <Struct as std::ops::Drop>::drop
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-cgu.0[External] 
fn mod1::user
fn mod1::user
  expected: local_drop_glue-mod1[External] 
  actual:   local_drop_glue-cgu.1[External] 

fn std::ptr::drop_in_place::<(u32, Struct)> - shim(Some((u32, Struct)))
  expected: local_drop_glue-fallback.cgu[Internal] 
  actual:   local_drop_glue-cgu.1[Internal] 

fn std::ptr::drop_in_place::<Outer> - shim(Some(Outer))
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-cgu.0[Internal] 

fn std::ptr::drop_in_place::<Struct> - shim(Some(Struct))
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-cgu.0[Internal] local_drop_glue-cgu.1[Internal] 

fn std::ptr::drop_in_place::<mod1::Struct2> - shim(Some(mod1::Struct2))
  expected: local_drop_glue-fallback.cgu[External] 
  actual:   local_drop_glue-cgu.1[Internal] 
fn user
fn user
  expected: local_drop_glue[External] 
  actual:   local_drop_glue-cgu.0[External] 
thread '[codegen-units] codegen-units/partitioning/local-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/incremental-merging.rs stdout ----


The following items were assigned to wrong codegen units:

fn aaa::foo
  expected: incremental_merging-aaa--incremental_merging-bbb[External] 
  actual:   incremental_merging-cgu.2[External] 

fn bbb::foo
  expected: incremental_merging-aaa--incremental_merging-bbb[External] 
  actual:   incremental_merging-cgu.2[External] 

fn ccc::foo
  expected: incremental_merging-ccc[External] 
  actual:   incremental_merging-cgu.1[External] 
fn ddd::foo
fn ddd::foo
  expected: incremental_merging-ddd[External] 
  actual:   incremental_merging-cgu.0[External] 
thread '[codegen-units] codegen-units/partitioning/incremental-merging.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs stdout ----


The following items were assigned to wrong codegen units:
fn inline::inlined_function
fn inline::inlined_function
  expected: local_inlining_but_not_all-inline[External] 
  actual:   local_inlining_but_not_all-cgu.0[External] 
fn non_user::baz
fn non_user::baz
  expected: local_inlining_but_not_all-non_user[External] 
  actual:   local_inlining_but_not_all-cgu.1[External] 
fn user1::foo
fn user1::foo
  expected: local_inlining_but_not_all-user1[External] 
  actual:   local_inlining_but_not_all-cgu.2[External] 
fn user2::bar
fn user2::bar
  expected: local_inlining_but_not_all-user2[External] 
  actual:   local_inlining_but_not_all-cgu.3[External] 
thread '[codegen-units] codegen-units/partitioning/local-inlining-but-not-all.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/local-inlining.rs stdout ----


The following items were assigned to wrong codegen units:
fn inline::inlined_function
fn inline::inlined_function
  expected: local_inlining-user1[Internal] local_inlining-user2[Internal] 
  actual:   local_inlining-cgu.1[Internal] local_inlining-cgu.2[Internal] 
fn non_user::baz
fn non_user::baz
  expected: local_inlining-non_user[External] 
  actual:   local_inlining-cgu.0[External] 
fn user1::foo
fn user1::foo
  expected: local_inlining-user1[External] 
  actual:   local_inlining-cgu.1[External] 
fn user2::bar
fn user2::bar
  expected: local_inlining-user2[External] 
  actual:   local_inlining-cgu.2[External] 
thread '[codegen-units] codegen-units/partitioning/local-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/statics.rs stdout ----


The following items were assigned to wrong codegen units:
fn function
fn function
  expected: statics[External] 
  actual:   statics-cgu.0[External] 
fn mod1::function
fn mod1::function
  expected: statics-mod1[External] 
  actual:   statics-cgu.1[External] 
static BAR
static BAR
  expected: statics[Internal] 
  actual:   statics-cgu.0[Internal] 
static FOO
static FOO
  expected: statics[Internal] 
  actual:   statics-cgu.0[Internal] 
static function::BAR
static function::BAR
  expected: statics[Internal] 
  actual:   statics-cgu.0[Internal] 
static function::FOO
static function::FOO
  expected: statics[Internal] 
  actual:   statics-cgu.0[Internal] 
static mod1::BAR
static mod1::BAR
  expected: statics-mod1[Internal] 
  actual:   statics-cgu.1[Internal] 
static mod1::FOO
static mod1::FOO
  expected: statics-mod1[Internal] 
  actual:   statics-cgu.1[Internal] 
static mod1::function::BAR
static mod1::function::BAR
  expected: statics-mod1[Internal] 
  actual:   statics-cgu.1[Internal] 
static mod1::function::FOO
static mod1::function::FOO
  expected: statics-mod1[Internal] 
  actual:   statics-cgu.1[Internal] 
thread '[codegen-units] codegen-units/partitioning/statics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/regular-modules.rs stdout ----


The following items were assigned to wrong codegen units:
fn bar
fn bar
  expected: regular_modules[Internal] 
  actual:   regular_modules-cgu.0[Internal] 
fn foo
fn foo
  expected: regular_modules[Internal] 
  actual:   regular_modules-cgu.0[Internal] 
fn mod1::bar
fn mod1::bar
  expected: regular_modules-mod1[Internal] 
  actual:   regular_modules-cgu.1[Internal] 
fn mod1::foo
fn mod1::foo
  expected: regular_modules-mod1[Internal] 
  actual:   regular_modules-cgu.1[Internal] 

fn mod1::mod1::bar
  expected: regular_modules-mod1-mod1[Internal] 
  actual:   regular_modules-cgu.2[Internal] 

fn mod1::mod1::foo
  expected: regular_modules-mod1-mod1[Internal] 
  actual:   regular_modules-cgu.2[Internal] 

fn mod1::mod2::bar
  expected: regular_modules-mod1-mod2[Internal] 
  actual:   regular_modules-cgu.3[Internal] 

fn mod1::mod2::foo
  expected: regular_modules-mod1-mod2[Internal] 
  actual:   regular_modules-cgu.3[Internal] 
fn mod2::bar
fn mod2::bar
  expected: regular_modules-mod2[Internal] 
  actual:   regular_modules-cgu.4[Internal] 
fn mod2::foo
fn mod2::foo
  expected: regular_modules-mod2[Internal] 
  actual:   regular_modules-cgu.4[Internal] 

fn mod2::mod1::bar
  expected: regular_modules-mod2-mod1[Internal] 
  actual:   regular_modules-cgu.5[Internal] 

fn mod2::mod1::foo
  expected: regular_modules-mod2-mod1[Internal] 
  actual:   regular_modules-cgu.5[Internal] 

fn mod2::mod2::bar
  expected: regular_modules-mod2-mod2[Internal] 
  actual:   regular_modules-cgu.6[Internal] 

fn mod2::mod2::foo
  expected: regular_modules-mod2-mod2[Internal] 
  actual:   regular_modules-cgu.6[Internal] 
static BAZ
static BAZ
  expected: regular_modules[Internal] 
  actual:   regular_modules-cgu.0[Internal] 

static mod1::BAZ
  expected: regular_modules-mod1[Internal] 
  actual:   regular_modules-cgu.1[Internal] 

static mod1::mod1::BAZ
  expected: regular_modules-mod1-mod1[Internal] 
  actual:   regular_modules-cgu.2[Internal] 

static mod1::mod2::BAZ
  expected: regular_modules-mod1-mod2[Internal] 
  actual:   regular_modules-cgu.3[Internal] 

static mod2::BAZ
  expected: regular_modules-mod2[Internal] 
  actual:   regular_modules-cgu.4[Internal] 

static mod2::mod1::BAZ
  expected: regular_modules-mod2-mod1[Internal] 
  actual:   regular_modules-cgu.5[Internal] 

static mod2::mod2::BAZ
  expected: regular_modules-mod2-mod2[Internal] 
  actual:   regular_modules-cgu.6[Internal] 
thread '[codegen-units] codegen-units/partitioning/regular-modules.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/local-transitive-inlining.rs stdout ----


The following items were assigned to wrong codegen units:
fn direct_user::foo
fn direct_user::foo
  expected: local_transitive_inlining-indirect_user[Internal] 
  actual:   local_transitive_inlining-cgu.0[Internal] 
fn indirect_user::bar
fn indirect_user::bar
  expected: local_transitive_inlining-indirect_user[External] 
  actual:   local_transitive_inlining-cgu.0[External] 
fn inline::inlined_function
fn inline::inlined_function
  expected: local_transitive_inlining-indirect_user[Internal] 
  actual:   local_transitive_inlining-cgu.0[Internal] 
fn non_user::baz
fn non_user::baz
  expected: local_transitive_inlining-non_user[External] 
  actual:   local_transitive_inlining-cgu.1[External] 
thread '[codegen-units] codegen-units/partitioning/local-transitive-inlining.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/extern-drop-glue.rs stdout ----


The following items were assigned to wrong codegen units:
fn mod1::user
fn mod1::user
  expected: extern_drop_glue-mod1[External] 
  actual:   extern_drop_glue-cgu.1[External] 

fn std::ptr::drop_in_place::<LocalStruct> - shim(Some(LocalStruct))
  expected: extern_drop_glue-fallback.cgu[External] 
  actual:   extern_drop_glue-cgu.0[Internal] 

fn std::ptr::drop_in_place::<cgu_extern_drop_glue::Struct> - shim(Some(cgu_extern_drop_glue::Struct))
  expected: extern_drop_glue-fallback.cgu[External] 
  actual:   extern_drop_glue-cgu.0[Internal] extern_drop_glue-cgu.1[Internal] 

fn std::ptr::drop_in_place::<mod1::LocalStruct> - shim(Some(mod1::LocalStruct))
  expected: extern_drop_glue-fallback.cgu[External] 
  actual:   extern_drop_glue-cgu.1[Internal] 
fn user
fn user
  expected: extern_drop_glue[External] 
  actual:   extern_drop_glue-cgu.0[External] 
thread '[codegen-units] codegen-units/partitioning/extern-drop-glue.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs stdout ----


The following items were assigned to wrong codegen units:

fn cgu_explicit_inlining::always_inlined
  expected: inlining_from_extern_crate-mod2[Internal] inlining_from_extern_crate[Internal] 
  actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.2[Internal] 

fn cgu_explicit_inlining::inlined
  expected: inlining_from_extern_crate-mod1[Internal] inlining_from_extern_crate[Internal] 
  actual:   inlining_from_extern_crate-cgu.0[Internal] inlining_from_extern_crate-cgu.1[Internal] 
fn mod1::user
fn mod1::user
  expected: inlining_from_extern_crate-mod1[External] 
  actual:   inlining_from_extern_crate-cgu.1[External] 
fn mod2::user
fn mod2::user
  expected: inlining_from_extern_crate-mod2[External] 
  actual:   inlining_from_extern_crate-cgu.2[External] 
fn user
fn user
  expected: inlining_from_extern_crate[External] 
  actual:   inlining_from_extern_crate-cgu.0[External] 
thread '[codegen-units] codegen-units/partitioning/inlining-from-extern-crate.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/extern-generic.rs stdout ----


The following items were assigned to wrong codegen units:

fn cgu_generic_function::bar::<&str>
  expected: cgu_generic_function-in-extern_generic.volatile[External] 
  actual:   extern_generic-cgu.0[External] 

fn cgu_generic_function::foo::<&str>
  expected: cgu_generic_function-in-extern_generic.volatile[External] 
  actual:   extern_generic-cgu.0[External] 

fn mod1::mod1::user
  expected: extern_generic-mod1-mod1[Internal] 
  actual:   extern_generic-cgu.3[Internal] 
fn mod1::user
fn mod1::user
  expected: extern_generic-mod1[Internal] 
  actual:   extern_generic-cgu.2[Internal] 
fn mod2::user
fn mod2::user
  expected: extern_generic-mod2[Internal] 
  actual:   extern_generic-cgu.4[Internal] 
fn mod3::non_user
fn mod3::non_user
  expected: extern_generic-mod3[Internal] 
  actual:   extern_generic-cgu.5[Internal] 
fn user
fn user
  expected: extern_generic[Internal] 
  actual:   extern_generic-cgu.1[Internal] 
thread '[codegen-units] codegen-units/partitioning/extern-generic.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/vtable-through-const.rs stdout ----


The following items were assigned to wrong codegen units:

fn <u32 as mod1::Trait1>::do_something
  expected: vtable_through_const-mod1.volatile[External] 
  actual:   vtable_through_const-cgu.1[External] 

fn <u32 as mod1::Trait1>::do_something_else
  expected: vtable_through_const-mod1.volatile[External] 
  actual:   vtable_through_const-cgu.1[External] 

fn <u32 as mod1::Trait1Gen<u8>>::do_something
  expected: vtable_through_const-mod1.volatile[External] 
  actual:   vtable_through_const-cgu.1[External] 

fn <u32 as mod1::Trait1Gen<u8>>::do_something_else
  expected: vtable_through_const-mod1.volatile[External] 
  actual:   vtable_through_const-cgu.1[External] 

fn <u32 as mod1::Trait2>::do_something
  expected: vtable_through_const-mod1.volatile[Internal] 
  actual:   vtable_through_const-cgu.1[Internal] 

fn <u32 as mod1::Trait2>::do_something_else
  expected: vtable_through_const-mod1.volatile[Internal] 
  actual:   vtable_through_const-cgu.1[Internal] 

fn <u32 as mod1::Trait2Gen<u8>>::do_something
  expected: vtable_through_const-mod1.volatile[Internal] 
  actual:   vtable_through_const-cgu.1[Internal] 

fn <u32 as mod1::Trait2Gen<u8>>::do_something_else
  expected: vtable_through_const-mod1.volatile[Internal] 
  actual:   vtable_through_const-cgu.1[Internal] 

fn mod1::id::<char>
  expected: vtable_through_const-mod1.volatile[External] 
  actual:   vtable_through_const-cgu.1[External] 

fn mod1::id::<i64>
  expected: vtable_through_const-mod1.volatile[Internal] 
  actual:   vtable_through_const-cgu.1[Internal] 

fn std::ptr::drop_in_place::<u32> - shim(None)
  expected: vtable_through_const[Internal] 
  actual:   vtable_through_const-cgu.0[Internal] 
thread '[codegen-units] codegen-units/partitioning/vtable-through-const.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13

---- [codegen-units] codegen-units/partitioning/shared-generics.rs stdout ----


The following items were assigned to wrong codegen units:

fn shared_generics_aux::generic_fn::<u16>
  expected: shared_generics_aux-in-shared_generics.volatile[External] 
  actual:   shared_generics-cgu.1[External] 
thread '[codegen-units] codegen-units/partitioning/shared-generics.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:2776:13


failures:
---
test result: FAILED. 25 passed; 13 failed; 3 ignored; 0 measured; 0 filtered out; finished in 0.33s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/codegen-units" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen-units" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "codegen-units" "--mode" "codegen-units" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:32
