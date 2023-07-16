plain
[RUSTC-TIMING] adler test:false 0.286
[RUSTC-TIMING] build_script_build test:false 0.400
[RUSTC-TIMING] ansi_term test:false 0.915
   Compiling libloading v0.7.1
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: Unknown(T)', compiler/rustc_lint/src/builtin.rs:2508:65
stack backtrace:
   0:     0x7fc904015730 - std::backtrace_rs::backtrace::libunwind::trace::h7bb04a2d4c508234
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7fc904015730 - std::backtrace_rs::backtrace::trace_unsynchronized::haac81d406e3db62e
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fc904015730 - std::sys_common::backtrace::_print_fmt::h19fab311aa796aa8
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fc904015730 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hfc77a3e5d171a068
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fc90407171e - core::fmt::write::h861db7d15f3ad9bc
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fc9040059a5 - std::io::Write::write_fmt::h64daa8607fa2ec16
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/io/mod.rs:1682:15
   6:     0x7fc9040154f5 - std::sys_common::backtrace::_print::h4f76f8cfa531df37
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fc9040154f5 - std::sys_common::backtrace::print::hdf68db07ad361a52
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fc9040182ff - std::panicking::default_hook::{{closure}}::h5c0488dfa8497bfc
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/panicking.rs:267:22
   9:     0x7fc90401803a - std::panicking::default_hook::hbfd4efe4a986a2f2
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/panicking.rs:286:9
  10:     0x7fc901304547 - <rustc_driver[de3e4f482d182886]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[6134bbfab4c35619]::ops::function::FnOnce<(&core[6134bbfab4c35619]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7fc904018b29 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h0b3dafa0acf454f0
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/alloc/src/boxed.rs:2001:9
  12:     0x7fc904018b29 - std::panicking::rust_panic_with_hook::h695f83723219c803
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/panicking.rs:692:13
  13:     0x7fc9040188a7 - std::panicking::begin_panic_handler::{{closure}}::h254921428e7cc934
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/panicking.rs:579:13
  14:     0x7fc904015bdc - std::sys_common::backtrace::__rust_end_short_backtrace::h9a04011f11610f56
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7fc9040185c2 - rust_begin_unwind
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/panicking.rs:575:5
  16:     0x7fc90406e103 - core::panicking::panic_fmt::h422d949dba0b9ba5
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/core/src/panicking.rs:65:14
  17:     0x7fc90406e673 - core::result::unwrap_failed::h6d76205b0c3abc34
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/core/src/result.rs:1791:5
  18:     0x7fc90338542c - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::variant_find_init_error
  19:     0x7fc903385cda - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::ty_find_init_error
  20:     0x7fc9033844e0 - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::variant_find_init_error
  21:     0x7fc903385cda - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::ty_find_init_error
  22:     0x7fc903386287 - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::ty_find_init_error
  23:     0x7fc9033844e0 - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::variant_find_init_error
  24:     0x7fc903385cda - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr::ty_find_init_error
  25:     0x7fc90338433e - <rustc_lint[b7bad8aa8ec2637e]::builtin::InvalidValue as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr
  26:     0x7fc9033cec98 - <rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass as rustc_lint[b7bad8aa8ec2637e]::passes::LateLintPass>::check_expr
  27:     0x7fc903389e0f - rustc_hir[6f692f52a20ed61e]::intravisit::walk_block::<rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass>>
  28:     0x7fc903389e1a - rustc_hir[6f692f52a20ed61e]::intravisit::walk_block::<rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass>>
  29:     0x7fc903395346 - rustc_hir[6f692f52a20ed61e]::intravisit::walk_body::<rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass>>
  30:     0x7fc90340e833 - <rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass> as rustc_hir[6f692f52a20ed61e]::intravisit::Visitor>::visit_nested_body
  31:     0x7fc90340f1bd - <rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass> as rustc_hir[6f692f52a20ed61e]::intravisit::Visitor>::visit_fn
  32:     0x7fc90338ba26 - rustc_hir[6f692f52a20ed61e]::intravisit::walk_impl_item::<rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass>>
  33:     0x7fc90340de7e - <rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass> as rustc_hir[6f692f52a20ed61e]::intravisit::Visitor>::visit_nested_impl_item
  34:     0x7fc90339847c - rustc_hir[6f692f52a20ed61e]::intravisit::walk_item::<rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass>>
  35:     0x7fc90340d9d8 - <rustc_lint[b7bad8aa8ec2637e]::late::LateContextAndPass<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass> as rustc_hir[6f692f52a20ed61e]::intravisit::Visitor>::visit_nested_item
  36:     0x7fc90341044d - rustc_lint[b7bad8aa8ec2637e]::late::late_lint_mod::<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedModuleLateLintPass>
  37:     0x7fc9033c757d - rustc_lint[b7bad8aa8ec2637e]::lint_mod
  38:     0x7fc902830eaa - rustc_query_system[67b756bdf57a3a5d]::query::plumbing::try_execute_query::<rustc_query_impl[411e2bd38bdc2ef7]::plumbing::QueryCtxt, rustc_query_system[67b756bdf57a3a5d]::query::caches::DefaultCache<rustc_span[7a8be1dd5f2bcb6d]::def_id::LocalDefId, ()>>
  39:     0x7fc902925c4b - rustc_query_system[67b756bdf57a3a5d]::query::plumbing::get_query::<rustc_query_impl[411e2bd38bdc2ef7]::queries::lint_mod, rustc_query_impl[411e2bd38bdc2ef7]::plumbing::QueryCtxt>
  40:     0x7fc90139521b - rustc_data_structures[e84429496420260]::sync::par_for_each_in::<&[rustc_hir[6f692f52a20ed61e]::hir_id::OwnerId], <rustc_middle[1899933e37dd6e6a]::hir::map::Map>::par_for_each_module<rustc_lint[b7bad8aa8ec2637e]::late::check_crate<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedLateLintPass, rustc_interface[d127916a3c4c48b7]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7fc9013b020b - <rustc_session[ee7a5ec49ac2c488]::session::Session>::time::<(), rustc_lint[b7bad8aa8ec2637e]::late::check_crate<rustc_lint[b7bad8aa8ec2637e]::BuiltinCombinedLateLintPass, rustc_interface[d127916a3c4c48b7]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  42:     0x7fc9013b03cd - <rustc_session[ee7a5ec49ac2c488]::session::Session>::time::<(), rustc_interface[d127916a3c4c48b7]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  43:     0x7fc9013e022e - <core[6134bbfab4c35619]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[d127916a3c4c48b7]::passes::analysis::{closure#5}::{closure#1}> as core[6134bbfab4c35619]::ops::function::FnOnce<()>>::call_once
  44:     0x7fc9013b24f5 - <rustc_session[ee7a5ec49ac2c488]::session::Session>::time::<(), rustc_interface[d127916a3c4c48b7]::passes::analysis::{closure#5}>
  45:     0x7fc9013d443c - rustc_interface[d127916a3c4c48b7]::passes::analysis
  46:     0x7fc9028a9737 - rustc_query_system[67b756bdf57a3a5d]::query::plumbing::try_execute_query::<rustc_query_impl[411e2bd38bdc2ef7]::plumbing::QueryCtxt, rustc_query_system[67b756bdf57a3a5d]::query::caches::DefaultCache<(), core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>>
  47:     0x7fc902925780 - rustc_query_system[67b756bdf57a3a5d]::query::plumbing::get_query::<rustc_query_impl[411e2bd38bdc2ef7]::queries::analysis, rustc_query_impl[411e2bd38bdc2ef7]::plumbing::QueryCtxt>
  48:     0x7fc9012983b9 - <rustc_interface[d127916a3c4c48b7]::passes::QueryContext>::enter::<rustc_driver[de3e4f482d182886]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>
  49:     0x7fc9012e1e29 - <rustc_interface[d127916a3c4c48b7]::interface::Compiler>::enter::<rustc_driver[de3e4f482d182886]::run_compiler::{closure#1}::{closure#2}, core[6134bbfab4c35619]::result::Result<core[6134bbfab4c35619]::option::Option<rustc_interface[d127916a3c4c48b7]::queries::Linker>, rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>
  50:     0x7fc90127a892 - rustc_span[7a8be1dd5f2bcb6d]::with_source_map::<core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>, rustc_interface[d127916a3c4c48b7]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>, rustc_driver[de3e4f482d182886]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  51:     0x7fc9012d2888 - <scoped_tls[10d8a9c768e14d1c]::ScopedKey<rustc_span[7a8be1dd5f2bcb6d]::SessionGlobals>>::set::<rustc_interface[d127916a3c4c48b7]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>, rustc_driver[de3e4f482d182886]::run_compiler::{closure#1}>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>
  52:     0x7fc90129ab40 - std[d4f85d34b8fafce1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d127916a3c4c48b7]::util::run_in_thread_pool_with_globals<rustc_interface[d127916a3c4c48b7]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>, rustc_driver[de3e4f482d182886]::run_compiler::{closure#1}>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>
  53:     0x7fc901280da4 - <<std[d4f85d34b8fafce1]::thread::Builder>::spawn_unchecked_<rustc_interface[d127916a3c4c48b7]::util::run_in_thread_pool_with_globals<rustc_interface[d127916a3c4c48b7]::interface::run_compiler<core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>, rustc_driver[de3e4f482d182886]::run_compiler::{closure#1}>::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[6134bbfab4c35619]::result::Result<(), rustc_errors[803dbed4b573a573]::ErrorGuaranteed>>::{closure#1} as core[6134bbfab4c35619]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  54:     0x7fc9040225d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hbcb20d7af978cab2
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/alloc/src/boxed.rs:1987:9
  55:     0x7fc9040225d3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h2d99f5bc14ac163f
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/alloc/src/boxed.rs:1987:9
  56:     0x7fc9040225d3 - std::sys::unix::thread::Thread::new::thread_start::h14666987fab504aa
                               at /rustc/a5ad1e28c77b80ed8339ab36c1438ffc121baafb/library/std/src/sys/unix/thread.rs:108:17
  57:     0x7fc8ffbcbea5 - start_thread
  58:     0x7fc8ff8f4b0d - clone
  59:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (a5ad1e28c 2022-11-03) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C link-args=-fuse-ld=lld -C split-debuginfo=off -Z unstable-options -C prefer-dynamic -C link-args=-Wl,--icf=all -Z dylib-lto -C lto=thin -C embed-bitcode=yes -Z binary-dep-depinfo -Z tls-model=initial-exec -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [lint_mod] linting module `flavors::list`
#1 [analysis] running analysis passes on this crate
[RUSTC-TIMING] crossbeam_channel test:false 0.661
error: could not compile `crossbeam-channel`
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] memchr test:false 2.437
