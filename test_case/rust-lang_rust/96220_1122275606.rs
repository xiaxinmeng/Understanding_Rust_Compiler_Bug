plain
---- [ui] src/test/ui/aligned_enum_cast.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/aligned_enum_cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/aligned_enum_cast/auxiliary"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `Size(1 bytes)`,
 right: `Size(8 bytes)`: abi::Scalar size does not match layout size', /checkout/compiler/rustc_const_eval/src/interpret/operand.rs:289:13
   0:     0x7f71d15c79ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h17f3083166418703
   0:     0x7f71d15c79ac - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h17f3083166418703
   1:     0x7f71d162df28 - core::fmt::write::hc2cf8779fb814709
   2:     0x7f71d15b77a1 - std::io::Write::write_fmt::h1cdb42a5a6d30072
   3:     0x7f71d15ca9de - std::panicking::default_hook::{{closure}}::h9805685509cf40fa
   4:     0x7f71d15ca60c - std::panicking::default_hook::hb827fea9ed39a82c
   5:     0x7f71d214a8f1 - rustc_driver[4cdb4d16c7520cea]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f71d15cb23e - std::panicking::rust_panic_with_hook::h7d57713d070d0632
   7:     0x7f71d15cb037 - std::panicking::begin_panic_handler::{{closure}}::h05ceb62803b882f5
   8:     0x7f71d15c7ec4 - std::sys_common::backtrace::__rust_end_short_backtrace::h68ec1952be8c81a8
   9:     0x7f71d15cad19 - rust_begin_unwind
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  10:     0x7f71d157f073 - core::panicking::panic_fmt::h22733906f6a3937e
  11:     0x7f71d162a56e - core::panicking::assert_failed_inner::h708f0f5eefc86dfc
  12:     0x7f71d1d8fe4b - core[2e01ac4cf6dab05e]::panicking::assert_failed::<rustc_target[e7b827d4f556958]::abi::Size, rustc_target[e7b827d4f556958]::abi::Size>
  13:     0x7f71d2819764 - <rustc_const_eval[b0f688a2c9e7cb28]::interpret::eval_context::InterpCx<rustc_mir_transform[c9f3dec782a7660c]::const_prop_lint::ConstPropMachine>>::read_immediate_raw
  14:     0x7f71d27ea527 - <rustc_const_eval[b0f688a2c9e7cb28]::interpret::eval_context::InterpCx<rustc_mir_transform[c9f3dec782a7660c]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  15:     0x7f71d26e91c2 - <rustc_mir_transform[c9f3dec782a7660c]::const_prop_lint::ConstPropagator as rustc_middle[eb5e00ca01b87278]::mir::visit::Visitor>::visit_statement
  16:     0x7f71d26e8c5f - <rustc_mir_transform[c9f3dec782a7660c]::const_prop_lint::ConstPropagator as rustc_middle[eb5e00ca01b87278]::mir::visit::Visitor>::visit_body
  17:     0x7f71d26e51bd - <rustc_mir_transform[c9f3dec782a7660c]::const_prop_lint::ConstProp as rustc_mir_transform[c9f3dec782a7660c]::pass_manager::MirLint>::run_lint
  18:     0x7f71d263f162 - rustc_mir_transform[c9f3dec782a7660c]::pass_manager::run_passes
  19:     0x7f71d2725609 - rustc_mir_transform[c9f3dec782a7660c]::run_post_borrowck_cleanup_passes
  20:     0x7f71d2724ec9 - rustc_mir_transform[c9f3dec782a7660c]::mir_drops_elaborated_and_const_checked
  21:     0x7f71d3ad7e68 - rustc_query_system[8365c6fc2a9ef711]::query::plumbing::try_execute_query::<rustc_query_impl[ef58c2cbec439f0b]::plumbing::QueryCtxt, rustc_query_system[8365c6fc2a9ef711]::query::caches::DefaultCache<rustc_middle[eb5e00ca01b87278]::ty::WithOptConstParam<rustc_span[5748b168e1edc3f0]::def_id::LocalDefId>, &rustc_data_structures[58527cbf4e263267]::steal::Steal<rustc_middle[eb5e00ca01b87278]::mir::Body>>>
  22:     0x7f71d3bf77a3 - rustc_query_system[8365c6fc2a9ef711]::query::plumbing::get_query::<rustc_query_impl[ef58c2cbec439f0b]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[ef58c2cbec439f0b]::plumbing::QueryCtxt>
  23:     0x7f71d3a28b57 - <rustc_query_impl[ef58c2cbec439f0b]::Queries as rustc_middle[eb5e00ca01b87278]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24:     0x7f71d2725b96 - rustc_mir_transform[c9f3dec782a7660c]::optimized_mir
  25:     0x7f71d3b0c2b8 - rustc_query_system[8365c6fc2a9ef711]::query::plumbing::try_execute_query::<rustc_query_impl[ef58c2cbec439f0b]::plumbing::QueryCtxt, rustc_query_system[8365c6fc2a9ef711]::query::caches::DefaultCache<rustc_span[5748b168e1edc3f0]::def_id::DefId, &rustc_middle[eb5e00ca01b87278]::mir::Body>>
  26:     0x7f71d3baf642 - rustc_query_system[8365c6fc2a9ef711]::query::plumbing::get_query::<rustc_query_impl[ef58c2cbec439f0b]::queries::optimized_mir, rustc_query_impl[ef58c2cbec439f0b]::plumbing::QueryCtxt>
  27:     0x7f71d3a2a5f9 - <rustc_query_impl[ef58c2cbec439f0b]::Queries as rustc_middle[eb5e00ca01b87278]::ty::query::QueryEngine>::optimized_mir
  28:     0x7f71d49a6c8c - <rustc_middle[eb5e00ca01b87278]::ty::context::TyCtxt>::instance_mir
  29:     0x7f71d256e687 - rustc_monomorphize[86a6048003f55953]::collector::collect_neighbours
  30:     0x7f71d2566e04 - rustc_monomorphize[86a6048003f55953]::collector::collect_items_rec
  31:     0x7f71d2583061 - <rustc_session[4bb5ff3e5b30430]::session::Session>::time::<(), rustc_monomorphize[86a6048003f55953]::collector::collect_crate_mono_items::{closure#1}>
  32:     0x7f71d256385f - rustc_monomorphize[86a6048003f55953]::collector::collect_crate_mono_items
  33:     0x7f71d2581ce9 - rustc_monomorphize[86a6048003f55953]::partitioning::collect_and_partition_mono_items
  34:     0x7f71d3b2e82a - rustc_query_system[8365c6fc2a9ef711]::query::plumbing::try_execute_query::<rustc_query_impl[ef58c2cbec439f0b]::plumbing::QueryCtxt, rustc_query_system[8365c6fc2a9ef711]::query::caches::DefaultCache<(), (&std[f801e923bb10d52f]::collections::hash::set::HashSet<rustc_span[5748b168e1edc3f0]::def_id::DefId, core[2e01ac4cf6dab05e]::hash::BuildHasherDefault<rustc_hash[fcd2943904a845d3]::FxHasher>>, &[rustc_middle[eb5e00ca01b87278]::mir::mono::CodegenUnit])>>
  35:     0x7f71d3bf2afa - rustc_query_system[8365c6fc2a9ef711]::query::plumbing::get_query::<rustc_query_impl[ef58c2cbec439f0b]::queries::collect_and_partition_mono_items, rustc_query_impl[ef58c2cbec439f0b]::plumbing::QueryCtxt>
  36:     0x7f71d3a6d999 - <rustc_query_impl[ef58c2cbec439f0b]::Queries as rustc_middle[eb5e00ca01b87278]::ty::query::QueryEngine>::collect_and_partition_mono_items
  37:     0x7f71d238fe69 - rustc_codegen_ssa[9448e158543b9257]::base::codegen_crate::<rustc_codegen_llvm[ee54ed49d1ea14b0]::LlvmCodegenBackend>
  38:     0x7f71d23d7e9b - <rustc_codegen_llvm[ee54ed49d1ea14b0]::LlvmCodegenBackend as rustc_codegen_ssa[9448e158543b9257]::traits::backend::CodegenBackend>::codegen_crate
  39:     0x7f71d2234591 - <rustc_session[4bb5ff3e5b30430]::session::Session>::time::<alloc[631165e48ad6c040]::boxed::Box<dyn core[2e01ac4cf6dab05e]::any::Any>, rustc_interface[a448d5331fd4871e]::passes::start_codegen::{closure#0}>
  40:     0x7f71d221be43 - <rustc_interface[a448d5331fd4871e]::passes::QueryContext>::enter::<<rustc_interface[a448d5331fd4871e]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<alloc[631165e48ad6c040]::boxed::Box<dyn core[2e01ac4cf6dab05e]::any::Any>, rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>
  41:     0x7f71d220750e - <rustc_interface[a448d5331fd4871e]::queries::Queries>::ongoing_codegen
  42:     0x7f71d20da110 - <rustc_interface[a448d5331fd4871e]::interface::Compiler>::enter::<rustc_driver[4cdb4d16c7520cea]::run_compiler::{closure#1}::{closure#2}, core[2e01ac4cf6dab05e]::result::Result<core[2e01ac4cf6dab05e]::option::Option<rustc_interface[a448d5331fd4871e]::queries::Linker>, rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>
  43:     0x7f71d20bb2ab - rustc_span[5748b168e1edc3f0]::with_source_map::<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, rustc_interface[a448d5331fd4871e]::interface::create_compiler_and_run<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, rustc_driver[4cdb4d16c7520cea]::run_compiler::{closure#1}>::{closure#1}>
  44:     0x7f71d20db2b9 - <scoped_tls[700d02192ac3a1f4]::ScopedKey<rustc_span[5748b168e1edc3f0]::SessionGlobals>>::set::<rustc_interface[a448d5331fd4871e]::interface::run_compiler<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, rustc_driver[4cdb4d16c7520cea]::run_compiler::{closure#1}>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>
  45:     0x7f71d21369a9 - std[f801e923bb10d52f]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a448d5331fd4871e]::util::run_in_thread_pool_with_globals<rustc_interface[a448d5331fd4871e]::interface::run_compiler<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, rustc_driver[4cdb4d16c7520cea]::run_compiler::{closure#1}>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>
  46:     0x7f71d20efa11 - std[f801e923bb10d52f]::panicking::try::<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, core[2e01ac4cf6dab05e]::panic::unwind_safe::AssertUnwindSafe<<std[f801e923bb10d52f]::thread::Builder>::spawn_unchecked_<rustc_interface[a448d5331fd4871e]::util::run_in_thread_pool_with_globals<rustc_interface[a448d5331fd4871e]::interface::run_compiler<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, rustc_driver[4cdb4d16c7520cea]::run_compiler::{closure#1}>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  47:     0x7f71d2131902 - <<std[f801e923bb10d52f]::thread::Builder>::spawn_unchecked_<rustc_interface[a448d5331fd4871e]::util::run_in_thread_pool_with_globals<rustc_interface[a448d5331fd4871e]::interface::run_compiler<core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>, rustc_driver[4cdb4d16c7520cea]::run_compiler::{closure#1}>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>::{closure#0}, core[2e01ac4cf6dab05e]::result::Result<(), rustc_errors[fd994c9671adeae3]::ErrorGuaranteed>>::{closure#1} as core[2e01ac4cf6dab05e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7f71d15d6593 - std::sys::unix::thread::Thread::new::thread_start::h3c0d742e422cb53d
  49:     0x7f71cbb29609 - start_thread
  50:     0x7f71d143c163 - clone
  51:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (279a5319d 2022-05-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [mir_drops_elaborated_and_const_checked] elaborating drops for `main`
#1 [optimized_mir] optimizing MIR for `main`
#2 [collect_and_partition_mono_items] collect_and_partition_mono_items
------------------------------------------



