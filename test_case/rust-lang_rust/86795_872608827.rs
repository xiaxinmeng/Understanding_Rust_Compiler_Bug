plain
failures:

---- [ui] ui/const-generics/issues/issue-64494.rs#full stdout ----

error in revision `full`: Error: expected failure status (Some(1)) but received status None.
status: signal: 4 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-64494.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-64494.full" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-64494.full/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: internal compiler error: compiler/rustc_middle/src/ty/fold.rs:893:25: Mismatched bound variable kinds! Expected type, found Region(BrAnon(0))
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1006:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (353886777 2021-07-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'substs of instance DefId(0:4 ~ issue_64494[317d]::Foo::VAL) not normalized for codegen: [^0]', compiler/rustc_middle/src/ty/instance.rs:285:9
stack backtrace:
   0:     0x7f8dc5c15140 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h0ad22358f179bff1
   1:     0x7f8dc5c89b10 - core::fmt::write::hd3665f81bd96b7f1
   2:     0x7f8dc5c05706 - std::io::Write::write_fmt::h9db6f24a0ab4c728
   3:     0x7f8dc5c19657 - std::panicking::default_hook::{{closure}}::h374101f43fb27a68
   4:     0x7f8dc5c19058 - std::panicking::default_hook::h3ee7b71bf81c784e
   5:     0x7f8dc64fb641 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h0e05d17111133f4d
   6:     0x7f8dc5c19fa6 - std::panicking::rust_panic_with_hook::h75c29b5ad5b07f8c
   7:     0x7f8dc5c19ac7 - std::panicking::begin_panic_handler::{{closure}}::hb20fd4d0247775d2
   8:     0x7f8dc5c155dc - std::sys_common::backtrace::__rust_end_short_backtrace::h87567b7080055fa1
   9:     0x7f8dc5c19a29 - rust_begin_unwind
  10:     0x7f8dc5c199db - std::panicking::begin_panic_fmt::he630a2624d374851
  11:     0x7f8dc8a0e3cd - rustc_middle::ty::instance::Instance::new::hf140b85a1b03406e
  12:     0x7f8dc7372a57 - std::thread::local::LocalKey<T>::with::he6fa8f98b7f77465
  13:     0x7f8dc7440d88 - rustc_query_impl::make_query::resolve_instance::h3decea6b8fda78a1
  14:     0x7f8dc71f7ff6 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h3aa385160e846018
  15:     0x7f8dc73c2758 - rustc_query_impl::Queries::try_collect_active_jobs::h4f0120df8e9b31a0
  16:     0x7f8dc7610991 - rustc_query_system::query::job::print_query_stack::heefa614c57a60652
  17:     0x7f8dc66238f4 - rustc_interface::interface::try_print_query_stack::h7b1b7e7441f5bc06
  18:     0x7f8dc64fbf09 - rustc_driver::report_ice::hc4e2f9e1f5c74864
  19:     0x7f8dc5c19fa6 - std::panicking::rust_panic_with_hook::h75c29b5ad5b07f8c
  20:     0x7f8dc8bd3d7d - std::panicking::begin_panic::{{closure}}::h0badbd3b35fa00df
  21:     0x7f8dc8bd3c66 - std::sys_common::backtrace::__rust_end_short_backtrace::hc6057bab15158849
  22:     0x7f8dc8bd3d1f - std::panicking::begin_panic::h12ece5e5c8025a50
  23:     0x7f8dc8b9967d - std::panic::panic_any::h836ea5603b780e93
  24:     0x7f8dc8b9ed95 - rustc_errors::HandlerInner::bug::had42cea75d0c996f
  25:     0x7f8dc8b9d3b0 - rustc_errors::Handler::bug::h8018da1aae71d501
  26:     0x7f8dc8a94a41 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h931236766824b14a
  27:     0x7f8dc8a8f7ab - rustc_middle::ty::context::tls::with_opt::{{closure}}::h26e47027a402041c
  28:     0x7f8dc8a8f758 - rustc_middle::ty::context::tls::with_opt::h4a88ca868f72ae0f
  29:     0x7f8dc8a94963 - rustc_middle::util::bug::opt_span_bug_fmt::hdcbc0c09dd2caa9f
  30:     0x7f8dc8a948d5 - rustc_middle::util::bug::bug_fmt::ha7aaf94b6afd2c6f
  31:     0x7f8dc8a38eaf - <rustc_middle::ty::fold::ValidateBoundVars as rustc_middle::ty::fold::TypeVisitor>::visit_ty::h98da01cc9208253f
  32:     0x7f8dc6e66c43 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::h55cbb7d301e40d98
  33:     0x7f8dc6e59649 - rustc_middle::ty::fold::TypeFoldable::visit_with::h9d38a489bb7ed7d4
  34:     0x7f8dc6e65627 - rustc_middle::ty::sty::Binder<T>::bind_with_vars::hf59ae7a7cd14ca95
  35:     0x7f8dc6e82c25 - rustc_ty_utils::instance::inner_resolve_instance::hfa28d19df5250c12
  36:     0x7f8dc6e81b91 - rustc_ty_utils::instance::resolve_instance::hb78e3ce4150d005d
  37:     0x7f8dc752e28b - rustc_data_structures::stack::ensure_sufficient_stack::ha98863af9fe9ab74
  38:     0x7f8dc70ecd58 - rustc_query_system::query::plumbing::get_query_impl::h8d6c64997d4b52cc
  39:     0x7f8dc723ab2e - rustc_query_system::query::plumbing::get_query::h33435a334a6b8978
  40:     0x7f8dc73c42de - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::resolve_instance::hf53761e9b9012768
  41:     0x7f8dc8a1ee16 - rustc_middle::ty::instance::Instance::resolve_opt_const_arg::h43fe9a0e5a3aef54
  42:     0x7f8dc88f4db0 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hd7c7391eb4b8d264
  43:     0x7f8dc8595226 - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_const::h3bcee74775a9f11c
  44:     0x7f8dc8620935 - rustc_middle::mir::type_foldable::<impl rustc_middle::ty::fold::TypeFoldable for rustc_middle::mir::ConstantKind>::super_fold_with::h7de4d1a6d99b4a3e
  45:     0x7f8dc859550f - <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_mir_const::hae4a6f288718ba42
  46:     0x7f8dc69aa44a - <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::h91c02b6decb508e8
  47:     0x7f8dc68ca59e - rustc_infer::infer::InferCtxtBuilder::enter::he4b6da4036ac00e4
  48:     0x7f8dc6986f06 - core::ops::function::FnOnce::call_once::h4891a58ba368007d
  49:     0x7f8dc7538b74 - rustc_data_structures::stack::ensure_sufficient_stack::hf2a7db355c3d755b
  50:     0x7f8dc7122b02 - rustc_query_system::query::plumbing::get_query_impl::hddedcfd0e61cac53
  51:     0x7f8dc7249cbb - rustc_query_system::query::plumbing::get_query::h8bfd2792aa3e2049
  52:     0x7f8dc73c3e42 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_mir_const_after_erasing_regions::h9cf88d9ef91b7479
  53:     0x7f8dc7ca5845 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::normalize_erasing_regions::h08fd0149b78737bd
  54:     0x7f8dc7ca8284 - rustc_middle::ty::normalize_erasing_regions::<impl rustc_middle::ty::context::TyCtxt>::subst_and_normalize_erasing_regions::h0f7118e13c971f85
  55:     0x7f8dc7be5672 - rustc_mir::interpret::eval_context::InterpCx<M>::push_stack_frame::h5832c3efc38bf489
  56:     0x7f8dc7abbfdb - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h48102b5a7395a679
  57:     0x7f8dc751eb7e - rustc_data_structures::stack::ensure_sufficient_stack::h43f3422914525ee2
  58:     0x7f8dc70c0955 - rustc_query_system::query::plumbing::get_query_impl::h3d12c438fc200d8b
  59:     0x7f8dc72362c5 - rustc_query_system::query::plumbing::get_query::h1e74956c164c48cd
  60:     0x7f8dc73c3226 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_allocation_raw::h12cde6a08ed9c948
  61:     0x7f8dc7aba2f8 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::hdfa777f137b9d389
  62:     0x7f8dc753057e - rustc_data_structures::stack::ensure_sufficient_stack::hb9cfe13d8b78049e
  63:     0x7f8dc70e1435 - rustc_query_system::query::plumbing::get_query_impl::h7cf2da0b2a502a0c
  64:     0x7f8dc7239199 - rustc_query_system::query::plumbing::get_query::h2c0460e912bc09e7
  65:     0x7f8dc73c3286 - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::eval_to_const_value_raw::hb235697e82c45e22
  66:     0x7f8dc88ca8d0 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::hf2e984f0715606c4
  67:     0x7f8dc88f505e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hd7c7391eb4b8d264
  68:     0x7f8dc8759296 - rustc_infer::infer::InferCtxt::const_eval_resolve::hef426c709502dc8d
  69:     0x7f8dc864fab6 - rustc_data_structures::stack::ensure_sufficient_stack::hd68cf6096fdf321d
  70:     0x7f8dc85fd403 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively::h6f35ed6e7ca04951
  71:     0x7f8dc85e59c4 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicates_recursively::h2b8da36a73693295
  72:     0x7f8dc8520e94 - rustc_infer::infer::InferCtxt::probe::h411745716b6853f8
  73:     0x7f8dc85fd9f3 - rustc_trait_selection::traits::select::SelectionContext::evaluate_candidate::hd6a8cd28ea11fa5a
  74:     0x7f8dc85e9d61 - rustc_trait_selection::traits::select::SelectionContext::evaluate_stack::hed52758421203092
  75:     0x7f8dc85abe5d - rustc_query_system::dep_graph::graph::DepGraph<K>::with_anon_task::h33735733b1090a61
  76:     0x7f8dc85e678f - rustc_trait_selection::traits::select::SelectionContext::evaluate_trait_predicate_recursively::hb506f8e121946493
  77:     0x7f8dc864f78f - rustc_data_structures::stack::ensure_sufficient_stack::hd68cf6096fdf321d
  78:     0x7f8dc85fd403 - rustc_trait_selection::traits::select::SelectionContext::evaluate_predicate_recursively::h6f35ed6e7ca04951
  79:     0x7f8dc8520829 - rustc_infer::infer::InferCtxt::probe::h1e9220b019f02665
  80:     0x7f8dc85e55e7 - rustc_trait_selection::traits::select::SelectionContext::predicate_may_hold_fatal::h41ba208d9fa494a8
  81:     0x7f8dc85ca2e9 - core::ops::function::impls::<impl core::ops::function::FnMut<A> for &mut F>::call_mut::hab0d256e2a9a2a54
  82:     0x7f8dc8575675 - <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold::h22056ff94265eec5
  83:     0x7f8dc86155cb - <core::iter::adapters::chain::Chain<A,B> as core::iter::traits::iterator::Iterator>::try_fold::h7bfd1866d89ced52
  84:     0x7f8dc85c4439 - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::try_fold::h819d8609e50acff9
  85:     0x7f8dc8567c78 - rustc_trait_selection::traits::coherence::overlap_within_probe::he83651ff90a428ee
  86:     0x7f8dc851f3c5 - rustc_infer::infer::InferCtxt::probe_maybe_skip_leak_check::h77b66847516a6c66
  87:     0x7f8dc85677ee - rustc_trait_selection::traits::coherence::overlap::hdd524ff73316ca55
  88:     0x7f8dc8514831 - rustc_infer::infer::InferCtxtBuilder::enter::h989604bad725fa7b
  89:     0x7f8dc85653c9 - rustc_trait_selection::traits::coherence::overlapping_impls::h532d242ea1a829d2
  90:     0x7f8dc866abfe - <rustc_middle::traits::specialization_graph::Children as rustc_trait_selection::traits::specialize::specialization_graph::ChildrenExt>::insert::h5ad07957ae85f829
  91:     0x7f8dc866c529 - <rustc_middle::traits::specialization_graph::Graph as rustc_trait_selection::traits::specialize::specialization_graph::GraphExt>::insert::ha0c1b5d91c613131
  92:     0x7f8dc8582a29 - rustc_trait_selection::traits::specialize::specialization_graph_provider::h1267547a6fcfca31
  93:     0x7f8dc7539552 - rustc_data_structures::stack::ensure_sufficient_stack::hf6b45ea35c5a3d69
  94:     0x7f8dc7096ce1 - rustc_query_system::query::plumbing::get_query_impl::h0378b8274cf9e8ac
  95:     0x7f8dc725373d - rustc_query_system::query::plumbing::get_query::hb8d101facbe0e961
  96:     0x7f8dc6c62aff - rustc_typeck::coherence::coherent_trait::h86e3f36a649ebe0d
  97:     0x7f8dc70f91d6 - rustc_query_system::query::plumbing::get_query_impl::h9c7b13bd34fba840
  98:     0x7f8dc7231a5d - rustc_query_system::query::plumbing::get_query::h0300a87199d2f3df
  99:     0x7f8dc6c6565e - rustc_typeck::coherence::check_coherence::h10a1f89737230025
 100:     0x7f8dc6c94908 - rustc_session::session::Session::track_errors::ha5736c764b095bbf
 101:     0x7f8dc6c697ee - rustc_typeck::check_crate::h25b205119df717ea
 102:     0x7f8dc6639f3a - rustc_interface::passes::analysis::h1f64f7393f4640e3
 103:     0x7f8dc70d7848 - rustc_query_system::query::plumbing::get_query_impl::h6c7236c03a8f126a
 104:     0x7f8dc7237849 - rustc_query_system::query::plumbing::get_query::h23e6348178021f5d
 105:     0x7f8dc657160e - rustc_interface::passes::QueryContext::enter::h38eb884f84b0b71d
 106:     0x7f8dc654f208 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h98fbd93a2bba01ab
 107:     0x7f8dc651567a - rustc_span::with_source_map::hbc89c96ac0eaf237
 108:     0x7f8dc655023c - rustc_interface::interface::create_compiler_and_run::hd03524749164bca9
 109:     0x7f8dc6515e62 - rustc_span::with_session_globals::h9e6c3f9ea9d5bcf3
 110:     0x7f8dc6550c80 - std::sys_common::backtrace::__rust_begin_short_backtrace::haa4b888a76f4c4b9
 111:     0x7f8dc6572136 - std::panicking::try::h5ce532b765bd974a
 112:     0x7f8dc64fdfba - core::ops::function::FnOnce::call_once{{vtable.shim}}::he2bb69d9ca63f50f
 113:     0x7f8dc5c2716a - std::sys::unix::thread::Thread::new::thread_start::h37ca38aa26d012f6
 114:     0x7f8dc08e16db - start_thread
 115:     0x7f8dc58ad71f - __clone
 116:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (353886777 2021-07-01) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

---
test result: FAILED. 11939 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 121.08s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:28
