plain
........................................................................................ 12760/13332
........................................................................................ 12848/13332
........................................................................................ 12936/13332
........................................................................................ 13024/13332
....................................................................F.F..F.............. 13112/13332
.........................iii............................................................ 13288/13332
............................................
failures:


---- [ui] src/test/ui/unsized-locals/by-value-trait-object-safety-rpass.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/by-value-trait-object-safety-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'cannot have unsized immediates', /checkout/compiler/rustc_const_eval/src/interpret/place.rs:645:17
stack backtrace:
   0:     0x7ff2c2295f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he2b2813cbab87088
   1:     0x7ff2c22fc9f8 - core::fmt::write::h85053c8e50ac906d
   2:     0x7ff2c22865b1 - std::io::Write::write_fmt::h1c70ced061411a22
   3:     0x7ff2c2298f5e - std::panicking::default_hook::{{closure}}::hf522f9bae8b1958a
   4:     0x7ff2c2298c1f - std::panicking::default_hook::hc3b110f368f13f1a
   5:     0x7ff2c2c50704 - rustc_driver[39f7a1d32b8dfc5d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff2c2299712 - std::panicking::rust_panic_with_hook::h07650724f4d15eb1
   7:     0x7ff2c22994f9 - std::panicking::begin_panic_handler::{{closure}}::ha6ccca2a50a9de21
   8:     0x7ff2c2296514 - std::sys_common::backtrace::__rust_end_short_backtrace::h46a0f5def1fd2bc3
   9:     0x7ff2c2299202 - rust_begin_unwind
  10:     0x7ff2c2248e13 - core::panicking::panic_fmt::h377e3e9e3f40d2d6
  11:     0x7ff2c332bcc2 - <rustc_const_eval[7ed7272cc296e65a]::interpret::eval_context::InterpCx<rustc_mir_transform[c0fae5e835c96068]::const_prop::ConstPropMachine>>::copy_op_no_validate
  12:     0x7ff2c33161e5 - <rustc_const_eval[7ed7272cc296e65a]::interpret::eval_context::InterpCx<rustc_mir_transform[c0fae5e835c96068]::const_prop::ConstPropMachine>>::eval_rvalue_into_place
  13:     0x7ff2c32cea94 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstPropagator as rustc_middle[871e6b34a4e1300a]::mir::visit::Visitor>::visit_statement
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  14:     0x7ff2c32ce339 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstPropagator as rustc_middle[871e6b34a4e1300a]::mir::visit::Visitor>::visit_body
  15:     0x7ff2c32cafc3 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstProp as rustc_mir_transform[c0fae5e835c96068]::pass_manager::MirLint>::run_lint
  16:     0x7ff2c316f5a7 - rustc_mir_transform[c0fae5e835c96068]::pass_manager::run_passes
  17:     0x7ff2c3227060 - rustc_mir_transform[c0fae5e835c96068]::run_post_borrowck_cleanup_passes
  18:     0x7ff2c3226ae6 - rustc_mir_transform[c0fae5e835c96068]::mir_drops_elaborated_and_const_checked
  19:     0x7ff2c4834634 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<rustc_middle[871e6b34a4e1300a]::ty::WithOptConstParam<rustc_span[6bf9c9ef6323007c]::def_id::LocalDefId>, &rustc_data_structures[abe6f3b38b66b0d1]::steal::Steal<rustc_middle[871e6b34a4e1300a]::mir::Body>>>
  20:     0x7ff2c4961f73 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  21:     0x7ff2c443ceea - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  22:     0x7ff2c32276c7 - rustc_mir_transform[c0fae5e835c96068]::optimized_mir
  23:     0x7ff2c4868ce0 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<rustc_span[6bf9c9ef6323007c]::def_id::DefId, &rustc_middle[871e6b34a4e1300a]::mir::Body>>
  24:     0x7ff2c4916904 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::optimized_mir, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  25:     0x7ff2c443ea39 - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::optimized_mir
  26:     0x7ff2c57f3899 - <rustc_middle[871e6b34a4e1300a]::ty::context::TyCtxt>::instance_mir
  27:     0x7ff2c309cbba - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_neighbours
  28:     0x7ff2c3097c4a - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_items_rec
  29:     0x7ff2c30a3c4d - <core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[50cabbe35dafdb11]::ops::function::FnOnce<()>>::call_once
  30:     0x7ff2c30dc535 - std[6090a79dacbb318d]::panicking::try::<(), core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  31:     0x7ff2c30b6dce - rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in::<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  32:     0x7ff2c30db50b - <rustc_session[572d661066ad9f72]::session::Session>::time::<(), rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7ff2c30945b7 - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items
  34:     0x7ff2c30a57ba - rustc_monomorphize[60d4ad8f60e0a70a]::partitioning::collect_and_partition_mono_items
  35:     0x7ff2c488ae72 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<(), (&std[6090a79dacbb318d]::collections::hash::set::HashSet<rustc_span[6bf9c9ef6323007c]::def_id::DefId, core[50cabbe35dafdb11]::hash::BuildHasherDefault<rustc_hash[ea08e494adf983e5]::FxHasher>>, &[rustc_middle[871e6b34a4e1300a]::mir::mono::CodegenUnit])>>
  36:     0x7ff2c495d39a - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::collect_and_partition_mono_items, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  37:     0x7ff2c4482759 - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7ff2c2e9fa4c - rustc_codegen_ssa[19c4f0e1ad5c2897]::base::codegen_crate::<rustc_codegen_llvm[37f5bada3dd787a8]::LlvmCodegenBackend>
  39:     0x7ff2c2f5584d - <rustc_codegen_llvm[37f5bada3dd787a8]::LlvmCodegenBackend as rustc_codegen_ssa[19c4f0e1ad5c2897]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7ff2c2dc51eb - <rustc_session[572d661066ad9f72]::session::Session>::time::<alloc[c3ac31bfa1c22eda]::boxed::Box<dyn core[50cabbe35dafdb11]::any::Any>, rustc_interface[f7a89b5299bdd86e]::passes::start_codegen::{closure#0}>
  41:     0x7ff2c2d8a32c - <rustc_interface[f7a89b5299bdd86e]::passes::QueryContext>::enter::<<rustc_interface[f7a89b5299bdd86e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[50cabbe35dafdb11]::result::Result<alloc[c3ac31bfa1c22eda]::boxed::Box<dyn core[50cabbe35dafdb11]::any::Any>, rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  42:     0x7ff2c2d6f41d - <rustc_interface[f7a89b5299bdd86e]::queries::Queries>::ongoing_codegen
  43:     0x7ff2c2c5238f - <rustc_interface[f7a89b5299bdd86e]::interface::Compiler>::enter::<rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}::{closure#2}, core[50cabbe35dafdb11]::result::Result<core[50cabbe35dafdb11]::option::Option<rustc_interface[f7a89b5299bdd86e]::queries::Linker>, rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  44:     0x7ff2c2c42311 - rustc_span[6bf9c9ef6323007c]::with_source_map::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_interface[f7a89b5299bdd86e]::interface::create_compiler_and_run<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7ff2c2c66f9a - rustc_interface[f7a89b5299bdd86e]::interface::create_compiler_and_run::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>
  46:     0x7ff2c2c3c262 - <scoped_tls[1c5395a31ad1702]::ScopedKey<rustc_span[6bf9c9ef6323007c]::SessionGlobals>>::set::<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  47:     0x7ff2c2c73f6f - std[6090a79dacbb318d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  48:     0x7ff2c2cab75e - std[6090a79dacbb318d]::panicking::try::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<<std[6090a79dacbb318d]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7ff2c2c75aa2 - <<std[6090a79dacbb318d]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#1} as core[50cabbe35dafdb11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7ff2c22a49e5 - std::sys::unix::thread::Thread::new::thread_start::h91b5d34fcb23e137
  51:     0x7ff2bc7ef609 - start_thread
  52:     0x7ff2c2102133 - clone
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (aee372a62 2022-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/unsized-locals/autoderef.rs stdout ----
---- [ui] src/test/ui/unsized-locals/autoderef.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/autoderef.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/autoderef/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'cannot have unsized immediates', /checkout/compiler/rustc_const_eval/src/interpret/place.rs:645:17
stack backtrace:
   0:     0x7fc6b0c84f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he2b2813cbab87088
   1:     0x7fc6b0ceb9f8 - core::fmt::write::h85053c8e50ac906d
   2:     0x7fc6b0c755b1 - std::io::Write::write_fmt::h1c70ced061411a22
   3:     0x7fc6b0c87f5e - std::panicking::default_hook::{{closure}}::hf522f9bae8b1958a
   4:     0x7fc6b0c87c1f - std::panicking::default_hook::hc3b110f368f13f1a
   5:     0x7fc6b163f704 - rustc_driver[39f7a1d32b8dfc5d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7fc6b0c88712 - std::panicking::rust_panic_with_hook::h07650724f4d15eb1
   7:     0x7fc6b0c884f9 - std::panicking::begin_panic_handler::{{closure}}::ha6ccca2a50a9de21
   8:     0x7fc6b0c85514 - std::sys_common::backtrace::__rust_end_short_backtrace::h46a0f5def1fd2bc3
   9:     0x7fc6b0c88202 - rust_begin_unwind
  10:     0x7fc6b0c37e13 - core::panicking::panic_fmt::h377e3e9e3f40d2d6
  11:     0x7fc6b1d1acc2 - <rustc_const_eval[7ed7272cc296e65a]::interpret::eval_context::InterpCx<rustc_mir_transform[c0fae5e835c96068]::const_prop::ConstPropMachine>>::copy_op_no_validate
  12:     0x7fc6b1d051e5 - <rustc_const_eval[7ed7272cc296e65a]::interpret::eval_context::InterpCx<rustc_mir_transform[c0fae5e835c96068]::const_prop::ConstPropMachine>>::eval_rvalue_into_place
  13:     0x7fc6b1cbda94 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstPropagator as rustc_middle[871e6b34a4e1300a]::mir::visit::Visitor>::visit_statement
  14:     0x7fc6b1cbd339 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstPropagator as rustc_middle[871e6b34a4e1300a]::mir::visit::Visitor>::visit_body
  15:     0x7fc6b1cb9fc3 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstProp as rustc_mir_transform[c0fae5e835c96068]::pass_manager::MirLint>::run_lint
  16:     0x7fc6b1b5e5a7 - rustc_mir_transform[c0fae5e835c96068]::pass_manager::run_passes
  17:     0x7fc6b1c16060 - rustc_mir_transform[c0fae5e835c96068]::run_post_borrowck_cleanup_passes
  18:     0x7fc6b1c15ae6 - rustc_mir_transform[c0fae5e835c96068]::mir_drops_elaborated_and_const_checked
  19:     0x7fc6b3223634 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<rustc_middle[871e6b34a4e1300a]::ty::WithOptConstParam<rustc_span[6bf9c9ef6323007c]::def_id::LocalDefId>, &rustc_data_structures[abe6f3b38b66b0d1]::steal::Steal<rustc_middle[871e6b34a4e1300a]::mir::Body>>>
  20:     0x7fc6b3350f73 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  21:     0x7fc6b2e2beea - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  22:     0x7fc6b1c166c7 - rustc_mir_transform[c0fae5e835c96068]::optimized_mir
  23:     0x7fc6b3257ce0 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<rustc_span[6bf9c9ef6323007c]::def_id::DefId, &rustc_middle[871e6b34a4e1300a]::mir::Body>>
  24:     0x7fc6b3305904 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::optimized_mir, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  25:     0x7fc6b2e2da39 - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::optimized_mir
  26:     0x7fc6b41e2899 - <rustc_middle[871e6b34a4e1300a]::ty::context::TyCtxt>::instance_mir
  27:     0x7fc6b1a8bbba - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_neighbours
  28:     0x7fc6b1a86c4a - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_items_rec
  29:     0x7fc6b1a92c4d - <core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[50cabbe35dafdb11]::ops::function::FnOnce<()>>::call_once
  30:     0x7fc6b1acb535 - std[6090a79dacbb318d]::panicking::try::<(), core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  31:     0x7fc6b1aa5dce - rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in::<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  32:     0x7fc6b1aca50b - <rustc_session[572d661066ad9f72]::session::Session>::time::<(), rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7fc6b1a835b7 - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items
  34:     0x7fc6b1a947ba - rustc_monomorphize[60d4ad8f60e0a70a]::partitioning::collect_and_partition_mono_items
  35:     0x7fc6b3279e72 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<(), (&std[6090a79dacbb318d]::collections::hash::set::HashSet<rustc_span[6bf9c9ef6323007c]::def_id::DefId, core[50cabbe35dafdb11]::hash::BuildHasherDefault<rustc_hash[ea08e494adf983e5]::FxHasher>>, &[rustc_middle[871e6b34a4e1300a]::mir::mono::CodegenUnit])>>
  36:     0x7fc6b334c39a - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::collect_and_partition_mono_items, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  37:     0x7fc6b2e71759 - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7fc6b188ea4c - rustc_codegen_ssa[19c4f0e1ad5c2897]::base::codegen_crate::<rustc_codegen_llvm[37f5bada3dd787a8]::LlvmCodegenBackend>
  39:     0x7fc6b194484d - <rustc_codegen_llvm[37f5bada3dd787a8]::LlvmCodegenBackend as rustc_codegen_ssa[19c4f0e1ad5c2897]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7fc6b17b41eb - <rustc_session[572d661066ad9f72]::session::Session>::time::<alloc[c3ac31bfa1c22eda]::boxed::Box<dyn core[50cabbe35dafdb11]::any::Any>, rustc_interface[f7a89b5299bdd86e]::passes::start_codegen::{closure#0}>
  41:     0x7fc6b177932c - <rustc_interface[f7a89b5299bdd86e]::passes::QueryContext>::enter::<<rustc_interface[f7a89b5299bdd86e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[50cabbe35dafdb11]::result::Result<alloc[c3ac31bfa1c22eda]::boxed::Box<dyn core[50cabbe35dafdb11]::any::Any>, rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  42:     0x7fc6b175e41d - <rustc_interface[f7a89b5299bdd86e]::queries::Queries>::ongoing_codegen
  43:     0x7fc6b164138f - <rustc_interface[f7a89b5299bdd86e]::interface::Compiler>::enter::<rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}::{closure#2}, core[50cabbe35dafdb11]::result::Result<core[50cabbe35dafdb11]::option::Option<rustc_interface[f7a89b5299bdd86e]::queries::Linker>, rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  44:     0x7fc6b1631311 - rustc_span[6bf9c9ef6323007c]::with_source_map::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_interface[f7a89b5299bdd86e]::interface::create_compiler_and_run<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7fc6b1655f9a - rustc_interface[f7a89b5299bdd86e]::interface::create_compiler_and_run::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>
  46:     0x7fc6b162b262 - <scoped_tls[1c5395a31ad1702]::ScopedKey<rustc_span[6bf9c9ef6323007c]::SessionGlobals>>::set::<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  47:     0x7fc6b1662f6f - std[6090a79dacbb318d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  48:     0x7fc6b169a75e - std[6090a79dacbb318d]::panicking::try::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<<std[6090a79dacbb318d]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7fc6b1664aa2 - <<std[6090a79dacbb318d]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#1} as core[50cabbe35dafdb11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7fc6b0c939e5 - std::sys::unix::thread::Thread::new::thread_start::h91b5d34fcb23e137
  51:     0x7fc6ab1de609 - start_thread
  52:     0x7fc6b0af1133 - clone
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (aee372a62 2022-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------


---- [ui] src/test/ui/unsized-locals/by-value-trait-object-safety-withdefault.rs stdout ----
---- [ui] src/test/ui/unsized-locals/by-value-trait-object-safety-withdefault.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized-locals/by-value-trait-object-safety-withdefault.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'cannot have unsized immediates', /checkout/compiler/rustc_const_eval/src/interpret/place.rs:645:17
   0:     0x7f8fbf3dcf9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he2b2813cbab87088
   1:     0x7f8fbf4439f8 - core::fmt::write::h85053c8e50ac906d
   2:     0x7f8fbf3cd5b1 - std::io::Write::write_fmt::h1c70ced061411a22
   2:     0x7f8fbf3cd5b1 - std::io::Write::write_fmt::h1c70ced061411a22
   3:     0x7f8fbf3dff5e - std::panicking::default_hook::{{closure}}::hf522f9bae8b1958a
   4:     0x7f8fbf3dfc1f - std::panicking::default_hook::hc3b110f368f13f1a
   5:     0x7f8fbfd97704 - rustc_driver[39f7a1d32b8dfc5d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f8fbf3e0712 - std::panicking::rust_panic_with_hook::h07650724f4d15eb1
   7:     0x7f8fbf3e04f9 - std::panicking::begin_panic_handler::{{closure}}::ha6ccca2a50a9de21
   8:     0x7f8fbf3dd514 - std::sys_common::backtrace::__rust_end_short_backtrace::h46a0f5def1fd2bc3
   9:     0x7f8fbf3e0202 - rust_begin_unwind
  10:     0x7f8fbf38fe13 - core::panicking::panic_fmt::h377e3e9e3f40d2d6
  11:     0x7f8fc0472cc2 - <rustc_const_eval[7ed7272cc296e65a]::interpret::eval_context::InterpCx<rustc_mir_transform[c0fae5e835c96068]::const_prop::ConstPropMachine>>::copy_op_no_validate
  12:     0x7f8fc045d1e5 - <rustc_const_eval[7ed7272cc296e65a]::interpret::eval_context::InterpCx<rustc_mir_transform[c0fae5e835c96068]::const_prop::ConstPropMachine>>::eval_rvalue_into_place
  13:     0x7f8fc0415a94 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstPropagator as rustc_middle[871e6b34a4e1300a]::mir::visit::Visitor>::visit_statement
  14:     0x7f8fc0415339 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstPropagator as rustc_middle[871e6b34a4e1300a]::mir::visit::Visitor>::visit_body
  15:     0x7f8fc0411fc3 - <rustc_mir_transform[c0fae5e835c96068]::const_prop_lint::ConstProp as rustc_mir_transform[c0fae5e835c96068]::pass_manager::MirLint>::run_lint
  16:     0x7f8fc02b65a7 - rustc_mir_transform[c0fae5e835c96068]::pass_manager::run_passes
  17:     0x7f8fc036e060 - rustc_mir_transform[c0fae5e835c96068]::run_post_borrowck_cleanup_passes
  18:     0x7f8fc036dae6 - rustc_mir_transform[c0fae5e835c96068]::mir_drops_elaborated_and_const_checked
  19:     0x7f8fc197b634 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<rustc_middle[871e6b34a4e1300a]::ty::WithOptConstParam<rustc_span[6bf9c9ef6323007c]::def_id::LocalDefId>, &rustc_data_structures[abe6f3b38b66b0d1]::steal::Steal<rustc_middle[871e6b34a4e1300a]::mir::Body>>>
  20:     0x7f8fc1aa8f73 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  21:     0x7f8fc1583eea - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  22:     0x7f8fc036e6c7 - rustc_mir_transform[c0fae5e835c96068]::optimized_mir
  23:     0x7f8fc19afce0 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<rustc_span[6bf9c9ef6323007c]::def_id::DefId, &rustc_middle[871e6b34a4e1300a]::mir::Body>>
  24:     0x7f8fc1a5d904 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::optimized_mir, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  25:     0x7f8fc1585a39 - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::optimized_mir
  26:     0x7f8fc293a899 - <rustc_middle[871e6b34a4e1300a]::ty::context::TyCtxt>::instance_mir
  27:     0x7f8fc01e3bba - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_neighbours
  28:     0x7f8fc01dec4a - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_items_rec
  29:     0x7f8fc01eac4d - <core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[50cabbe35dafdb11]::ops::function::FnOnce<()>>::call_once
  30:     0x7f8fc0223535 - std[6090a79dacbb318d]::panicking::try::<(), core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  31:     0x7f8fc01fddce - rustc_data_structures[abe6f3b38b66b0d1]::sync::par_for_each_in::<alloc[c3ac31bfa1c22eda]::vec::Vec<rustc_middle[871e6b34a4e1300a]::mir::mono::MonoItem>, rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>
  32:     0x7f8fc022250b - <rustc_session[572d661066ad9f72]::session::Session>::time::<(), rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items::{closure#1}>
  33:     0x7f8fc01db5b7 - rustc_monomorphize[60d4ad8f60e0a70a]::collector::collect_crate_mono_items
  34:     0x7f8fc01ec7ba - rustc_monomorphize[60d4ad8f60e0a70a]::partitioning::collect_and_partition_mono_items
  35:     0x7f8fc19d1e72 - rustc_query_system[32d98eccfd57aea1]::query::plumbing::try_execute_query::<rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt, rustc_query_system[32d98eccfd57aea1]::query::caches::DefaultCache<(), (&std[6090a79dacbb318d]::collections::hash::set::HashSet<rustc_span[6bf9c9ef6323007c]::def_id::DefId, core[50cabbe35dafdb11]::hash::BuildHasherDefault<rustc_hash[ea08e494adf983e5]::FxHasher>>, &[rustc_middle[871e6b34a4e1300a]::mir::mono::CodegenUnit])>>
  36:     0x7f8fc1aa439a - rustc_query_system[32d98eccfd57aea1]::query::plumbing::get_query::<rustc_query_impl[9dca0033ae7ca475]::queries::collect_and_partition_mono_items, rustc_query_impl[9dca0033ae7ca475]::plumbing::QueryCtxt>
  37:     0x7f8fc15c9759 - <rustc_query_impl[9dca0033ae7ca475]::Queries as rustc_middle[871e6b34a4e1300a]::ty::query::QueryEngine>::collect_and_partition_mono_items
  38:     0x7f8fbffe6a4c - rustc_codegen_ssa[19c4f0e1ad5c2897]::base::codegen_crate::<rustc_codegen_llvm[37f5bada3dd787a8]::LlvmCodegenBackend>
  39:     0x7f8fc009c84d - <rustc_codegen_llvm[37f5bada3dd787a8]::LlvmCodegenBackend as rustc_codegen_ssa[19c4f0e1ad5c2897]::traits::backend::CodegenBackend>::codegen_crate
  40:     0x7f8fbff0c1eb - <rustc_session[572d661066ad9f72]::session::Session>::time::<alloc[c3ac31bfa1c22eda]::boxed::Box<dyn core[50cabbe35dafdb11]::any::Any>, rustc_interface[f7a89b5299bdd86e]::passes::start_codegen::{closure#0}>
  41:     0x7f8fbfed132c - <rustc_interface[f7a89b5299bdd86e]::passes::QueryContext>::enter::<<rustc_interface[f7a89b5299bdd86e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[50cabbe35dafdb11]::result::Result<alloc[c3ac31bfa1c22eda]::boxed::Box<dyn core[50cabbe35dafdb11]::any::Any>, rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  42:     0x7f8fbfeb641d - <rustc_interface[f7a89b5299bdd86e]::queries::Queries>::ongoing_codegen
  43:     0x7f8fbfd9938f - <rustc_interface[f7a89b5299bdd86e]::interface::Compiler>::enter::<rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}::{closure#2}, core[50cabbe35dafdb11]::result::Result<core[50cabbe35dafdb11]::option::Option<rustc_interface[f7a89b5299bdd86e]::queries::Linker>, rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  44:     0x7f8fbfd89311 - rustc_span[6bf9c9ef6323007c]::with_source_map::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_interface[f7a89b5299bdd86e]::interface::create_compiler_and_run<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#1}>
  45:     0x7f8fbfdadf9a - rustc_interface[f7a89b5299bdd86e]::interface::create_compiler_and_run::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>
  46:     0x7f8fbfd83262 - <scoped_tls[1c5395a31ad1702]::ScopedKey<rustc_span[6bf9c9ef6323007c]::SessionGlobals>>::set::<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  47:     0x7f8fbfdbaf6f - std[6090a79dacbb318d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>
  48:     0x7f8fbfdf275e - std[6090a79dacbb318d]::panicking::try::<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, core[50cabbe35dafdb11]::panic::unwind_safe::AssertUnwindSafe<<std[6090a79dacbb318d]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  49:     0x7f8fbfdbcaa2 - <<std[6090a79dacbb318d]::thread::Builder>::spawn_unchecked_<rustc_interface[f7a89b5299bdd86e]::util::run_in_thread_pool_with_globals<rustc_interface[f7a89b5299bdd86e]::interface::run_compiler<core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>, rustc_driver[39f7a1d32b8dfc5d]::run_compiler::{closure#1}>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#0}, core[50cabbe35dafdb11]::result::Result<(), rustc_errors[e9e189480c866166]::ErrorGuaranteed>>::{closure#1} as core[50cabbe35dafdb11]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f8fbf3eb9e5 - std::sys::unix::thread::Thread::new::thread_start::h91b5d34fcb23e137
  51:     0x7f8fb9936609 - start_thread
  52:     0x7f8fbf249133 - clone
  53:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.65.0-nightly (aee372a62 2022-08-07) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------



