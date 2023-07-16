plain
failures:

---- [ui] ui/const-generics/issues/issue-64494.rs#full stdout ----

error in revision `full`: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-64494.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-64494.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-64494.full/auxiliary"
------------------------------------------

------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stderr:
------------------------------------------
thread 'rustc' panicked at 'assertion failed: !value.has_escaping_bound_vars()', /checkout/compiler/rustc_middle/src/ty/sty.rs:968:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (0db9fdb66 2021-06-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'substs of instance DefId(0:4 ~ issue_64494[317d]::Foo::VAL) not normalized for codegen: [^0]', compiler/rustc_middle/src/ty/instance.rs:285:9
stack backtrace:
   0:     0x7f64f9d323c0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h333c6825549d9397
   1:     0x7f64f9da6d90 - core::fmt::write::hf00778d011964c9e
   2:     0x7f64f9d22916 - std::io::Write::write_fmt::h89fc314624c9579a
   3:     0x7f64f9d368d7 - std::panicking::default_hook::{{closure}}::hbf15a66ebffebd57
   4:     0x7f64f9d362d8 - std::panicking::default_hook::h0e8bb5a15afbcabb
   5:     0x7f64fa618161 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hfd32885fb9b93451
   6:     0x7f64f9d37226 - std::panicking::rust_panic_with_hook::h4ccbdc9c788b3b63
   7:     0x7f64f9d36d47 - std::panicking::begin_panic_handler::{{closure}}::h2dbb841b0bf3317b
   8:     0x7f64f9d3287c - std::sys_common::backtrace::__rust_end_short_backtrace::h8efbc358ebde68c4
   9:     0x7f64f9d36ca9 - rust_begin_unwind
  10:     0x7f64f9d36c5b - std::panicking::begin_panic_fmt::h23923b1033bebc2d
  11:     0x7f64fcb2b2bd - rustc_middle::ty::instance::Instance::new::h2c4a2465daf54480
  12:     0x7f64fb473f97 - std::thread::local::LocalKey<T>::with::h1d088cbe93607e6c
  13:     0x7f64fb54e208 - rustc_query_impl::make_query::resolve_instance::ha92ec4d43b597ee0
  14:     0x7f64fb31c206 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::hfd5c34a621ea441a
  15:     0x7f64fb4cf8f8 - rustc_query_impl::Queries::try_collect_active_jobs::hff8443f92b26df3a
  16:     0x7f64fb72a4c1 - rustc_query_system::query::job::print_query_stack::hd372f10cda8b6c6b
  17:     0x7f64fa73c634 - rustc_interface::interface::try_print_query_stack::h0d015def0c0c471b
  18:     0x7f64fa618a29 - rustc_driver::report_ice::he044d5428b66d63e
  19:     0x7f64f9d37226 - std::panicking::rust_panic_with_hook::h4ccbdc9c788b3b63
  20:     0x7f64f9d36d17 - std::panicking::begin_panic_handler::{{closure}}::h2dbb841b0bf3317b
  21:     0x7f64f9d3287c - std::sys_common::backtrace::__rust_end_short_backtrace::h8efbc358ebde68c4
  22:     0x7f64f9d36ca9 - rust_begin_unwind
  23:     0x7f64f9da3341 - core::panicking::panic_fmt::hacef841ef98e28f3
  24:     0x7f64f9da328d - core::panicking::panic::ha8591c28ea9dd8bf
  25:     0x7f64faf9e522 - rustc_middle::ty::sty::Binder<T>::dummy::h2ab55cc45b57e05e
  26:     0x7f64fafa16a1 - rustc_ty_utils::instance::inner_resolve_instance::he556c8b84c148806
  27:     0x7f64fafa0901 - rustc_ty_utils::instance::resolve_instance::h820ed2e692a6f8f0
  28:     0x7f64fb64375b - rustc_data_structures::stack::ensure_sufficient_stack::hb06f5d59b9982548
  29:     0x7f64fb1fa948 - rustc_query_system::query::plumbing::get_query_impl::h7b86a9f517fbd037
  30:     0x7f64fb35e8fe - rustc_query_system::query::plumbing::get_query::h546bfe7dd9e6d6d7
  31:     0x7f64fb4d147e - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance::hd75adcd97f941507
  32:     0x7f64fcb341b6 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::h74de7ed9eac8c19d
  33:     0x7f64fca0bf00 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::h2bc834b5d7f5a76e
  34:     0x7f64fc6b0a8d - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::hc9a23c41230db5cd
  35:     0x7f64fc722505 - rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with::h3f80b5988b65cd75
  36:     0x7f64fc6b0d7f - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const::h075b768ff867a858
  37:     0x7f64faacb1da - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::ha2b54c515ef5cdf3
  38:     0x7f64fa9ebc9e - rustc_infer::infer::InferCtxtBuilder::enter::h72d8cc553e4f7f82
  39:     0x7f64faaa7cc6 - core::ops::function::FnOnce::call_once::h4e5e73d917b6faa9
  40:     0x7f64fb639774 - rustc_data_structures::stack::ensure_sufficient_stack::h6c0da4b464a98402
  41:     0x7f64fb21c7a2 - rustc_query_system::query::plumbing::get_query_impl::hb13f5083b6bc6595
  42:     0x7f64fb35af8b - rustc_query_system::query::plumbing::get_query::h3f5f1eb63c9ea5e7
  43:     0x7f64fb4d0fe2 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions::he0b3f1ab41d5e7d4
  44:     0x7f64fbd48855 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::h579369cc3c343da9
  45:     0x7f64fbd4ad44 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::hd078e5f808a69077
  46:     0x7f64fbcfea12 - rustc_mir::interpret::eval_context::InterpCx<M>::push_stack_frame::hd85d24a576e6cd72
  47:     0x7f64fbbd252b - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::hd79f1b81cd090768
  48:     0x7f64fb64e94e - rustc_data_structures::stack::ensure_sufficient_stack::hf7afb01253db0414
  49:     0x7f64fb1e5955 - rustc_query_system::query::plumbing::get_query_impl::h566b36509e6e9f10
  50:     0x7f64fb377b05 - rustc_query_system::query::plumbing::get_query::hd0b2551509e389e9
  51:     0x7f64fb4d03c6 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h42e89891447b9131
  52:     0x7f64fbbd0848 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::h8c1d831f3c060f42
  53:     0x7f64fb63fd7e - rustc_data_structures::stack::ensure_sufficient_stack::h99ab312149343de3
  54:     0x7f64fb1c7eb5 - rustc_query_system::query::plumbing::get_query_impl::h298258e86252cdec
  55:     0x7f64fb3796f9 - rustc_query_system::query::plumbing::get_query::hda226bffed61e33e
  56:     0x7f64fb4d0426 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::hd335ba1f89ef86b6
  57:     0x7f64fc9e2d80 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::hd8768c412bd0a490
  58:     0x7f64fca0c1ae - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::h2bc834b5d7f5a76e
  59:     0x7f64fc8738f6 - rustc_infer::infer::InferCtxt::const_eval_resolve::h6ed3979d87cb29c0
  60:     0x7f64fc76a676 - rustc_data_structures::stack::ensure_sufficient_stack::hb73e2af36626f411
  61:     0x7f64fc718003 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively::h6c938d99e5734fec
  62:     0x7f64fc7009e4 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively::h3a2e6c13fdac013b
  63:     0x7f64fc63d294 - rustc_infer::infer::InferCtxt::probe::ha2f4df02aebf6d71
  64:     0x7f64fc7185f3 - rustc_trait_selection::traits::select::SelectionContext::evaluate_candidate::hfb9e54450e3a62ad
  65:     0x7f64fc704961 - rustc_trait_selection::traits::select::SelectionContext::evaluate_stack::hd6aeb064931b9fac
  66:     0x7f64fc6c7add - rustc_query_system::dep_graph::graph::DepGraph<K>::with_anon_task::h6eaca0aed8d67217
  67:     0x7f64fc70138f - rustc_trait_selection::traits::select::SelectionContext::evaluate_trait_predicate_recursively::hbd7402c64ed09894
  68:     0x7f64fc76a34f - rustc_data_structures::stack::ensure_sufficient_stack::hb73e2af36626f411
  69:     0x7f64fc718003 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively::h6c938d99e5734fec
  70:     0x7f64fc63e2e9 - rustc_infer::infer::InferCtxt::probe::hcf57364be1dfaf21
  71:     0x7f64fc7001e7 - rustc_trait_selection::traits::select::SelectionContext::predicate_may_hold_fatal::hdfb2f808127d145a
  72:     0x7f64fc6e6f99 - core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::h025bdcbbcfc7befb
  73:     0x7f64fc6a0065 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::hbf5554e025370489
  74:     0x7f64fc73764b - <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold::h5dbf57ded13077a3
  75:     0x7f64fc6e0f99 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold::h7486058c0ab0ed8b
  76:     0x7f64fc6828d8 - rustc_trait_selection::traits::coherence::overlap_within_probe::h8c4efd6d19d44b6b
  77:     0x7f64fc63b1b5 - rustc_infer::infer::InferCtxt::probe_maybe_skip_leak_check::hccd0cf3492646521
  78:     0x7f64fc68244e - rustc_trait_selection::traits::coherence::overlap::hbfec17a188ffa011
  79:     0x7f64fc62f5e1 - rustc_infer::infer::InferCtxtBuilder::enter::h9e0f8a4bbd95683d
  80:     0x7f64fc680fc9 - rustc_trait_selection::traits::coherence::overlapping_impls::hc4c645923e8e357f
  81:     0x7f64fc785b7e - <rustc_middle::traits::specialization_graph::Children as rustc_trait_selection::traits::specialize::specialization_graph::ChildrenExt>::insert::h932e708fc620ccaa
  82:     0x7f64fc7874a9 - <rustc_middle::traits::specialization_graph::Graph as rustc_trait_selection::traits::specialize::specialization_graph::GraphExt>::insert::hf5f78c2a19da3f27
  83:     0x7f64fc69abf1 - rustc_trait_selection::traits::specialize::specialization_graph_provider::h8636d20018e4c69c
  84:     0x7f64fb635a52 - rustc_data_structures::stack::ensure_sufficient_stack::h51b52d9add80dd98
  85:     0x7f64fb1b4141 - rustc_query_system::query::plumbing::get_query_impl::h04b93f2bfeb115fc
  86:     0x7f64fb35b2dd - rustc_query_system::query::plumbing::get_query::h40514d32a036d347
  87:     0x7f64fadaa803 - rustc_typeck::coherence::coherent_trait::hb1a66c85b8964101
  88:     0x7f64fb1cecf6 - rustc_query_system::query::plumbing::get_query_impl::h36e765ceedf99c51
  89:     0x7f64fb37b6fd - rustc_query_system::query::plumbing::get_query::he63701190edeb635
  90:     0x7f64fadad3a2 - rustc_typeck::coherence::check_coherence::h424cb07940570728
  91:     0x7f64fadb6658 - rustc_session::session::Session::track_errors::ha7e331f0f92191d5
  92:     0x7f64fad5481e - rustc_typeck::check_crate::h9df269554b52459d
  93:     0x7f64fa760e86 - rustc_interface::passes::analysis::hc20a186682309dc6
  94:     0x7f64fb230558 - rustc_query_system::query::plumbing::get_query_impl::hc96b1506aae2022a
  95:     0x7f64fb358a29 - rustc_query_system::query::plumbing::get_query::h3a00e29af4c9191d
  96:     0x7f64fa68ea1e - rustc_interface::passes::QueryContext::enter::he2fba39a2702d51d
  97:     0x7f64fa66c597 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h1f69bb3340a594d1
  98:     0x7f64fa630daa - rustc_span::with_source_map::h32bfc03d19b1b56a
  99:     0x7f64fa66d5ec - rustc_interface::interface::create_compiler_and_run::hdd3bd6116ea355cc
 100:     0x7f64fa631ac2 - rustc_span::with_session_globals::hdfce748773020655
 101:     0x7f64fa68f030 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd2c583e5852843d7
 102:     0x7f64fa68f4d6 - std::panicking::try::hbc4e36f445552ae1
 103:     0x7f64fa61aaea - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf92dd1019df03773
 104:     0x7f64f9d443ea - std::sys::unix::thread::Thread::new::thread_start::h2bed6ca1ba9b33fb
 105:     0x7f64f49fe6db - start_thread
 106:     0x7f64f99ca71f - __clone
 107:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (0db9fdb66 2021-06-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 11927 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 121.00s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:10
