plain
---- [ui] src/test/ui/mir/issue-67639-normalization-ice.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-67639-normalization-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-67639-normalization-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-67639-normalization-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:199:90: Failed to normalize alloc::raw_vec::RawVec<<<I as Foo>::Item as Bar>::Item2>, maybe try to call `try_normalize_erasing_regions` instead
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1453:9
stack backtrace:
stack backtrace:
   0:     0x7fe1971ef12c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8e863e42dab07d41
   1:     0x7fe197257da8 - core::fmt::write::ha762e9c3ab0cbfc0
   2:     0x7fe1971df901 - std::io::Write::write_fmt::h379c30b49ccf0ce1
   3:     0x7fe1971f211e - std::panicking::default_hook::{{closure}}::h49604e0067c1f6f5
   4:     0x7fe1971f1de7 - std::panicking::default_hook::h0e9d33491d599781
   5:     0x7fe197b83b34 - rustc_driver[a18928092135fc2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fe1971f28d1 - std::panicking::rust_panic_with_hook::h4b818f5be515fd0b
   7:     0x7fe19a61bd03 - std[c943a0dae08fa5b6]::panicking::begin_panic::<rustc_errors[dd450c0a4f25ee80]::ExplicitBug>::{closure#0}
   8:     0x7fe19a61b3e6 - std[c943a0dae08fa5b6]::sys_common::backtrace::__rust_end_short_backtrace::<std[c943a0dae08fa5b6]::panicking::begin_panic<rustc_errors[dd450c0a4f25ee80]::ExplicitBug>::{closure#0}, !>
   9:     0x7fe197b1c286 - std[c943a0dae08fa5b6]::panicking::begin_panic::<rustc_errors[dd450c0a4f25ee80]::ExplicitBug>
  10:     0x7fe19a6925c6 - std[c943a0dae08fa5b6]::panic::panic_any::<rustc_errors[dd450c0a4f25ee80]::ExplicitBug>
  11:     0x7fe19a689a16 - <rustc_errors[dd450c0a4f25ee80]::HandlerInner>::bug::<&alloc[5a6b3e9d90f203bb]::string::String>
  12:     0x7fe19a689510 - <rustc_errors[dd450c0a4f25ee80]::Handler>::bug::<&alloc[5a6b3e9d90f203bb]::string::String>
  13:     0x7fe19a72251e - rustc_middle[7db2bff75237ef10]::ty::context::tls::with_context_opt::<rustc_middle[7db2bff75237ef10]::ty::context::tls::with_opt<rustc_middle[7db2bff75237ef10]::util::bug::opt_span_bug_fmt<rustc_span[e90df5a5c5d80a9f]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  14:     0x7fe19a7248f9 - rustc_middle[7db2bff75237ef10]::util::bug::opt_span_bug_fmt::<rustc_span[e90df5a5c5d80a9f]::span_encoding::Span>
  15:     0x7fe197b261b8 - rustc_middle[7db2bff75237ef10]::util::bug::bug_fmt
  16:     0x7fe19a641e41 - <rustc_middle[7db2bff75237ef10]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  17:     0x7fe19a629156 - <rustc_middle[7db2bff75237ef10]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[7db2bff75237ef10]::ty::fold::TypeFolder>::fold_ty
  18:     0x7fe19814272c - <rustc_middle[7db2bff75237ef10]::ty::context::TyCtxt>::normalize_erasing_regions::<rustc_middle[7db2bff75237ef10]::ty::Ty>
  19:     0x7fe198148a48 - rustc_mir_dataflow[e999e2fc260535a2]::value_analysis::iter_fields::<<rustc_mir_dataflow[e999e2fc260535a2]::value_analysis::Map>::register_with_filter_rec<<rustc_mir_transform[d934cff822f16f0e]::dataflow_const_prop::DataflowConstProp as rustc_middle[7db2bff75237ef10]::mir::MirPass>::run_pass::{closure#0}>::{closure#0}>
  20:     0x7fe19813da2a - <rustc_mir_dataflow[e999e2fc260535a2]::value_analysis::Map>::register_with_filter_rec::<<rustc_mir_transform[d934cff822f16f0e]::dataflow_const_prop::DataflowConstProp as rustc_middle[7db2bff75237ef10]::mir::MirPass>::run_pass::{closure#0}>
  21:     0x7fe19813d867 - <rustc_mir_dataflow[e999e2fc260535a2]::value_analysis::Map>::register_with_filter::<rustc_middle[7db2bff75237ef10]::mir::Body, <rustc_mir_transform[d934cff822f16f0e]::dataflow_const_prop::DataflowConstProp as rustc_middle[7db2bff75237ef10]::mir::MirPass>::run_pass::{closure#0}>
  22:     0x7fe1980f30eb - <rustc_mir_transform[d934cff822f16f0e]::dataflow_const_prop::DataflowConstProp as rustc_middle[7db2bff75237ef10]::mir::MirPass>::run_pass
  23:     0x7fe19809d56f - rustc_mir_transform[d934cff822f16f0e]::pass_manager::run_passes
  24:     0x7fe1981cc588 - rustc_mir_transform[d934cff822f16f0e]::optimized_mir
  25:     0x7fe1996f0109 - rustc_query_system[6a4411d05c026908]::query::plumbing::try_execute_query::<rustc_query_impl[5e1e007cd3b1864d]::plumbing::QueryCtxt, rustc_query_system[6a4411d05c026908]::query::caches::DefaultCache<rustc_span[e90df5a5c5d80a9f]::def_id::DefId, &rustc_middle[7db2bff75237ef10]::mir::Body>>
  26:     0x7fe19979cc29 - rustc_query_system[6a4411d05c026908]::query::plumbing::get_query::<rustc_query_impl[5e1e007cd3b1864d]::queries::optimized_mir, rustc_query_impl[5e1e007cd3b1864d]::plumbing::QueryCtxt>
  27:     0x7fe1995e7ed9 - <rustc_query_impl[5e1e007cd3b1864d]::Queries as rustc_middle[7db2bff75237ef10]::ty::query::QueryEngine>::optimized_mir
  28:     0x7fe19a687654 - <rustc_middle[7db2bff75237ef10]::ty::context::TyCtxt>::instance_mir
  29:     0x7fe1981906b2 - <rustc_mir_transform[d934cff822f16f0e]::inline::Inliner>::try_inlining
  30:     0x7fe19818f9cc - <rustc_mir_transform[d934cff822f16f0e]::inline::Inliner>::process_blocks
  31:     0x7fe19818f07d - <rustc_mir_transform[d934cff822f16f0e]::inline::Inline as rustc_middle[7db2bff75237ef10]::mir::MirPass>::run_pass
  32:     0x7fe19809d56f - rustc_mir_transform[d934cff822f16f0e]::pass_manager::run_passes
  33:     0x7fe1981cc240 - rustc_mir_transform[d934cff822f16f0e]::optimized_mir
  34:     0x7fe1996f0109 - rustc_query_system[6a4411d05c026908]::query::plumbing::try_execute_query::<rustc_query_impl[5e1e007cd3b1864d]::plumbing::QueryCtxt, rustc_query_system[6a4411d05c026908]::query::caches::DefaultCache<rustc_span[e90df5a5c5d80a9f]::def_id::DefId, &rustc_middle[7db2bff75237ef10]::mir::Body>>
  35:     0x7fe19979cc29 - rustc_query_system[6a4411d05c026908]::query::plumbing::get_query::<rustc_query_impl[5e1e007cd3b1864d]::queries::optimized_mir, rustc_query_impl[5e1e007cd3b1864d]::plumbing::QueryCtxt>
  36:     0x7fe1995e7ed9 - <rustc_query_impl[5e1e007cd3b1864d]::Queries as rustc_middle[7db2bff75237ef10]::ty::query::QueryEngine>::optimized_mir
  37:     0x7fe19a687654 - <rustc_middle[7db2bff75237ef10]::ty::context::TyCtxt>::instance_mir
  38:     0x7fe197fe2ae9 - rustc_monomorphize[93b286a323235d36]::collector::collect_neighbours
  39:     0x7fe197fddc36 - rustc_monomorphize[93b286a323235d36]::collector::collect_items_rec
  40:     0x7fe197fe807d - <core[c8aafe456cafcd54]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d60149a2b2df0665]::sync::par_for_each_in<alloc[5a6b3e9d90f203bb]::vec::Vec<rustc_middle[7db2bff75237ef10]::mir::mono::MonoItem>, rustc_monomorphize[93b286a323235d36]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[c8aafe456cafcd54]::ops::function::FnOnce<()>>::call_once
  41:     0x7fe198022a85 - std[c943a0dae08fa5b6]::panicking::try::<(), core[c8aafe456cafcd54]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[d60149a2b2df0665]::sync::par_for_each_in<alloc[5a6b3e9d90f203bb]::vec::Vec<rustc_middle[7db2bff75237ef10]::mir::mono::MonoItem>, rustc_monomorphize[93b286a323235d36]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  42:     0x7fe197ff99be - rustc_data_structures[d60149a2b2df0665]::sync::par_for_each_in::<alloc[5a6b3e9d90f203bb]::vec::Vec<rustc_middle[7db2bff75237ef10]::mir::mono::MonoItem>, rustc_monomorphize[93b286a323235d36]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  43:     0x7fe1980216a0 - <rustc_session[68a9fd602099ef3]::session::Session>::time::<(), rustc_monomorphize[93b286a323235d36]::collector::collect_crate_mono_items::{closure#1}>
  44:     0x7fe197fdaacf - rustc_monomorphize[93b286a323235d36]::collector::collect_crate_mono_items
  45:     0x7fe197fe9f3a - rustc_monomorphize[93b286a323235d36]::partitioning::collect_and_partition_mono_items
  46:     0x7fe199715981 - rustc_query_system[6a4411d05c026908]::query::plumbing::try_execute_query::<rustc_query_impl[5e1e007cd3b1864d]::plumbing::QueryCtxt, rustc_query_system[6a4411d05c026908]::query::caches::DefaultCache<(), (&std[c943a0dae08fa5b6]::collections::hash::set::HashSet<rustc_span[e90df5a5c5d80a9f]::def_id::DefId, core[c8aafe456cafcd54]::hash::BuildHasherDefault<rustc_hash[c0b04c56704d7008]::FxHasher>>, &[rustc_middle[7db2bff75237ef10]::mir::mono::CodegenUnit])>>
  47:     0x7fe1997e31dc - rustc_query_system[6a4411d05c026908]::query::plumbing::get_query::<rustc_query_impl[5e1e007cd3b1864d]::queries::collect_and_partition_mono_items, rustc_query_impl[5e1e007cd3b1864d]::plumbing::QueryCtxt>
  48:     0x7fe19962aa63 - <rustc_query_impl[5e1e007cd3b1864d]::Queries as rustc_middle[7db2bff75237ef10]::ty::query::QueryEngine>::collect_and_partition_mono_items
  49:     0x7fe197df92ad - rustc_codegen_ssa[eefbd6f6b6e67537]::base::codegen_crate::<rustc_codegen_llvm[2054c5a2d7e436ac]::LlvmCodegenBackend>
  50:     0x7fe197d9ccad - <rustc_codegen_llvm[2054c5a2d7e436ac]::LlvmCodegenBackend as rustc_codegen_ssa[eefbd6f6b6e67537]::traits::backend::CodegenBackend>::codegen_crate
  51:     0x7fe197d107bb - <rustc_session[68a9fd602099ef3]::session::Session>::time::<alloc[5a6b3e9d90f203bb]::boxed::Box<dyn core[c8aafe456cafcd54]::any::Any>, rustc_interface[c7119b3329294eda]::passes::start_codegen::{closure#0}>
  52:     0x7fe197cd9a25 - <rustc_interface[c7119b3329294eda]::passes::QueryContext>::enter::<<rustc_interface[c7119b3329294eda]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c8aafe456cafcd54]::result::Result<alloc[5a6b3e9d90f203bb]::boxed::Box<dyn core[c8aafe456cafcd54]::any::Any>, rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>
  53:     0x7fe197cbfa7d - <rustc_interface[c7119b3329294eda]::queries::Queries>::ongoing_codegen
  54:     0x7fe197b8bcf3 - <rustc_interface[c7119b3329294eda]::interface::Compiler>::enter::<rustc_driver[a18928092135fc2]::run_compiler::{closure#1}::{closure#2}, core[c8aafe456cafcd54]::result::Result<core[c8aafe456cafcd54]::option::Option<rustc_interface[c7119b3329294eda]::queries::Linker>, rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>
  55:     0x7fe197b6e4a5 - rustc_span[e90df5a5c5d80a9f]::with_source_map::<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_interface[c7119b3329294eda]::interface::create_compiler_and_run<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_driver[a18928092135fc2]::run_compiler::{closure#1}>::{closure#1}>
  56:     0x7fe197b8e331 - rustc_interface[c7119b3329294eda]::interface::create_compiler_and_run::<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_driver[a18928092135fc2]::run_compiler::{closure#1}>
  57:     0x7fe197b70322 - <scoped_tls[24b61ed9afdb20d1]::ScopedKey<rustc_span[e90df5a5c5d80a9f]::SessionGlobals>>::set::<rustc_interface[c7119b3329294eda]::interface::run_compiler<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_driver[a18928092135fc2]::run_compiler::{closure#1}>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>
  58:     0x7fe197be6409 - std[c943a0dae08fa5b6]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c7119b3329294eda]::util::run_in_thread_pool_with_globals<rustc_interface[c7119b3329294eda]::interface::run_compiler<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_driver[a18928092135fc2]::run_compiler::{closure#1}>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>
  59:     0x7fe197b72b3e - std[c943a0dae08fa5b6]::panic::catch_unwind::<core[c8aafe456cafcd54]::panic::unwind_safe::AssertUnwindSafe<<std[c943a0dae08fa5b6]::thread::Builder>::spawn_unchecked_<rustc_interface[c7119b3329294eda]::util::run_in_thread_pool_with_globals<rustc_interface[c7119b3329294eda]::interface::run_compiler<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_driver[a18928092135fc2]::run_compiler::{closure#1}>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>
  60:     0x7fe197beaa30 - <<std[c943a0dae08fa5b6]::thread::Builder>::spawn_unchecked_<rustc_interface[c7119b3329294eda]::util::run_in_thread_pool_with_globals<rustc_interface[c7119b3329294eda]::interface::run_compiler<core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>, rustc_driver[a18928092135fc2]::run_compiler::{closure#1}>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>::{closure#0}, core[c8aafe456cafcd54]::result::Result<(), rustc_errors[dd450c0a4f25ee80]::ErrorGuaranteed>>::{closure#1} as core[c8aafe456cafcd54]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7fe1971ff645 - std::sys::unix::thread::Thread::new::thread_start::h32a42dba374599a5
  62:     0x7fe196f9bb43 - <unknown>
  63:     0x7fe19702da00 - <unknown>
  64:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (61f1fa55f 2022-08-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z mir-opt-level=4
query stack during panic:
#0 [optimized_mir] optimizing MIR for `join_all`
#1 [optimized_mir] optimizing MIR for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
error: aborting due to previous error
------------------------------------------


