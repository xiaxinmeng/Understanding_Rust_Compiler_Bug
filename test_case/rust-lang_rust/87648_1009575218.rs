plain
failures:

---- [ui] ui/associated-consts/assoc-const.rs stdout ----

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: test compilation failed although it shouldn't!
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/assoc-const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/assoc-const/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'missing associated type', compiler/rustc_typeck/src/astconv/mod.rs:1133:14

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (0be1ffc2a 2022-01-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [explicit_predicates_of] computing explicit predicates of `foo`
#1 [predicates_defined_on] computing predicates of `foo`
end of query stack
error: internal compiler error: no arg matching AnonConst in path
   |
   |
LL | fn foo<F: Foo<N=3>>() {}
   |
   = note: delayed at compiler/rustc_typeck/src/collect/type_of.rs:187:34


error: internal compiler error: unexpected const parent in type_of(): TraitRef(TraitRef { path: Path { span: /checkout/src/test/ui/associated-consts/assoc-const.rs:16:11: 16:19 (#0), res: Def(Trait, DefId(0:3 ~ assoc_const[43f8]::Foo)), segments: [PathSegment { ident: Foo#0, hir_id: Some(HirId { owner: DefId(0:10 ~ assoc_const[43f8]::foo), local_id: 6 }), res: Some(Def(Trait, DefId(0:3 ~ assoc_const[43f8]::Foo))), args: Some(GenericArgs { args: [], bindings: [TypeBinding { hir_id: HirId { owner: DefId(0:10 ~ assoc_const[43f8]::foo), local_id: 5 }, ident: N#0, gen_args: GenericArgs { args: [], bindings: [], parenthesized: false, span_ext: no-location (#0) }, kind: Equality { term: Const(AnonConst { hir_id: HirId { owner: DefId(0:10 ~ assoc_const[43f8]::foo), local_id: 3 }, body: BodyId { hir_id: HirId { owner: DefId(0:10 ~ assoc_const[43f8]::foo), local_id: 4 } } }) }, span: /checkout/src/test/ui/associated-consts/assoc-const.rs:16:15: 16:18 (#0) }], parenthesized: false, span_ext: /checkout/src/test/ui/associated-consts/assoc-const.rs:16:14: 16:19 (#0) }), infer_args: false }] }, hir_ref_id: HirId { owner: DefId(0:10 ~ assoc_const[43f8]::foo), local_id: 7 } })
   = note: delayed at compiler/rustc_typeck/src/collect/type_of.rs:523:26


error: internal compiler error: Const::from_anon_const: couldn't lit_to_const TypeError
   |
   |
LL | fn foo<F: Foo<N=3>>() {}
   |
   = note: delayed at compiler/rustc_middle/src/ty/consts.rs:94:30


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1188:13
stack backtrace:
   0:     0x7fa786107a10 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc056d55c3f454599
   1:     0x7fa786175e3f - core::fmt::write::ha1972d3b6c553e48
   2:     0x7fa7860f42c5 - std::io::Write::write_fmt::h48ee34f79474b7e0
   3:     0x7fa78610c427 - std::panicking::default_hook::{{closure}}::h2e1ec832054830dc
   4:     0x7fa78610be50 - std::panicking::default_hook::hfdea267e4db88ec0
   5:     0x7fa786b6bf91 - rustc_driver[2550728c433fb0ad]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fa78610cb8b - std::panicking::rust_panic_with_hook::h98527b96c4d7721b
   7:     0x7fa78610c9f0 - std::panicking::begin_panic_handler::{{closure}}::he11267d072718f18
   8:     0x7fa786107f04 - std::sys_common::backtrace::__rust_end_short_backtrace::hc91a3998aeab7606
   9:     0x7fa78610c709 - rust_begin_unwind
  10:     0x7fa7860bf831 - core::panicking::panic_fmt::h4bb412419e219f1e
  11:     0x7fa789510008 - core[c67f74300e0198a3]::panicking::panic_display::<&str>
  12:     0x7fa78950cddf - <rustc_errors[fe4764ac605f8723]::HandlerInner>::flush_delayed
  13:     0x7fa789509966 - <rustc_errors[fe4764ac605f8723]::HandlerInner as core[c67f74300e0198a3]::ops::drop::Drop>::drop
  14:     0x7fa786b53f32 - core[c67f74300e0198a3]::ptr::drop_in_place::<rustc_session[38755cf0e4a0d2f4]::parse::ParseSess>
  15:     0x7fa786b598fa - <alloc[2ba61a082bca1943]::rc::Rc<rustc_session[38755cf0e4a0d2f4]::session::Session> as core[c67f74300e0198a3]::ops::drop::Drop>::drop
  16:     0x7fa786b73cfc - core[c67f74300e0198a3]::ptr::drop_in_place::<rustc_interface[202a996f02b1553]::interface::Compiler>
  17:     0x7fa786b77a90 - rustc_span[d02d167932e55a48]::with_source_map::<core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>, rustc_interface[202a996f02b1553]::interface::create_compiler_and_run<core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>, rustc_driver[2550728c433fb0ad]::run_compiler::{closure#1}>::{closure#1}>
  18:     0x7fa786bd104d - rustc_interface[202a996f02b1553]::interface::create_compiler_and_run::<core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>, rustc_driver[2550728c433fb0ad]::run_compiler::{closure#1}>
  19:     0x7fa786b7e9d4 - std[76b09e2d6e7bbbd4]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[202a996f02b1553]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[202a996f02b1553]::interface::run_compiler<core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>, rustc_driver[2550728c433fb0ad]::run_compiler::{closure#1}>::{closure#0}, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>::{closure#0}, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>
  20:     0x7fa786bd71d1 - std[76b09e2d6e7bbbd4]::panic::catch_unwind::<core[c67f74300e0198a3]::panic::unwind_safe::AssertUnwindSafe<<std[76b09e2d6e7bbbd4]::thread::Builder>::spawn_unchecked<rustc_interface[202a996f02b1553]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[202a996f02b1553]::interface::run_compiler<core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>, rustc_driver[2550728c433fb0ad]::run_compiler::{closure#1}>::{closure#0}, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>::{closure#0}, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>::{closure#1}::{closure#0}>, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>
  21:     0x7fa786b5755a - <<std[76b09e2d6e7bbbd4]::thread::Builder>::spawn_unchecked<rustc_interface[202a996f02b1553]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[202a996f02b1553]::interface::run_compiler<core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>, rustc_driver[2550728c433fb0ad]::run_compiler::{closure#1}>::{closure#0}, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>::{closure#0}, core[c67f74300e0198a3]::result::Result<(), rustc_errors[fe4764ac605f8723]::ErrorReported>>::{closure#1} as core[c67f74300e0198a3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:     0x7fa78611abb3 - std::sys::unix::thread::Thread::new::thread_start::ha18977dd7e4088e3
  23:     0x7fa78048a609 - start_thread
  24:     0x7fa785f81293 - clone
  25:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (0be1ffc2a 2022-01-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 12426 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 143.28s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:09
