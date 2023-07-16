plain
---- [ui] src/test/ui/aligned_enum_cast.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/aligned_enum_cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size { raw: 1 }`,
 right: `Size { raw: 8 }`: abi::Scalar size does not match layout size', /checkout/compiler/rustc_const_eval/src/interpret/operand.rs:277:17
stack backtrace:
   0:     0x7f501eab989c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08db31968c59cd2b
   1:     0x7f501eb1f958 - core::fmt::write::hc5632aa350849b29
   2:     0x7f501eaa9801 - std::io::Write::write_fmt::h6e848e19b2636a6c
   3:     0x7f501eabc8ce - std::panicking::default_hook::{{closure}}::h94f4b616e9c404d7
   4:     0x7f501eabc4fc - std::panicking::default_hook::h9854ecd4ebe0528a
   5:     0x7f501f612511 - rustc_driver[2fed6fcadfedae5]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f501eabd12e - std::panicking::rust_panic_with_hook::h02cb1f0ca5dc0d35
   7:     0x7f501eabcf27 - std::panicking::begin_panic_handler::{{closure}}::h468c82ff1f834a74
   8:     0x7f501eab9db4 - std::sys_common::backtrace::__rust_end_short_backtrace::h8a06ff610c6db596
   9:     0x7f501eabcc09 - rust_begin_unwind
  10:     0x7f501ea71073 - core::panicking::panic_fmt::h7fcd13e110c9aadd
  11:     0x7f501eb1c02e - core::panicking::assert_failed_inner::h4f6e0d5db92b9953
  12:     0x7f501f25d4bb - core[ceb24926b86f2849]::panicking::assert_failed::<rustc_target[ebd5ab28b27cbc7e]::abi::Size, rustc_target[ebd5ab28b27cbc7e]::abi::Size>
  13:     0x7f501fb0985b - <rustc_const_eval[fd84432c8275fa8a]::interpret::eval_context::InterpCx<rustc_mir_transform[89bcdc5b88e8a80a]::const_prop_lint::ConstPropMachine>>::try_read_immediate
  14:     0x7f501fae8072 - <rustc_const_eval[fd84432c8275fa8a]::interpret::eval_context::InterpCx<rustc_mir_transform[89bcdc5b88e8a80a]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  15:     0x7f501fc3f923 - <rustc_mir_transform[89bcdc5b88e8a80a]::const_prop_lint::ConstPropagator as rustc_middle[d1a84f18fd605ef]::mir::visit::Visitor>::visit_statement
  16:     0x7f501fc3f3bf - <rustc_mir_transform[89bcdc5b88e8a80a]::const_prop_lint::ConstPropagator as rustc_middle[d1a84f18fd605ef]::mir::visit::Visitor>::visit_body
  17:     0x7f501fc3ba08 - <rustc_mir_transform[89bcdc5b88e8a80a]::const_prop_lint::ConstProp as rustc_mir_transform[89bcdc5b88e8a80a]::pass_manager::MirLint>::run_lint
  18:     0x7f501fcb976c - rustc_mir_transform[89bcdc5b88e8a80a]::pass_manager::run_passes
  19:     0x7f501fc7f7f5 - rustc_mir_transform[89bcdc5b88e8a80a]::run_post_borrowck_cleanup_passes
  20:     0x7f501fc7f069 - rustc_mir_transform[89bcdc5b88e8a80a]::mir_drops_elaborated_and_const_checked
  21:     0x7f5020cbaca8 - rustc_query_system[e9a1cafc088d7b72]::query::plumbing::try_execute_query::<rustc_query_impl[52398d7834588e4b]::plumbing::QueryCtxt, rustc_query_system[e9a1cafc088d7b72]::query::caches::DefaultCache<rustc_middle[d1a84f18fd605ef]::ty::WithOptConstParam<rustc_span[b9b2a9a7860e582e]::def_id::LocalDefId>, &rustc_data_structures[65707d561bb1f60e]::steal::Steal<rustc_middle[d1a84f18fd605ef]::mir::Body>>>
  22:     0x7f5020dfc3fa - rustc_query_system[e9a1cafc088d7b72]::query::plumbing::get_query::<rustc_query_impl[52398d7834588e4b]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[52398d7834588e4b]::plumbing::QueryCtxt>
  23:     0x7f5021143af6 - <rustc_query_impl[52398d7834588e4b]::Queries as rustc_middle[d1a84f18fd605ef]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24:     0x7f501fc7fd93 - rustc_mir_transform[89bcdc5b88e8a80a]::optimized_mir
  25:     0x7f5020cef0f8 - rustc_query_system[e9a1cafc088d7b72]::query::plumbing::try_execute_query::<rustc_query_impl[52398d7834588e4b]::plumbing::QueryCtxt, rustc_query_system[e9a1cafc088d7b72]::query::caches::DefaultCache<rustc_span[b9b2a9a7860e582e]::def_id::DefId, &rustc_middle[d1a84f18fd605ef]::mir::Body>>
  26:     0x7f5020d96923 - rustc_query_system[e9a1cafc088d7b72]::query::plumbing::get_query::<rustc_query_impl[52398d7834588e4b]::queries::optimized_mir, rustc_query_impl[52398d7834588e4b]::plumbing::QueryCtxt>
  27:     0x7f5021df2abc - <rustc_middle[d1a84f18fd605ef]::ty::context::TyCtxt>::instance_mir
  28:     0x7f501fa914f7 - rustc_monomorphize[c36fd4a08f0a46e9]::collector::collect_neighbours
  29:     0x7f501fa89c55 - rustc_monomorphize[c36fd4a08f0a46e9]::collector::collect_items_rec
  30:     0x7f501faa56f1 - <rustc_session[ccf1c1c903e254bb]::session::Session>::time::<(), rustc_monomorphize[c36fd4a08f0a46e9]::collector::collect_crate_mono_items::{closure#1}>
  31:     0x7f501fa866bf - rustc_monomorphize[c36fd4a08f0a46e9]::collector::collect_crate_mono_items
  32:     0x7f501faa4479 - rustc_monomorphize[c36fd4a08f0a46e9]::partitioning::collect_and_partition_mono_items
  33:     0x7f5020d1166a - rustc_query_system[e9a1cafc088d7b72]::query::plumbing::try_execute_query::<rustc_query_impl[52398d7834588e4b]::plumbing::QueryCtxt, rustc_query_system[e9a1cafc088d7b72]::query::caches::DefaultCache<(), (&std[783f0500e86faa1c]::collections::hash::set::HashSet<rustc_span[b9b2a9a7860e582e]::def_id::DefId, core[ceb24926b86f2849]::hash::BuildHasherDefault<rustc_hash[8f12388d2b110fad]::FxHasher>>, &[rustc_middle[d1a84f18fd605ef]::mir::mono::CodegenUnit])>>
  34:     0x7f5020df5cf5 - rustc_query_system[e9a1cafc088d7b72]::query::plumbing::get_query::<rustc_query_impl[52398d7834588e4b]::queries::collect_and_partition_mono_items, rustc_query_impl[52398d7834588e4b]::plumbing::QueryCtxt>
  35:     0x7f5021145082 - <rustc_query_impl[52398d7834588e4b]::Queries as rustc_middle[d1a84f18fd605ef]::ty::query::QueryEngine>::collect_and_partition_mono_items
  36:     0x7f501f84f030 - rustc_codegen_ssa[fb92c178210dc226]::base::codegen_crate::<rustc_codegen_llvm[5c839074fc61dc4c]::LlvmCodegenBackend>
  37:     0x7f501f896d4b - <rustc_codegen_llvm[5c839074fc61dc4c]::LlvmCodegenBackend as rustc_codegen_ssa[fb92c178210dc226]::traits::backend::CodegenBackend>::codegen_crate
  38:     0x7f501f7235b1 - <rustc_session[ccf1c1c903e254bb]::session::Session>::time::<alloc[cec04d996afa36d4]::boxed::Box<dyn core[ceb24926b86f2849]::any::Any>, rustc_interface[eb4cc42f508a2d4]::passes::start_codegen::{closure#0}>
  39:     0x7f501f70a904 - <rustc_interface[eb4cc42f508a2d4]::passes::QueryContext>::enter::<<rustc_interface[eb4cc42f508a2d4]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ceb24926b86f2849]::result::Result<alloc[cec04d996afa36d4]::boxed::Box<dyn core[ceb24926b86f2849]::any::Any>, rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>
  40:     0x7f501f6f5eee - <rustc_interface[eb4cc42f508a2d4]::queries::Queries>::ongoing_codegen
  41:     0x7f501f5a32e0 - <rustc_interface[eb4cc42f508a2d4]::interface::Compiler>::enter::<rustc_driver[2fed6fcadfedae5]::run_compiler::{closure#1}::{closure#2}, core[ceb24926b86f2849]::result::Result<core[ceb24926b86f2849]::option::Option<rustc_interface[eb4cc42f508a2d4]::queries::Linker>, rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>
  42:     0x7f501f5ff6cb - rustc_span[b9b2a9a7860e582e]::with_source_map::<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, rustc_interface[eb4cc42f508a2d4]::interface::create_compiler_and_run<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, rustc_driver[2fed6fcadfedae5]::run_compiler::{closure#1}>::{closure#1}>
  43:     0x7f501f5a4489 - <scoped_tls[8272629d2af5bc7d]::ScopedKey<rustc_span[b9b2a9a7860e582e]::SessionGlobals>>::set::<rustc_interface[eb4cc42f508a2d4]::interface::run_compiler<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, rustc_driver[2fed6fcadfedae5]::run_compiler::{closure#1}>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>
  44:     0x7f501f5f87d9 - std[783f0500e86faa1c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[eb4cc42f508a2d4]::util::run_in_thread_pool_with_globals<rustc_interface[eb4cc42f508a2d4]::interface::run_compiler<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, rustc_driver[2fed6fcadfedae5]::run_compiler::{closure#1}>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>
  45:     0x7f501f5a5cfe - std[783f0500e86faa1c]::panicking::try::<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, core[ceb24926b86f2849]::panic::unwind_safe::AssertUnwindSafe<<std[783f0500e86faa1c]::thread::Builder>::spawn_unchecked_<rustc_interface[eb4cc42f508a2d4]::util::run_in_thread_pool_with_globals<rustc_interface[eb4cc42f508a2d4]::interface::run_compiler<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, rustc_driver[2fed6fcadfedae5]::run_compiler::{closure#1}>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x7f501f5f94e2 - <<std[783f0500e86faa1c]::thread::Builder>::spawn_unchecked_<rustc_interface[eb4cc42f508a2d4]::util::run_in_thread_pool_with_globals<rustc_interface[eb4cc42f508a2d4]::interface::run_compiler<core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>, rustc_driver[2fed6fcadfedae5]::run_compiler::{closure#1}>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>::{closure#0}, core[ceb24926b86f2849]::result::Result<(), rustc_errors[bec18ab4b38278a0]::ErrorGuaranteed>>::{closure#1} as core[ceb24926b86f2849]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f501eac83e3 - std::sys::unix::thread::Thread::new::thread_start::h9c687204712ea762
  48:     0x7f501901b609 - start_thread
  49:     0x7f501e92e163 - clone
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (b3677c165 2022-04-19) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------



