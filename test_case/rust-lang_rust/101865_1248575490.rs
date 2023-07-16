plain
   Compiling profiler_builtins v0.0.0 (/checkout/library/profiler_builtins)
[RUSTC-TIMING] build_script_build test:false 0.178
[RUSTC-TIMING] build_script_build test:false 0.209
[RUSTC-TIMING] build_script_build test:false 0.433
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:337:1: `tcx.hir_owner_indices(DefId(0:177 ~ core[1f24]::f32::{impl#0}::to_be_bytes))` unsupported by its crate; perhaps the `hir_owner_indices` query was never assigned a provider function

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/compiler/rustc_errors/src/lib.rs:1458:9
   0:     0x7ffb16510570 - std::backtrace_rs::backtrace::libunwind::trace::hc124e0911c16566c
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7ffb16510570 - std::backtrace_rs::backtrace::trace_unsynchronized::h1002d393d1719d00
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffb16510570 - std::sys_common::backtrace::_print_fmt::ha7087e4ba56c5a26
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7ffb16510570 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb33fe85e879df56b
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7ffb1656f8cc - core::fmt::write::hf970a4d7eb447230
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/core/src/fmt/mod.rs:1202:17
   5:     0x7ffb16500c95 - std::io::Write::write_fmt::h53f0aca5464009c6
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/io/mod.rs:1679:15
   6:     0x7ffb165133d1 - std::sys_common::backtrace::_print::ha72626d8989291c2
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7ffb165133d1 - std::sys_common::backtrace::print::h814497532f0c8a4b
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7ffb165133d1 - std::panicking::default_hook::{{closure}}::hed44907d634d668a
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:295:22
   9:     0x7ffb1651308e - std::panicking::default_hook::h1d80056e592acc25
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:314:9
  10:     0x7ffb133f3cf6 - <rustc_driver[609c8905cc717744]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[2bb8d0aa17313d7]::ops::function::FnOnce<(&core[2bb8d0aa17313d7]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7ffb16513c46 - std::panicking::rust_panic_with_hook::hc3c78944e414cef8
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:702:17
  12:     0x7ffb15c69903 - std[b078befb018cfa9a]::panicking::begin_panic::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>::{closure#0}
  13:     0x7ffb15c67236 - std[b078befb018cfa9a]::sys_common::backtrace::__rust_end_short_backtrace::<std[b078befb018cfa9a]::panicking::begin_panic<rustc_errors[1ff3bff36c6062df]::ExplicitBug>::{closure#0}, !>
  14:     0x7ffb15b52c96 - std[b078befb018cfa9a]::panicking::begin_panic::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>
  15:     0x7ffb15c670b6 - std[b078befb018cfa9a]::panic::panic_any::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>
  16:     0x7ffb15c55e73 - <rustc_errors[1ff3bff36c6062df]::HandlerInner>::bug::<&alloc[50cd12edb6646b8b]::string::String>
  17:     0x7ffb15c558e0 - <rustc_errors[1ff3bff36c6062df]::Handler>::bug::<&alloc[50cd12edb6646b8b]::string::String>
  18:     0x7ffb15d3275a - rustc_middle[d37df52345c2f8a6]::ty::context::tls::with_context_opt::<rustc_middle[d37df52345c2f8a6]::ty::context::tls::with_opt<rustc_middle[d37df52345c2f8a6]::util::bug::opt_span_bug_fmt<rustc_span[43b640252150d843]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  19:     0x7ffb15d35079 - rustc_middle[d37df52345c2f8a6]::util::bug::opt_span_bug_fmt::<rustc_span[43b640252150d843]::span_encoding::Span>
  20:     0x7ffb15d34ff5 - rustc_middle[d37df52345c2f8a6]::util::bug::bug_fmt
  21:     0x7ffb15ce56b3 - <<rustc_middle[d37df52345c2f8a6]::ty::query::Providers as core[2bb8d0aa17313d7]::default::Default>::default::{closure#11} as core[2bb8d0aa17313d7]::ops::function::FnOnce<(rustc_middle[d37df52345c2f8a6]::ty::context::TyCtxt, rustc_span[43b640252150d843]::def_id::LocalDefId)>>::call_once
  22:     0x7ffb14bcf20a - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<rustc_span[43b640252150d843]::def_id::LocalDefId, rustc_hir[85848aa7b45a13d0]::hir::MaybeOwner<&rustc_hir[85848aa7b45a13d0]::hir::OwnerIndexing>>>
  23:     0x7ffb14cb920f - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::hir_owner_indices, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  24:     0x7ffb14e9f449 - <rustc_query_impl[2b1202858a66a654]::Queries as rustc_middle[d37df52345c2f8a6]::ty::query::QueryEngine>::hir_owner_indices
  25:     0x7ffb15be2a73 - <rustc_middle[d37df52345c2f8a6]::hir::map::Map>::opt_local_def_id
  26:     0x7ffb15beb8c2 - <rustc_middle[d37df52345c2f8a6]::hir::map::ItemCollector as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_anon_const
  27:     0x7ffb15c6001f - rustc_hir[85848aa7b45a13d0]::intravisit::walk_fn::<rustc_middle[d37df52345c2f8a6]::hir::map::ItemCollector>
  28:     0x7ffb15c5ebbf - rustc_hir[85848aa7b45a13d0]::intravisit::walk_impl_item::<rustc_middle[d37df52345c2f8a6]::hir::map::ItemCollector>
  29:     0x7ffb15c62fd5 - rustc_hir[85848aa7b45a13d0]::intravisit::walk_item::<rustc_middle[d37df52345c2f8a6]::hir::map::ItemCollector>
  30:     0x7ffb15beb7e5 - <rustc_middle[d37df52345c2f8a6]::hir::map::ItemCollector as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_item
  31:     0x7ffb15c60c94 - rustc_hir[85848aa7b45a13d0]::intravisit::walk_mod::<rustc_middle[d37df52345c2f8a6]::hir::map::ItemCollector>
  32:     0x7ffb15beb259 - rustc_middle[d37df52345c2f8a6]::hir::map::hir_crate_items
  33:     0x7ffb14bb570f - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::ArenaCache<(), rustc_middle[d37df52345c2f8a6]::hir::ModuleItems>>
  34:     0x7ffb14cb183d - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::hir_crate_items, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  35:     0x7ffb1361eaa6 - rustc_interface[2bc777e04ad07507]::proc_macro_decls::proc_macro_decls_static
  36:     0x7ffb14c3aa48 - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<(), core[2bb8d0aa17313d7]::option::Option<rustc_span[43b640252150d843]::def_id::LocalDefId>>>
  37:     0x7ffb14cd0d40 - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::proc_macro_decls_static, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  38:     0x7ffb13579135 - <rustc_session[f2c77fdc990fe9ee]::session::Session>::time::<(), rustc_interface[2bc777e04ad07507]::passes::analysis::{closure#0}::{closure#0}::{closure#1}>
  39:     0x7ffb13579e27 - <rustc_session[f2c77fdc990fe9ee]::session::Session>::time::<(), rustc_interface[2bc777e04ad07507]::passes::analysis::{closure#0}>
  40:     0x7ffb13572c49 - rustc_interface[2bc777e04ad07507]::passes::analysis
  41:     0x7ffb14c3d54d - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<(), core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>>
  42:     0x7ffb14ce1970 - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::analysis, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  43:     0x7ffb13437339 - <rustc_interface[2bc777e04ad07507]::passes::QueryContext>::enter::<rustc_driver[609c8905cc717744]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  44:     0x7ffb13405c44 - rustc_interface[2bc777e04ad07507]::interface::create_compiler_and_run::<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>
  45:     0x7ffb13496f22 - <scoped_tls[da5adb31d26bd224]::ScopedKey<rustc_span[43b640252150d843]::SessionGlobals>>::set::<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  46:     0x7ffb1344792f - std[b078befb018cfa9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2bc777e04ad07507]::util::run_in_thread_pool_with_globals<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  47:     0x7ffb13448b49 - <<std[b078befb018cfa9a]::thread::Builder>::spawn_unchecked_<rustc_interface[2bc777e04ad07507]::util::run_in_thread_pool_with_globals<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#1} as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7ffb1651e243 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h06da489939a0fe31
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/alloc/src/boxed.rs:1940:9
  49:     0x7ffb1651e243 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h10f79067252c7632
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/alloc/src/boxed.rs:1940:9
  50:     0x7ffb1651e243 - std::sys::unix::thread::Thread::new::thread_start::ha0b3536ba188d794
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys/unix/thread.rs:108:17
  51:     0x7ffb11b87ea5 - start_thread
  52:     0x7ffb118b0b0d - clone
  53:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e395fd6fa 2022-09-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [hir_owner_indices] HIR owner indices in `f32::<impl at library/core/src/num/f32.rs:350:1: 350:9>::to_be_bytes`
#1 [hir_crate_items] get HIR crate items
#2 [proc_macro_decls_static] looking up the derive registrar for a crate
#3 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:337:1: `tcx.hir_owner_indices(DefId(0:24173 ~ core[1f24]::num::bignum::Big32x40))` unsupported by its crate; perhaps the `hir_owner_indices` query was never assigned a provider function

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/compiler/rustc_errors/src/lib.rs:1458:9
   0:     0x7ffb16510570 - std::backtrace_rs::backtrace::libunwind::trace::hc124e0911c16566c
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7ffb16510570 - std::backtrace_rs::backtrace::trace_unsynchronized::h1002d393d1719d00
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffb16510570 - std::sys_common::backtrace::_print_fmt::ha7087e4ba56c5a26
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7ffb16510570 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb33fe85e879df56b
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7ffb1656f8cc - core::fmt::write::hf970a4d7eb447230
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/core/src/fmt/mod.rs:1202:17
   5:     0x7ffb16500c95 - std::io::Write::write_fmt::h53f0aca5464009c6
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/io/mod.rs:1679:15
   6:     0x7ffb165133d1 - std::sys_common::backtrace::_print::ha72626d8989291c2
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7ffb165133d1 - std::sys_common::backtrace::print::h814497532f0c8a4b
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7ffb165133d1 - std::panicking::default_hook::{{closure}}::hed44907d634d668a
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:295:22
   9:     0x7ffb1651308e - std::panicking::default_hook::h1d80056e592acc25
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:314:9
  10:     0x7ffb133f3cf6 - <rustc_driver[609c8905cc717744]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[2bb8d0aa17313d7]::ops::function::FnOnce<(&core[2bb8d0aa17313d7]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7ffb16513c46 - std::panicking::rust_panic_with_hook::hc3c78944e414cef8
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:702:17
  12:     0x7ffb15c69903 - std[b078befb018cfa9a]::panicking::begin_panic::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>::{closure#0}
  13:     0x7ffb15c67236 - std[b078befb018cfa9a]::sys_common::backtrace::__rust_end_short_backtrace::<std[b078befb018cfa9a]::panicking::begin_panic<rustc_errors[1ff3bff36c6062df]::ExplicitBug>::{closure#0}, !>
  14:     0x7ffb15b52c96 - std[b078befb018cfa9a]::panicking::begin_panic::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>
  15:     0x7ffb15c670b6 - std[b078befb018cfa9a]::panic::panic_any::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>
  16:     0x7ffb15c55e73 - <rustc_errors[1ff3bff36c6062df]::HandlerInner>::bug::<&alloc[50cd12edb6646b8b]::string::String>
  17:     0x7ffb15c558e0 - <rustc_errors[1ff3bff36c6062df]::Handler>::bug::<&alloc[50cd12edb6646b8b]::string::String>
  18:     0x7ffb15d3275a - rustc_middle[d37df52345c2f8a6]::ty::context::tls::with_context_opt::<rustc_middle[d37df52345c2f8a6]::ty::context::tls::with_opt<rustc_middle[d37df52345c2f8a6]::util::bug::opt_span_bug_fmt<rustc_span[43b640252150d843]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  19:     0x7ffb15d35079 - rustc_middle[d37df52345c2f8a6]::util::bug::opt_span_bug_fmt::<rustc_span[43b640252150d843]::span_encoding::Span>
  20:     0x7ffb15d34ff5 - rustc_middle[d37df52345c2f8a6]::util::bug::bug_fmt
  21:     0x7ffb15ce56b3 - <<rustc_middle[d37df52345c2f8a6]::ty::query::Providers as core[2bb8d0aa17313d7]::default::Default>::default::{closure#11} as core[2bb8d0aa17313d7]::ops::function::FnOnce<(rustc_middle[d37df52345c2f8a6]::ty::context::TyCtxt, rustc_span[43b640252150d843]::def_id::LocalDefId)>>::call_once
  22:     0x7ffb14bcf20a - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<rustc_span[43b640252150d843]::def_id::LocalDefId, rustc_hir[85848aa7b45a13d0]::hir::MaybeOwner<&rustc_hir[85848aa7b45a13d0]::hir::OwnerIndexing>>>
  23:     0x7ffb14cb920f - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::hir_owner_indices, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  24:     0x7ffb14e9f449 - <rustc_query_impl[2b1202858a66a654]::Queries as rustc_middle[d37df52345c2f8a6]::ty::query::QueryEngine>::hir_owner_indices
  25:     0x7ffb138391fd - <rustc_middle[d37df52345c2f8a6]::hir::map::Map>::local_def_id
  26:     0x7ffb13841473 - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>::update_macro_reachable_def
  27:     0x7ffb13840d55 - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>::update_macro_reachable
  28:     0x7ffb13840d55 - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>::update_macro_reachable
  29:     0x7ffb13840d55 - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>::update_macro_reachable
  30:     0x7ffb138423cd - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_item
  31:     0x7ffb13856fec - rustc_hir[85848aa7b45a13d0]::intravisit::walk_mod::<rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>
  32:     0x7ffb13844600 - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_item
  33:     0x7ffb13856fec - rustc_hir[85848aa7b45a13d0]::intravisit::walk_mod::<rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>
  34:     0x7ffb13844600 - <rustc_privacy[c834f81a92e3e655]::EmbargoVisitor as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_item
  35:     0x7ffb13856fec - rustc_hir[85848aa7b45a13d0]::intravisit::walk_mod::<rustc_privacy[c834f81a92e3e655]::EmbargoVisitor>
  36:     0x7ffb1384d03e - rustc_privacy[c834f81a92e3e655]::privacy_access_levels
  37:     0x7ffb14c464f3 - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<(), &rustc_middle[d37df52345c2f8a6]::middle::privacy::AccessLevels>>
  38:     0x7ffb14cc6040 - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::privacy_access_levels, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  39:     0x7ffb14254b56 - rustc_passes[86a2b42321058e0c]::stability::check_unused_or_stable_features
  40:     0x7ffb13579eda - <rustc_session[f2c77fdc990fe9ee]::session::Session>::time::<(), rustc_interface[2bc777e04ad07507]::passes::analysis::{closure#0}>
  41:     0x7ffb13572c49 - rustc_interface[2bc777e04ad07507]::passes::analysis
  42:     0x7ffb14c3d54d - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<(), core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>>
  43:     0x7ffb14ce1970 - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::analysis, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  44:     0x7ffb13437339 - <rustc_interface[2bc777e04ad07507]::passes::QueryContext>::enter::<rustc_driver[609c8905cc717744]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  45:     0x7ffb13405c44 - rustc_interface[2bc777e04ad07507]::interface::create_compiler_and_run::<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>
  46:     0x7ffb13496f22 - <scoped_tls[da5adb31d26bd224]::ScopedKey<rustc_span[43b640252150d843]::SessionGlobals>>::set::<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  47:     0x7ffb1344792f - std[b078befb018cfa9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2bc777e04ad07507]::util::run_in_thread_pool_with_globals<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  48:     0x7ffb13448b49 - <<std[b078befb018cfa9a]::thread::Builder>::spawn_unchecked_<rustc_interface[2bc777e04ad07507]::util::run_in_thread_pool_with_globals<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#1} as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  49:     0x7ffb1651e243 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h06da489939a0fe31
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/alloc/src/boxed.rs:1940:9
  50:     0x7ffb1651e243 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h10f79067252c7632
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/alloc/src/boxed.rs:1940:9
  51:     0x7ffb1651e243 - std::sys::unix::thread::Thread::new::thread_start::ha0b3536ba188d794
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys/unix/thread.rs:108:17
  52:     0x7ffb11b87ea5 - start_thread
  53:     0x7ffb118b0b0d - clone
  54:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e395fd6fa 2022-09-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [hir_owner_indices] HIR owner indices in `num::bignum::Big32x40`
#1 [privacy_access_levels] privacy access levels
#2 [analysis] running analysis passes on this crate
end of query stack
error: internal compiler error: compiler/rustc_middle/src/ty/query.rs:337:1: `tcx.hir_owner_indices(DefId(0:169 ~ core[1f24]::f32::{impl#0}::to_int_unchecked))` unsupported by its crate; perhaps the `hir_owner_indices` query was never assigned a provider function

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/compiler/rustc_errors/src/lib.rs:1458:9
   0:     0x7ffb16510570 - std::backtrace_rs::backtrace::libunwind::trace::hc124e0911c16566c
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7ffb16510570 - std::backtrace_rs::backtrace::trace_unsynchronized::h1002d393d1719d00
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7ffb16510570 - std::sys_common::backtrace::_print_fmt::ha7087e4ba56c5a26
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7ffb16510570 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb33fe85e879df56b
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7ffb1656f8cc - core::fmt::write::hf970a4d7eb447230
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/core/src/fmt/mod.rs:1202:17
   5:     0x7ffb16500c95 - std::io::Write::write_fmt::h53f0aca5464009c6
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/io/mod.rs:1679:15
   6:     0x7ffb165133d1 - std::sys_common::backtrace::_print::ha72626d8989291c2
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7ffb165133d1 - std::sys_common::backtrace::print::h814497532f0c8a4b
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7ffb165133d1 - std::panicking::default_hook::{{closure}}::hed44907d634d668a
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:295:22
   9:     0x7ffb1651308e - std::panicking::default_hook::h1d80056e592acc25
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:314:9
  10:     0x7ffb133f3cf6 - <rustc_driver[609c8905cc717744]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[2bb8d0aa17313d7]::ops::function::FnOnce<(&core[2bb8d0aa17313d7]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7ffb16513c46 - std::panicking::rust_panic_with_hook::hc3c78944e414cef8
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/panicking.rs:702:17
  12:     0x7ffb15c69903 - std[b078befb018cfa9a]::panicking::begin_panic::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>::{closure#0}
  13:     0x7ffb15c67236 - std[b078befb018cfa9a]::sys_common::backtrace::__rust_end_short_backtrace::<std[b078befb018cfa9a]::panicking::begin_panic<rustc_errors[1ff3bff36c6062df]::ExplicitBug>::{closure#0}, !>
  14:     0x7ffb15b52c96 - std[b078befb018cfa9a]::panicking::begin_panic::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>
  15:     0x7ffb15c670b6 - std[b078befb018cfa9a]::panic::panic_any::<rustc_errors[1ff3bff36c6062df]::ExplicitBug>
  16:     0x7ffb15c55e73 - <rustc_errors[1ff3bff36c6062df]::HandlerInner>::bug::<&alloc[50cd12edb6646b8b]::string::String>
  17:     0x7ffb15c558e0 - <rustc_errors[1ff3bff36c6062df]::Handler>::bug::<&alloc[50cd12edb6646b8b]::string::String>
  18:     0x7ffb15d3275a - rustc_middle[d37df52345c2f8a6]::ty::context::tls::with_context_opt::<rustc_middle[d37df52345c2f8a6]::ty::context::tls::with_opt<rustc_middle[d37df52345c2f8a6]::util::bug::opt_span_bug_fmt<rustc_span[43b640252150d843]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  19:     0x7ffb15d35079 - rustc_middle[d37df52345c2f8a6]::util::bug::opt_span_bug_fmt::<rustc_span[43b640252150d843]::span_encoding::Span>
  20:     0x7ffb15d34ff5 - rustc_middle[d37df52345c2f8a6]::util::bug::bug_fmt
  21:     0x7ffb15ce56b3 - <<rustc_middle[d37df52345c2f8a6]::ty::query::Providers as core[2bb8d0aa17313d7]::default::Default>::default::{closure#11} as core[2bb8d0aa17313d7]::ops::function::FnOnce<(rustc_middle[d37df52345c2f8a6]::ty::context::TyCtxt, rustc_span[43b640252150d843]::def_id::LocalDefId)>>::call_once
  22:     0x7ffb14bcf20a - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<rustc_span[43b640252150d843]::def_id::LocalDefId, rustc_hir[85848aa7b45a13d0]::hir::MaybeOwner<&rustc_hir[85848aa7b45a13d0]::hir::OwnerIndexing>>>
  23:     0x7ffb14cb920f - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::hir_owner_indices, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  24:     0x7ffb14e9f449 - <rustc_query_impl[2b1202858a66a654]::Queries as rustc_middle[d37df52345c2f8a6]::ty::query::QueryEngine>::hir_owner_indices
  25:     0x7ffb14240f83 - <rustc_middle[d37df52345c2f8a6]::hir::map::Map>::opt_local_def_id
  26:     0x7ffb1424f6d6 - <rustc_passes[86a2b42321058e0c]::stability::Annotator as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_generic_param
  27:     0x7ffb1425c4ae - rustc_hir[85848aa7b45a13d0]::intravisit::walk_generics::<rustc_passes[86a2b42321058e0c]::stability::Annotator>
  28:     0x7ffb1425d498 - rustc_hir[85848aa7b45a13d0]::intravisit::walk_impl_item::<rustc_passes[86a2b42321058e0c]::stability::Annotator>
  29:     0x7ffb1424ae86 - <rustc_passes[86a2b42321058e0c]::stability::Annotator as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_impl_item
  30:     0x7ffb142836ff - rustc_hir[85848aa7b45a13d0]::intravisit::walk_item::<rustc_passes[86a2b42321058e0c]::stability::Annotator>
  31:     0x7ffb14247679 - <rustc_passes[86a2b42321058e0c]::stability::Annotator as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_item
  32:     0x7ffb14271a4c - rustc_hir[85848aa7b45a13d0]::intravisit::walk_mod::<rustc_passes[86a2b42321058e0c]::stability::Annotator>
  33:     0x7ffb14247679 - <rustc_passes[86a2b42321058e0c]::stability::Annotator as rustc_hir[85848aa7b45a13d0]::intravisit::Visitor>::visit_item
  34:     0x7ffb14271a4c - rustc_hir[85848aa7b45a13d0]::intravisit::walk_mod::<rustc_passes[86a2b42321058e0c]::stability::Annotator>
  35:     0x7ffb142532bd - rustc_passes[86a2b42321058e0c]::stability::stability_index
  36:     0x7ffb14bb7c79 - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::ArenaCache<(), rustc_middle[d37df52345c2f8a6]::middle::stability::Index>>
  37:     0x7ffb14cb1e20 - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::stability_index, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  38:     0x7ffb13600563 - <core[2bb8d0aa17313d7]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[2bc777e04ad07507]::passes::analysis::{closure#0}::{closure#3}> as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once
  39:     0x7ffb13579eef - <rustc_session[f2c77fdc990fe9ee]::session::Session>::time::<(), rustc_interface[2bc777e04ad07507]::passes::analysis::{closure#0}>
  40:     0x7ffb13572c49 - rustc_interface[2bc777e04ad07507]::passes::analysis
  41:     0x7ffb14c3d54d - rustc_query_system[3588919a8bbfb827]::query::plumbing::try_execute_query::<rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt, rustc_query_system[3588919a8bbfb827]::query::caches::DefaultCache<(), core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>>
  42:     0x7ffb14ce1970 - rustc_query_system[3588919a8bbfb827]::query::plumbing::get_query::<rustc_query_impl[2b1202858a66a654]::queries::analysis, rustc_query_impl[2b1202858a66a654]::plumbing::QueryCtxt>
  43:     0x7ffb13437339 - <rustc_interface[2bc777e04ad07507]::passes::QueryContext>::enter::<rustc_driver[609c8905cc717744]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  44:     0x7ffb13405c44 - rustc_interface[2bc777e04ad07507]::interface::create_compiler_and_run::<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>
  45:     0x7ffb13496f22 - <scoped_tls[da5adb31d26bd224]::ScopedKey<rustc_span[43b640252150d843]::SessionGlobals>>::set::<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  46:     0x7ffb1344792f - std[b078befb018cfa9a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2bc777e04ad07507]::util::run_in_thread_pool_with_globals<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>
  47:     0x7ffb13448b49 - <<std[b078befb018cfa9a]::thread::Builder>::spawn_unchecked_<rustc_interface[2bc777e04ad07507]::util::run_in_thread_pool_with_globals<rustc_interface[2bc777e04ad07507]::interface::run_compiler<core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>, rustc_driver[609c8905cc717744]::run_compiler::{closure#1}>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#0}, core[2bb8d0aa17313d7]::result::Result<(), rustc_errors[1ff3bff36c6062df]::ErrorGuaranteed>>::{closure#1} as core[2bb8d0aa17313d7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  48:     0x7ffb1651e243 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h06da489939a0fe31
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/alloc/src/boxed.rs:1940:9
  49:     0x7ffb1651e243 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h10f79067252c7632
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/alloc/src/boxed.rs:1940:9
  50:     0x7ffb1651e243 - std::sys::unix::thread::Thread::new::thread_start::ha0b3536ba188d794
                               at /rustc/e395fd6faf5d1aeb98686d8ad98d8d196f122df3/library/std/src/sys/unix/thread.rs:108:17
  51:     0x7ffb11b87ea5 - start_thread
  52:     0x7ffb118b0b0d - clone
  53:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.65.0-nightly (e395fd6fa 2022-09-15) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=1 -Z unstable-options -C linker=clang -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z save-analysis -C prefer-dynamic -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [hir_owner_indices] HIR owner indices in `f32::<impl at library/core/src/num/f32.rs:350:1: 350:9>::to_int_unchecked`
#1 [stability_index] calculating the stability index for the local crate
#2 [analysis] running analysis passes on this crate
[RUSTC-TIMING] core test:false 4.423
error: could not compile `core`
Build completed unsuccessfully in 0:10:16
