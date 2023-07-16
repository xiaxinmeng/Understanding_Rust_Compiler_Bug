plain
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
   Compiling rustc_driver v0.0.0 (/checkout/compiler/rustc_driver)
warning: LTO with dylibs may not be as effective
warning: `rustc_driver` (lib) generated 1 warning
   Compiling rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
    Finished release [optimized] target(s) in 5m 42s
Copying stage1 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
........................................................................................ 12144/13474
........................................................................................ 12232/13474
........................................................................................ 12320/13474
........................................................................................ 12408/13474
..............................................................................F..F...... 12496/13474
............................i........................................................... 12672/13474
........................................................................................ 12760/13474
........................................................................................ 12848/13474
........................................................................................ 12936/13474
---
failures:

---- [ui] src/test/ui/consts/const-eval/const-eval-query-stack.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug=2" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-query-stack/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:19:16
   |
   |
LL | const X: i32 = 1 / 0; //~WARN any use of this value will cause an error
   | ------------   ^^^^^ attempt to divide `1_i32` by zero
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/const-eval/const-eval-query-stack.rs:18:8
   |
LL | #[warn(const_err)]
---
   = note: `#[deny(const_err)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

thread '<unnamed>' panicked at 'aborting after 2 errors due to `-Z treat-err-as-bug=2`', compiler/rustc_errors/src/lib.rs:1525:36
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   1: core::panicking::panic_fmt
   2: <rustc_errors::HandlerInner>::panic_if_treat_err_as_bug
   4: <rustc_errors::Handler>::emit_diagnostic
   4: <rustc_errors::Handler>::emit_diagnostic
   5: <() as rustc_errors::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
   6: <<rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2} as core::ops::function::FnOnce<(rustc_errors::diagnostic_builder::LintDiagnosticBuilder<()>,)>>::call_once::{shim:vtable#0}
   7: <alloc::boxed::Box<dyn for<'a> core::ops::function::FnOnce<(rustc_errors::diagnostic_builder::LintDiagnosticBuilder<'a, ()>,), Output = ()>> as core::ops::function::FnOnce<(rustc_errors::diagnostic_builder::LintDiagnosticBuilder<()>,)>>::call_once
   8: rustc_middle::lint::struct_lint_level::struct_lint_level_impl
   9: rustc_middle::lint::struct_lint_level::<<rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2}>
  10: <rustc_middle::ty::context::TyCtxt>::struct_span_lint_hir::<rustc_span::span_encoding::Span, <rustc_const_eval::const_eval::error::ConstEvalErr>::struct_generic<<rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint::{closure#0}>::{closure#2}>
  11: <rustc_const_eval::const_eval::error::ConstEvalErr>::report_as_lint
  12: <rustc_mir_transform::const_prop_lint::ConstPropagator>::eval_constant
  13: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_constant
  14: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_statement
  15: <rustc_mir_transform::const_prop_lint::ConstPropagator as rustc_middle::mir::visit::Visitor>::visit_body
  16: <rustc_mir_transform::const_prop_lint::ConstProp as rustc_mir_transform::pass_manager::MirLint>::run_lint
  18: rustc_mir_transform::run_analysis_to_runtime_passes
  19: rustc_mir_transform::mir_drops_elaborated_and_const_checked
  19: rustc_mir_transform::mir_drops_elaborated_and_const_checked
  20: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::ty::WithOptConstParam<rustc_span::def_id::LocalDefId>, &rustc_data_structures::steal::Steal<rustc_middle::mir::Body>>>
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  24: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, &rustc_middle::mir::Body>>
  25: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::optimized_mir, rustc_query_impl::plumbing::QueryCtxt>
  26: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::optimized_mir
  27: <rustc_middle::ty::context::TyCtxt>::instance_mir
  27: <rustc_middle::ty::context::TyCtxt>::instance_mir
  28: rustc_monomorphize::collector::collect_neighbours
  29: rustc_monomorphize::collector::collect_items_rec
  30: <core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core::ops::function::FnOnce<()>>::call_once
  31: std::panic::catch_unwind::<core::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures::sync::par_for_each_in<alloc::vec::Vec<rustc_middle::mir::mono::MonoItem>, rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>, ()>
  32: <rustc_session::session::Session>::time::<(), rustc_monomorphize::collector::collect_crate_mono_items::{closure#1}>
  33: rustc_monomorphize::collector::collect_crate_mono_items
  34: rustc_monomorphize::partitioning::collect_and_partition_mono_items
  35: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<(), (&std::collections::hash::set::HashSet<rustc_span::def_id::DefId, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &[rustc_middle::mir::mono::CodegenUnit])>>
  36: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::collect_and_partition_mono_items, rustc_query_impl::plumbing::QueryCtxt>
  37: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::collect_and_partition_mono_items
  38: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  39: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  40: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  41: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorGuaranteed>>
  42: <rustc_interface::queries::Queries>::ongoing_codegen
  44: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
  45: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>
  46: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorGuaranteed>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorGuaranteed>>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread '<unnamed>' panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
stack backtrace:
   0:     0x7fcaed2a015c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7fcaed308c98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7fcaed2909d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7fcaed2a314e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7fcaed2a2e17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7fcaed2a3857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7fcaed2a36e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7fcaed2a06d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7fcaed2a33f2 - rust_begin_unwind
   9:     0x7fcaed253e43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7fcaed253d0d - core::panicking::panic::he60abbc443b65679
  11:     0x7fcaedc29a93 - std::panicking::try::cleanup::h391eb9fc6dbb46f3
  12:     0x7fcaef67f265 - std[3373497bae73761a]::panic::catch_unwind::<core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[64117771aba3b63a]::sync::par_for_each_in<alloc[e7b8874fa83489af]::vec::Vec<rustc_middle[6cc332b257bc4296]::mir::mono::MonoItem>, rustc_monomorphize[15fafc6d2dde90a9]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>, ()>
  13:     0x7fcaef6554ec - <rustc_session[8894c466f148e9fc]::session::Session>::time::<(), rustc_monomorphize[15fafc6d2dde90a9]::collector::collect_crate_mono_items::{closure#1}>
  14:     0x7fcaef632349 - rustc_monomorphize[15fafc6d2dde90a9]::collector::collect_crate_mono_items
  15:     0x7fcaef649c2c - rustc_monomorphize[15fafc6d2dde90a9]::partitioning::collect_and_partition_mono_items
  16:     0x7fcaef995c91 - rustc_query_system[f869d2416888f8e0]::query::plumbing::try_execute_query::<rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt, rustc_query_system[f869d2416888f8e0]::query::caches::DefaultCache<(), (&std[3373497bae73761a]::collections::hash::set::HashSet<rustc_span[fa21936b1d52bae]::def_id::DefId, core[1078bc74954e7e1d]::hash::BuildHasherDefault<rustc_hash[781c97b3372b41ea]::FxHasher>>, &[rustc_middle[6cc332b257bc4296]::mir::mono::CodegenUnit])>>
  17:     0x7fcaefa6605c - rustc_query_system[f869d2416888f8e0]::query::plumbing::get_query::<rustc_query_impl[2ee80727772d1cf9]::queries::collect_and_partition_mono_items, rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt>
  18:     0x7fcaefe88e05 - <rustc_query_impl[2ee80727772d1cf9]::Queries as rustc_middle[6cc332b257bc4296]::ty::query::QueryEngine>::collect_and_partition_mono_items
  19:     0x7fcaee51f137 - rustc_codegen_ssa[2923288115558b34]::base::codegen_crate::<rustc_codegen_llvm[be50a40afd9aa378]::LlvmCodegenBackend>
  20:     0x7fcaee4a86fd - <rustc_codegen_llvm[be50a40afd9aa378]::LlvmCodegenBackend as rustc_codegen_ssa[2923288115558b34]::traits::backend::CodegenBackend>::codegen_crate
  21:     0x7fcaeedaeceb - <rustc_session[8894c466f148e9fc]::session::Session>::time::<alloc[e7b8874fa83489af]::boxed::Box<dyn core[1078bc74954e7e1d]::any::Any>, rustc_interface[39f9eac39b7bbe47]::passes::start_codegen::{closure#0}>
  22:     0x7fcaeed70372 - <rustc_interface[39f9eac39b7bbe47]::passes::QueryContext>::enter::<<rustc_interface[39f9eac39b7bbe47]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[1078bc74954e7e1d]::result::Result<alloc[e7b8874fa83489af]::boxed::Box<dyn core[1078bc74954e7e1d]::any::Any>, rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  23:     0x7fcaeed525bd - <rustc_interface[39f9eac39b7bbe47]::queries::Queries>::ongoing_codegen
  24:     0x7fcaee8aaa69 - <rustc_interface[39f9eac39b7bbe47]::interface::Compiler>::enter::<rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}::{closure#2}, core[1078bc74954e7e1d]::result::Result<core[1078bc74954e7e1d]::option::Option<rustc_interface[39f9eac39b7bbe47]::queries::Linker>, rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  25:     0x7fcaee892a0a - rustc_span[fa21936b1d52bae]::with_source_map::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#1}>
  26:     0x7fcaee8c1fe8 - rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>
  27:     0x7fcaee88e491 - <scoped_tls[1ad84bfc992b6a7c]::ScopedKey<rustc_span[fa21936b1d52bae]::SessionGlobals>>::set::<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  28:     0x7fcaee9091b8 - std[3373497bae73761a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  29:     0x7fcaee894b9e - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x7fcaee90a9e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7fcaf0b9c645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  32:     0x7fcaed04cb43 - <unknown>
  33:     0x7fcaed0dea00 - <unknown>
  34:                0x0 - <unknown>
------------------------------------------


---- [ui] src/test/ui/higher-rank-trait-bounds/issue-95034.rs stdout ----
---- [ui] src/test/ui/higher-rank-trait-bounds/issue-95034.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/higher-rank-trait-bounds/issue-95034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-95034" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/higher-rank-trait-bounds/issue-95034/auxiliary"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'std::future::from_generator::GenFuture<[static generator@/checkout/src/test/ui/higher-rank-trait-bounds/issue-95034.rs:93:49: 95:6]>', compiler/rustc_traits/src/normalize_erasing_regions.rs:51:17
thread '<unnamed>' panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
stack backtrace:
stack backtrace:
   0:     0x7f6d9ffdc15c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f6da0044c98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f6d9ffcc9d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f6d9ffdf14e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f6d9ffdee17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f6d9ffdf857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f6d9ffdf6e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f6d9ffdc6d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f6d9ffdf3f2 - rust_begin_unwind
   9:     0x7f6d9ff8fe43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f6d9ff8fd0d - core::panicking::panic::he60abbc443b65679
  11:     0x7f6da0965a93 - std::panicking::try::cleanup::h391eb9fc6dbb46f3
  12:     0x7f6da15d0bba - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  13:     0x7f6da16469e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  14:     0x7f6da38d8645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  15:     0x7f6d9fd88b43 - <unknown>
  16:     0x7f6d9fe1aa00 - <unknown>
  17:                0x0 - <unknown>
------------------------------------------


---- [ui] src/test/ui/impl-trait/issues/issue-86800.rs stdout ----
---- [ui] src/test/ui/impl-trait/issues/issue-86800.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-86800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-Z" "treat-err-as-bug=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-86800/auxiliary"
stdout: none
--- stderr -------------------------------
thread '<unnamed>' panicked at 'assertion failed: self.scc_universes[scc] == ty::UniverseIndex::ROOT', compiler/rustc_borrowck/src/region_infer/mod.rs:718:9
   0:     0x7f412009115c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f41200f9c98 - core::fmt::write::h9da8e2abe157b8f7
   1:     0x7f41200f9c98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f41200819d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f412009414e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f4120093e17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f4120094857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f41200946e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f41200916d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f41200943f2 - rust_begin_unwind
   9:     0x7f4120044e43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f4120044d0d - core::panicking::panic::he60abbc443b65679
  11:     0x7f41211097c9 - <rustc_borrowck[b3e3fc1327c10565]::region_infer::RegionInferenceContext>::solve
  12:     0x7f4120edd3c0 - rustc_borrowck[b3e3fc1327c10565]::nll::compute_regions
  13:     0x7f41210a1cbd - rustc_borrowck[b3e3fc1327c10565]::do_mir_borrowck
  14:     0x7f4120e6dd1f - <rustc_infer[162f4bfdde2ac72f]::infer::InferCtxtBuilder>::enter::<rustc_middle[6cc332b257bc4296]::mir::query::BorrowCheckResult, rustc_borrowck[b3e3fc1327c10565]::mir_borrowck::{closure#0}>
  15:     0x7f4121092eb1 - rustc_borrowck[b3e3fc1327c10565]::mir_borrowck
  16:     0x7f4121061dee - <rustc_borrowck[b3e3fc1327c10565]::provide::{closure#0} as core[1078bc74954e7e1d]::ops::function::FnOnce<(rustc_middle[6cc332b257bc4296]::ty::context::TyCtxt, rustc_span[fa21936b1d52bae]::def_id::LocalDefId)>>::call_once
  17:     0x7f412273b294 - rustc_query_system[f869d2416888f8e0]::query::plumbing::try_execute_query::<rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt, rustc_query_system[f869d2416888f8e0]::query::caches::DefaultCache<rustc_span[fa21936b1d52bae]::def_id::LocalDefId, &rustc_middle[6cc332b257bc4296]::mir::query::BorrowCheckResult>>
  18:     0x7f412280f116 - rustc_query_system[f869d2416888f8e0]::query::plumbing::get_query::<rustc_query_impl[2ee80727772d1cf9]::queries::mir_borrowck, rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt>
  19:     0x7f4122c37bad - <rustc_query_impl[2ee80727772d1cf9]::Queries as rustc_middle[6cc332b257bc4296]::ty::query::QueryEngine>::mir_borrowck
  20:     0x7f41236c3cbd - <rustc_typeck[12f86a3a00abca02]::collect::type_of::find_opaque_ty_constraints_for_tait::ConstraintLocator>::check
  21:     0x7f41236c2cdb - <rustc_typeck[12f86a3a00abca02]::collect::type_of::find_opaque_ty_constraints_for_tait::ConstraintLocator as rustc_hir[204ea0cc06337379]::intravisit::Visitor>::visit_item
  22:     0x7f41238c4be3 - rustc_hir[204ea0cc06337379]::intravisit::walk_mod::<rustc_typeck[12f86a3a00abca02]::collect::type_of::find_opaque_ty_constraints_for_tait::ConstraintLocator>
  23:     0x7f41236c20b2 - rustc_typeck[12f86a3a00abca02]::collect::type_of::find_opaque_ty_constraints_for_tait
  24:     0x7f41236b5b02 - rustc_typeck[12f86a3a00abca02]::collect::type_of::type_of
  25:     0x7f4122756839 - rustc_query_system[f869d2416888f8e0]::query::plumbing::try_execute_query::<rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt, rustc_query_system[f869d2416888f8e0]::query::caches::DefaultCache<rustc_span[fa21936b1d52bae]::def_id::DefId, rustc_middle[6cc332b257bc4296]::ty::Ty>>
  26:     0x7f4122860ca8 - rustc_query_system[f869d2416888f8e0]::query::plumbing::get_query::<rustc_query_impl[2ee80727772d1cf9]::queries::type_of, rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt>
  27:     0x7f4122c0b812 - <rustc_query_impl[2ee80727772d1cf9]::Queries as rustc_middle[6cc332b257bc4296]::ty::query::QueryEngine>::type_of
  28:     0x7f41237da23f - rustc_typeck[12f86a3a00abca02]::check::check::check_opaque
  29:     0x7f41237dd100 - rustc_typeck[12f86a3a00abca02]::check::check::check_item_type
  30:     0x7f41237e74fa - rustc_typeck[12f86a3a00abca02]::check::check::check_mod_item_types
  31:     0x7f412273dc54 - rustc_query_system[f869d2416888f8e0]::query::plumbing::try_execute_query::<rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt, rustc_query_system[f869d2416888f8e0]::query::caches::DefaultCache<rustc_span[fa21936b1d52bae]::def_id::LocalDefId, ()>>
  32:     0x7f412282c566 - rustc_query_system[f869d2416888f8e0]::query::plumbing::get_query::<rustc_query_impl[2ee80727772d1cf9]::queries::check_mod_item_types, rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt>
  33:     0x7f4122c30200 - <rustc_query_impl[2ee80727772d1cf9]::Queries as rustc_middle[6cc332b257bc4296]::ty::query::QueryEngine>::check_mod_item_types
  34:     0x7f4123697b3a - <rustc_middle[6cc332b257bc4296]::hir::map::Map>::for_each_module::<rustc_typeck[12f86a3a00abca02]::check_crate::{closure#6}::{closure#0}>
  35:     0x7f41236c6332 - <rustc_session[8894c466f148e9fc]::session::Session>::time::<(), rustc_typeck[12f86a3a00abca02]::check_crate::{closure#6}>
  36:     0x7f4123904568 - rustc_typeck[12f86a3a00abca02]::check_crate
  37:     0x7f4121b620e1 - rustc_interface[39f9eac39b7bbe47]::passes::analysis
  38:     0x7f412277c224 - rustc_query_system[f869d2416888f8e0]::query::plumbing::try_execute_query::<rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt, rustc_query_system[f869d2416888f8e0]::query::caches::DefaultCache<(), core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>>
  39:     0x7f4122860dc4 - rustc_query_system[f869d2416888f8e0]::query::plumbing::get_query::<rustc_query_impl[2ee80727772d1cf9]::queries::analysis, rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt>
  40:     0x7f4122c0bfa9 - <rustc_query_impl[2ee80727772d1cf9]::Queries as rustc_middle[6cc332b257bc4296]::ty::query::QueryEngine>::analysis
  41:     0x7f41216efba4 - <rustc_interface[39f9eac39b7bbe47]::passes::QueryContext>::enter::<rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  42:     0x7f412169b9f7 - <rustc_interface[39f9eac39b7bbe47]::interface::Compiler>::enter::<rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}::{closure#2}, core[1078bc74954e7e1d]::result::Result<core[1078bc74954e7e1d]::option::Option<rustc_interface[39f9eac39b7bbe47]::queries::Linker>, rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  43:     0x7f4121683a0a - rustc_span[fa21936b1d52bae]::with_source_map::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#1}>
  44:     0x7f41216b2fe8 - rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>
  45:     0x7f412167f491 - <scoped_tls[1ad84bfc992b6a7c]::ScopedKey<rustc_span[fa21936b1d52bae]::SessionGlobals>>::set::<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  46:     0x7f41216fa1b8 - std[3373497bae73761a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  47:     0x7f4121685b9e - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  48:     0x7f41216fb9e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7f412398d645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  50:     0x7f411fe3db43 - <unknown>
  51:     0x7f411fecfa00 - <unknown>
  52:                0x0 - <unknown>
thread '<unnamed>' panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
   0:     0x7f412009115c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f41200f9c98 - core::fmt::write::h9da8e2abe157b8f7
   1:     0x7f41200f9c98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f41200819d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f412009414e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f4120093e17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f4120094857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f41200946e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f41200916d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f41200943f2 - rust_begin_unwind
   9:     0x7f4120044e43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f4120044d0d - core::panicking::panic::he60abbc443b65679
  11:     0x7f4120a1aa93 - std::panicking::try::cleanup::h391eb9fc6dbb46f3
  12:     0x7f4121685bba - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  13:     0x7f41216fb9e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  14:     0x7f412398d645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  15:     0x7f411fe3db43 - <unknown>
  16:     0x7f411fecfa00 - <unknown>
  17:                0x0 - <unknown>
------------------------------------------


---- [ui] src/test/ui/lto/issue-11154.rs stdout ----
---- [ui] src/test/ui/lto/issue-11154.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/issue-11154.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-11154" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-C" "prefer-dynamic" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/issue-11154/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/panics/default-backtrace-ice.rs stdout ----


error: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/default-backtrace-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "treat-err-as-bug=1" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/default-backtrace-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `missing_ident` in this scope
   |
   |
LL | fn main() { missing_ident; }


thread '<unnamed>' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1523:27
stack backtrace:
   0:     0x7f02cbda315c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f02cbe0bc98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f02cbd939d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f02cbda614e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f02cbda5e17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f02cbda6857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f02cbda66e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f02cbda36d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f02cbda63f2 - rust_begin_unwind
   9:     0x7f02cbd56e43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f02cd48b69d - <rustc_errors[6ac68d8fd66c57be]::HandlerInner>::panic_if_treat_err_as_bug
  11:     0x7f02cd48ae80 - <rustc_errors[6ac68d8fd66c57be]::HandlerInner>::emit_diagnostic
  12:     0x7f02cd486a86 - <rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed as rustc_errors[6ac68d8fd66c57be]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  13:     0x7f02ceaa2ed0 - <rustc_resolve[2ecee4004edca970]::Resolver>::report_errors
  14:     0x7f02ceb30a41 - <rustc_session[8894c466f148e9fc]::session::Session>::time::<(), <rustc_resolve[2ecee4004edca970]::Resolver>::resolve_crate::{closure#0}>
  15:     0x7f02cd86f0ea - <rustc_interface[39f9eac39b7bbe47]::passes::boxed_resolver::BoxedResolver>::access::<<rustc_interface[39f9eac39b7bbe47]::queries::Queries>::expansion::{closure#0}::{closure#0}, core[1078bc74954e7e1d]::result::Result<rustc_ast[b8ef76688071ff77]::ast::Crate, rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  16:     0x7f02cd854187 - <rustc_interface[39f9eac39b7bbe47]::queries::Queries>::expansion
  17:     0x7f02cd3ad86c - <rustc_interface[39f9eac39b7bbe47]::interface::Compiler>::enter::<rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}::{closure#2}, core[1078bc74954e7e1d]::result::Result<core[1078bc74954e7e1d]::option::Option<rustc_interface[39f9eac39b7bbe47]::queries::Linker>, rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  18:     0x7f02cd395a0a - rustc_span[fa21936b1d52bae]::with_source_map::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#1}>
  19:     0x7f02cd3c4fe8 - rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>
  20:     0x7f02cd391491 - <scoped_tls[1ad84bfc992b6a7c]::ScopedKey<rustc_span[fa21936b1d52bae]::SessionGlobals>>::set::<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  21:     0x7f02cd40c1b8 - std[3373497bae73761a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  22:     0x7f02cd397b9e - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  23:     0x7f02cd40d9e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  24:     0x7f02cf69f645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  25:     0x7f02cbb4fb43 - <unknown>
  26:     0x7f02cbbe1a00 - <unknown>
  27:                0x0 - <unknown>
thread '<unnamed>' panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
stack backtrace:
   0:     0x7f02cbda315c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f02cbe0bc98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f02cbd939d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f02cbda614e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f02cbda5e17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f02cbda6857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f02cbda66e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f02cbda36d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f02cbda63f2 - rust_begin_unwind
   9:     0x7f02cbd56e43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f02cbd56d0d - core::panicking::panic::he60abbc443b65679
  11:     0x7f02cc72ca93 - std::panicking::try::cleanup::h391eb9fc6dbb46f3
  12:     0x7f02cd397bba - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  13:     0x7f02cd40d9e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  14:     0x7f02cf69f645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  15:     0x7f02cbb4fb43 - <unknown>
  16:     0x7f02cbbe1a00 - <unknown>
  17:                0x0 - <unknown>
------------------------------------------


---- [ui] src/test/ui/treat-err-as-bug/delay_span_bug.rs stdout ----
---- [ui] src/test/ui/treat-err-as-bug/delay_span_bug.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/delay_span_bug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/delay_span_bug/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: delayed span bug triggered by #[rustc_error(delay_span_bug_from_inside_query)]
   |
LL | fn main() {}
   | ^^^^^^^^^


thread '<unnamed>' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1523:27
thread '<unnamed>' panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
stack backtrace:
stack backtrace:
   0:     0x7f2a04f8c15c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f2a04ff4c98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f2a04f7c9d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f2a04f8f14e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f2a04f8ee17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f2a04f8f857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f2a04f8f6e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f2a04f8c6d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f2a04f8f3f2 - rust_begin_unwind
   9:     0x7f2a04f3fe43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f2a04f3fd0d - core::panicking::panic::he60abbc443b65679
  11:     0x7f2a05915a93 - std::panicking::try::cleanup::h391eb9fc6dbb46f3
  12:     0x7f2a06580bba - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  13:     0x7f2a065f69e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  14:     0x7f2a08888645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  15:     0x7f2a04d38b43 - <unknown>
  16:     0x7f2a04dcaa00 - <unknown>
  17:                0x0 - <unknown>
------------------------------------------


---- [ui] src/test/ui/treat-err-as-bug/err.rs stdout ----
---- [ui] src/test/ui/treat-err-as-bug/err.rs stdout ----

error: Error: expected failure status (Some(101)) but received status None.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/treat-err-as-bug/err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Ztreat-err-as-bug" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/treat-err-as-bug/err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL | pub static C: u32 = 0 - 1;


thread '<unnamed>' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1523:27
thread '<unnamed>' panicked at 'attempt to subtract with overflow', library/std/src/panicking.rs:363:24
stack backtrace:
stack backtrace:
   0:     0x7f2473bff15c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5be8b79f88b848eb
   1:     0x7f2473c67c98 - core::fmt::write::h9da8e2abe157b8f7
   2:     0x7f2473bef9d1 - std::io::Write::write_fmt::h933fa6d37a85da54
   3:     0x7f2473c0214e - std::panicking::default_hook::{{closure}}::hb3eb994baf7a37dc
   4:     0x7f2473c01e17 - std::panicking::default_hook::heee06dec453b4dd7
   5:     0x7f2473c02857 - std::panicking::rust_panic_with_hook::h2a7a53891745ae29
   6:     0x7f2473c026e9 - std::panicking::begin_panic_handler::{{closure}}::h5044c6ca0d0b7ad0
   7:     0x7f2473bff6d4 - std::sys_common::backtrace::__rust_end_short_backtrace::h068a34b257dc6f49
   8:     0x7f2473c023f2 - rust_begin_unwind
   9:     0x7f2473bb2e43 - core::panicking::panic_fmt::h90b3ab9441172d65
  10:     0x7f2473bb2d0d - core::panicking::panic::he60abbc443b65679
  11:     0x7f2474588a93 - std::panicking::try::cleanup::h391eb9fc6dbb46f3
  12:     0x7f247568e59c - rustc_data_structures[64117771aba3b63a]::sync::par_for_each_in::<&[rustc_span[fa21936b1d52bae]::def_id::LocalDefId], <rustc_middle[6cc332b257bc4296]::hir::map::Map>::par_for_each_module<rustc_lint[c79f200012caa85a]::late::check_crate<rustc_lint[c79f200012caa85a]::BuiltinCombinedLateLintPass, rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  13:     0x7f247570ee54 - <rustc_session[8894c466f148e9fc]::session::Session>::time::<(), rustc_lint[c79f200012caa85a]::late::check_crate<rustc_lint[c79f200012caa85a]::BuiltinCombinedLateLintPass, rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  14:     0x7f247570efb0 - <rustc_session[8894c466f148e9fc]::session::Session>::time::<(), rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  15:     0x7f247568df28 - std[3373497bae73761a]::panic::catch_unwind::<core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  16:     0x7f2475734f35 - <core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}::{closure#1}> as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once
  17:     0x7f247568e049 - std[3373497bae73761a]::panic::catch_unwind::<core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}::{closure#1}>, ()>
  18:     0x7f2475710ee6 - <rustc_session[8894c466f148e9fc]::session::Session>::time::<(), rustc_interface[39f9eac39b7bbe47]::passes::analysis::{closure#5}>
  19:     0x7f24756d0180 - rustc_interface[39f9eac39b7bbe47]::passes::analysis
  20:     0x7f24762ea224 - rustc_query_system[f869d2416888f8e0]::query::plumbing::try_execute_query::<rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt, rustc_query_system[f869d2416888f8e0]::query::caches::DefaultCache<(), core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>>
  21:     0x7f24763cedc4 - rustc_query_system[f869d2416888f8e0]::query::plumbing::get_query::<rustc_query_impl[2ee80727772d1cf9]::queries::analysis, rustc_query_impl[2ee80727772d1cf9]::plumbing::QueryCtxt>
  22:     0x7f2476779fa9 - <rustc_query_impl[2ee80727772d1cf9]::Queries as rustc_middle[6cc332b257bc4296]::ty::query::QueryEngine>::analysis
  23:     0x7f247525dba4 - <rustc_interface[39f9eac39b7bbe47]::passes::QueryContext>::enter::<rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  24:     0x7f24752099f7 - <rustc_interface[39f9eac39b7bbe47]::interface::Compiler>::enter::<rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}::{closure#2}, core[1078bc74954e7e1d]::result::Result<core[1078bc74954e7e1d]::option::Option<rustc_interface[39f9eac39b7bbe47]::queries::Linker>, rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  25:     0x7f24751f1a0a - rustc_span[fa21936b1d52bae]::with_source_map::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#1}>
  26:     0x7f2475220fe8 - rustc_interface[39f9eac39b7bbe47]::interface::create_compiler_and_run::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>
  27:     0x7f24751ed491 - <scoped_tls[1ad84bfc992b6a7c]::ScopedKey<rustc_span[fa21936b1d52bae]::SessionGlobals>>::set::<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  28:     0x7f24752681b8 - std[3373497bae73761a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>
  29:     0x7f24751f3b9e - std[3373497bae73761a]::panicking::try::<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, core[1078bc74954e7e1d]::panic::unwind_safe::AssertUnwindSafe<<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  30:     0x7f24752699e9 - <<std[3373497bae73761a]::thread::Builder>::spawn_unchecked_<rustc_interface[39f9eac39b7bbe47]::util::run_in_thread_pool_with_globals<rustc_interface[39f9eac39b7bbe47]::interface::run_compiler<core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>, rustc_driver[7ec31ef4a10e18f3]::run_compiler::{closure#1}>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#0}, core[1078bc74954e7e1d]::result::Result<(), rustc_errors[6ac68d8fd66c57be]::ErrorGuaranteed>>::{closure#1} as core[1078bc74954e7e1d]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f24774fb645 - std::sys::unix::thread::Thread::new::thread_start::h434ba9bc0c7af2ff
  32:     0x7f24739abb43 - <unknown>
  33:     0x7f2473a3da00 - <unknown>
  34:                0x0 - <unknown>
------------------------------------------



