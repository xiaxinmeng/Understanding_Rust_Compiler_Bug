
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:198:90: Failed to normalize <for<'a, 'b> fn(&'a Alias<'b>) {foo} as std::ops::FnOnce<(&&S,)>>::Output, maybe try to call `try_normalize_erasing_regions` instead
thread 'rustc' panicked at 'Box<dyn Any>', /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/compiler/rustc_errors/src/lib.rs:1519:9
stack backtrace:
   0:     0x7f51d9f74390 - std::backtrace_rs::backtrace::libunwind::trace::haefc0dd0a0a62b67
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f51d9f74390 - std::backtrace_rs::backtrace::trace_unsynchronized::h231f73070a372c1c
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f51d9f74390 - std::sys_common::backtrace::_print_fmt::haa15dcb5c660c296
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f51d9f74390 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfcfc314fff15b0e4
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f51d61696ce - core::fmt::write::h2e893235039ae031
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f51d9f680e5 - std::io::Write::write_fmt::h310687acf0f328ac
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/io/mod.rs:1682:15
   6:     0x7f51d9f74155 - std::sys_common::backtrace::_print::h7594904cd4694801
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f51d9f74155 - std::sys_common::backtrace::print::hc63fb4b3dff70d7e
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f51d9f764af - std::panicking::default_hook::{{closure}}::h6ab59645549c33df
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:267:22
   9:     0x7f51d9f761ea - std::panicking::default_hook::h9d471aa0319ef778
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:286:9
  10:     0x7f51d9238f14 - <rustc_driver[d75d50cb7f36eddb]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[1359fc6eb4466362]::ops::function::FnOnce<(&core[1359fc6eb4466362]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f51d9f76c99 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h4001808d80334131
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/alloc/src/boxed.rs:2001:9
  12:     0x7f51d9f76c99 - std::panicking::rust_panic_with_hook::hedb82473dd3f5a0d
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/panicking.rs:692:13
  13:     0x7f51d965f391 - std[aff2bf53d717ad31]::panicking::begin_panic::<rustc_errors[192e0008dc26bff9]::ExplicitBug>::{closure#0}
  14:     0x7f51d965eef6 - std[aff2bf53d717ad31]::sys_common::backtrace::__rust_end_short_backtrace::<std[aff2bf53d717ad31]::panicking::begin_panic<rustc_errors[192e0008dc26bff9]::ExplicitBug>::{closure#0}, !>
  15:     0x7f51d96c83c6 - std[aff2bf53d717ad31]::panicking::begin_panic::<rustc_errors[192e0008dc26bff9]::ExplicitBug>
  16:     0x7f51d965eee6 - std[aff2bf53d717ad31]::panic::panic_any::<rustc_errors[192e0008dc26bff9]::ExplicitBug>
  17:     0x7f51d965ddbd - <rustc_errors[192e0008dc26bff9]::HandlerInner>::bug::<&alloc[648b0635dcc328ab]::string::String>
  18:     0x7f51d965d830 - <rustc_errors[192e0008dc26bff9]::Handler>::bug::<&alloc[648b0635dcc328ab]::string::String>
  19:     0x7f51d97121dd - rustc_middle[35b10629088ad2a7]::ty::context::tls::with_context_opt::<rustc_middle[35b10629088ad2a7]::ty::context::tls::with_opt<rustc_middle[35b10629088ad2a7]::util::bug::opt_span_bug_fmt<rustc_span[4a72619fff0c1ddc]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7f51d9712536 - rustc_middle[35b10629088ad2a7]::util::bug::opt_span_bug_fmt::<rustc_span[4a72619fff0c1ddc]::span_encoding::Span>
  21:     0x7f51d7506293 - rustc_middle[35b10629088ad2a7]::util::bug::bug_fmt
  22:     0x7f51d743ef42 - <rustc_middle[35b10629088ad2a7]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[35b10629088ad2a7]::ty::fold::TypeFolder>::fold_ty
  23:     0x7f51d8ab2cbc - rustc_middle[35b10629088ad2a7]::ty::util::fold_list::<rustc_middle[35b10629088ad2a7]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder, rustc_middle[35b10629088ad2a7]::ty::Ty, <&rustc_middle[35b10629088ad2a7]::ty::list::List<rustc_middle[35b10629088ad2a7]::ty::Ty> as rustc_middle[35b10629088ad2a7]::ty::fold::TypeFoldable>::try_fold_with<rustc_middle[35b10629088ad2a7]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::{closure#0}>
  24:     0x7f51d7ed868c - <rustc_middle[35b10629088ad2a7]::ty::context::TyCtxt>::normalize_erasing_late_bound_regions::<rustc_middle[35b10629088ad2a7]::ty::sty::FnSig>
  25:     0x7f51d7e67297 - <rustc_const_eval[c2053576d6089077]::interpret::eval_context::InterpCx<rustc_const_eval[c2053576d6089077]::const_eval::machine::CompileTimeInterpreter>>::run
  26:     0x7f51d7e5668e - rustc_const_eval[c2053576d6089077]::const_eval::eval_queries::eval_to_allocation_raw_provider
  27:     0x7f51d85a924b - rustc_query_system[b620ddc51e5ea697]::query::plumbing::try_execute_query::<rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt, rustc_query_system[b620ddc51e5ea697]::query::caches::DefaultCache<rustc_middle[35b10629088ad2a7]::ty::ParamEnvAnd<rustc_middle[35b10629088ad2a7]::mir::interpret::GlobalId>, core[1359fc6eb4466362]::result::Result<rustc_middle[35b10629088ad2a7]::mir::interpret::value::ConstAlloc, rustc_middle[35b10629088ad2a7]::mir::interpret::error::ErrorHandled>>>
  28:     0x7f51d8c607c6 - <rustc_query_impl[5c9b5ec5f9c83566]::Queries as rustc_middle[35b10629088ad2a7]::ty::query::QueryEngine>::eval_to_allocation_raw
  29:     0x7f51d858af6a - rustc_const_eval[c2053576d6089077]::const_eval::eval_queries::eval_to_const_value_raw_provider
  30:     0x7f51d7fa41cc - rustc_query_system[b620ddc51e5ea697]::query::plumbing::try_execute_query::<rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt, rustc_query_system[b620ddc51e5ea697]::query::caches::DefaultCache<rustc_middle[35b10629088ad2a7]::ty::ParamEnvAnd<rustc_middle[35b10629088ad2a7]::mir::interpret::GlobalId>, core[1359fc6eb4466362]::result::Result<rustc_middle[35b10629088ad2a7]::mir::interpret::value::ConstValue, rustc_middle[35b10629088ad2a7]::mir::interpret::error::ErrorHandled>>>
  31:     0x7f51d8c60959 - <rustc_query_impl[5c9b5ec5f9c83566]::Queries as rustc_middle[35b10629088ad2a7]::ty::query::QueryEngine>::eval_to_const_value_raw
  32:     0x7f51d858b386 - rustc_const_eval[c2053576d6089077]::const_eval::eval_queries::eval_to_const_value_raw_provider
  33:     0x7f51d7fa41cc - rustc_query_system[b620ddc51e5ea697]::query::plumbing::try_execute_query::<rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt, rustc_query_system[b620ddc51e5ea697]::query::caches::DefaultCache<rustc_middle[35b10629088ad2a7]::ty::ParamEnvAnd<rustc_middle[35b10629088ad2a7]::mir::interpret::GlobalId>, core[1359fc6eb4466362]::result::Result<rustc_middle[35b10629088ad2a7]::mir::interpret::value::ConstValue, rustc_middle[35b10629088ad2a7]::mir::interpret::error::ErrorHandled>>>
  34:     0x7f51d8c60959 - <rustc_query_impl[5c9b5ec5f9c83566]::Queries as rustc_middle[35b10629088ad2a7]::ty::query::QueryEngine>::eval_to_const_value_raw
  35:     0x7f51d83a0fdc - <rustc_middle[35b10629088ad2a7]::ty::query::TyCtxtEnsure>::const_eval_poly
  36:     0x7f51d839c5f7 - <rustc_lint[24d4f045bf5e69d4]::BuiltinCombinedModuleLateLintPass as rustc_lint[24d4f045bf5e69d4]::passes::LateLintPass>::check_item
  37:     0x7f51d8398a7d - rustc_hir[441ce2da0a2c76d7]::intravisit::walk_mod::<rustc_lint[24d4f045bf5e69d4]::late::LateContextAndPass<rustc_lint[24d4f045bf5e69d4]::BuiltinCombinedModuleLateLintPass>>
  38:     0x7f51d8397fda - rustc_lint[24d4f045bf5e69d4]::late::late_lint_mod::<rustc_lint[24d4f045bf5e69d4]::BuiltinCombinedModuleLateLintPass>
  39:     0x7f51d8397e2c - rustc_lint[24d4f045bf5e69d4]::lint_mod
  40:     0x7f51d7bd5648 - rustc_query_system[b620ddc51e5ea697]::query::plumbing::try_execute_query::<rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt, rustc_query_system[b620ddc51e5ea697]::query::caches::DefaultCache<rustc_span[4a72619fff0c1ddc]::def_id::LocalDefId, ()>>
  41:     0x7f51d86de9f9 - rustc_query_system[b620ddc51e5ea697]::query::plumbing::get_query::<rustc_query_impl[5c9b5ec5f9c83566]::queries::lint_mod, rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt>
  42:     0x7f51d89122a8 - rustc_data_structures[9a3f2338e9c2b80]::sync::par_for_each_in::<&[rustc_hir[441ce2da0a2c76d7]::hir_id::OwnerId], <rustc_middle[35b10629088ad2a7]::hir::map::Map>::par_for_each_module<rustc_lint[24d4f045bf5e69d4]::late::check_crate<rustc_lint[24d4f045bf5e69d4]::BuiltinCombinedLateLintPass, rustc_interface[a64a4e4385c65420]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  43:     0x7f51d8911fae - <rustc_session[4ded857a98b896bc]::session::Session>::time::<(), rustc_lint[24d4f045bf5e69d4]::late::check_crate<rustc_lint[24d4f045bf5e69d4]::BuiltinCombinedLateLintPass, rustc_interface[a64a4e4385c65420]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  44:     0x7f51d8911b8a - <rustc_session[4ded857a98b896bc]::session::Session>::time::<(), rustc_interface[a64a4e4385c65420]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  45:     0x7f51d895e42f - <core[1359fc6eb4466362]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[a64a4e4385c65420]::passes::analysis::{closure#5}::{closure#1}> as core[1359fc6eb4466362]::ops::function::FnOnce<()>>::call_once
  46:     0x7f51d779f067 - <rustc_session[4ded857a98b896bc]::session::Session>::time::<(), rustc_interface[a64a4e4385c65420]::passes::analysis::{closure#5}>
  47:     0x7f51d779b5d6 - rustc_interface[a64a4e4385c65420]::passes::analysis
  48:     0x7f51d8a94fa4 - rustc_query_system[b620ddc51e5ea697]::query::plumbing::try_execute_query::<rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt, rustc_query_system[b620ddc51e5ea697]::query::caches::DefaultCache<(), core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>>
  49:     0x7f51d8a94ca7 - rustc_query_system[b620ddc51e5ea697]::query::plumbing::get_query::<rustc_query_impl[5c9b5ec5f9c83566]::queries::analysis, rustc_query_impl[5c9b5ec5f9c83566]::plumbing::QueryCtxt>
  50:     0x7f51d85c63cd - <rustc_interface[a64a4e4385c65420]::passes::QueryContext>::enter::<rustc_driver[d75d50cb7f36eddb]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  51:     0x7f51d85c2a3f - <rustc_interface[a64a4e4385c65420]::interface::Compiler>::enter::<rustc_driver[d75d50cb7f36eddb]::run_compiler::{closure#1}::{closure#2}, core[1359fc6eb4466362]::result::Result<core[1359fc6eb4466362]::option::Option<rustc_interface[a64a4e4385c65420]::queries::Linker>, rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  52:     0x7f51d85b9c32 - rustc_span[4a72619fff0c1ddc]::with_source_map::<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustc_interface[a64a4e4385c65420]::interface::run_compiler<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustc_driver[d75d50cb7f36eddb]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  53:     0x7f51d85b9729 - <scoped_tls[1f85de2fb8312e22]::ScopedKey<rustc_span[4a72619fff0c1ddc]::SessionGlobals>>::set::<rustc_interface[a64a4e4385c65420]::interface::run_compiler<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustc_driver[d75d50cb7f36eddb]::run_compiler::{closure#1}>::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  54:     0x7f51d85b8d38 - std[aff2bf53d717ad31]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a64a4e4385c65420]::util::run_in_thread_pool_with_globals<rustc_interface[a64a4e4385c65420]::interface::run_compiler<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustc_driver[d75d50cb7f36eddb]::run_compiler::{closure#1}>::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>
  55:     0x7f51d85b8a73 - <<std[aff2bf53d717ad31]::thread::Builder>::spawn_unchecked_<rustc_interface[a64a4e4385c65420]::util::run_in_thread_pool_with_globals<rustc_interface[a64a4e4385c65420]::interface::run_compiler<core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>, rustc_driver[d75d50cb7f36eddb]::run_compiler::{closure#1}>::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1359fc6eb4466362]::result::Result<(), rustc_errors[192e0008dc26bff9]::ErrorGuaranteed>>::{closure#1} as core[1359fc6eb4466362]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  56:     0x7f51d9f7dc03 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h27f3134faf8b42b4
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/alloc/src/boxed.rs:1987:9
  57:     0x7f51d9f7dc03 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h46d82bf285dffad3
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/alloc/src/boxed.rs:1987:9
  58:     0x7f51d9f7dc03 - std::sys::unix::thread::Thread::new::thread_start::hdb1ca70a964535d0
                               at /rustc/758f19645b8ebce61ea52d1f6672fd057bc8dbee/library/std/src/sys/unix/thread.rs:108:17
  59:     0x7f51d5fec609 - start_thread
  60:     0x7f51d5f0f133 - clone
  61:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (758f19645 2022-10-24) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `BAR`
#1 [eval_to_const_value_raw] simplifying constant for the type system `BAR`
#2 [eval_to_const_value_raw] simplifying constant for the type system `BAR`
#3 [lint_mod] linting top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
