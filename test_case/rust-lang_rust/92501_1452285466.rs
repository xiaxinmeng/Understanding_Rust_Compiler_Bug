
running  'cargo' 'xtask' 'install' '--server'
    Finished dev [unoptimized] target(s) in 0.08s
     Running `target/debug/xtask install --server`
$ cargo install --path crates/rust-analyzer --locked --force --features force-always-assert
  Installing rust-analyzer v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/rust-analyzer)
    Updating crates.io index
   Compiling hir-def v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/hir-def)
   Compiling rust-analyzer v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/rust-analyzer)
   Compiling hir-ty v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/hir-ty)
   Compiling hir v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/hir)
   Compiling ide-db v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/ide-db)
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs:23:62
stack backtrace:
   0:     0x7f16995656fa - std::backtrace_rs::backtrace::libunwind::trace::h79937bc171ada62c
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f16995656fa - std::backtrace_rs::backtrace::trace_unsynchronized::h2292bca8571cb919
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f16995656fa - std::sys_common::backtrace::_print_fmt::h9c461f248e4ae90d
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f16995656fa - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he9fe6bf1a39182e1
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f16995c825e - core::fmt::write::h032658c119c720d7
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/fmt/mod.rs:1208:17
   5:     0x7f1699555a85 - std::io::Write::write_fmt::h299fc90dfae41c0d
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/io/mod.rs:1682:15
   6:     0x7f16995654c5 - std::sys_common::backtrace::_print::heb70d25df9937e3f
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f16995654c5 - std::sys_common::backtrace::print::had745c0a76b8b521
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f169956820f - std::panicking::default_hook::{{closure}}::h1ea782cdfa2fd097
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:267:22
   9:     0x7f1699567f4b - std::panicking::default_hook::h1cc3af63455a163c
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:286:9
  10:     0x7f169c860ab1 - <rustc_driver[5c3b90d1fb3964ba]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[e6a29f2585b3d454]::ops::function::FnOnce<(&core[e6a29f2585b3d454]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f1699568a4d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h6e4950ba7c0fd82a
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/alloc/src/boxed.rs:2032:9
  12:     0x7f1699568a4d - std::panicking::rust_panic_with_hook::h5cafdc4b3bfd5528
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:692:13
  13:     0x7f1699568782 - std::panicking::begin_panic_handler::{{closure}}::hf31c60f40775892c
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:577:13
  14:     0x7f1699565bac - std::sys_common::backtrace::__rust_end_short_backtrace::h28a5c7be595826cd
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys_common/backtrace.rs:137:18
  15:     0x7f16995684d2 - rust_begin_unwind
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:575:5
  16:     0x7f16995c4c43 - core::panicking::panic_fmt::h8fa27a0b37dd98b7
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/panicking.rs:64:14
  17:     0x7f16995c4d1d - core::panicking::panic::h545818946343732b
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/panicking.rs:111:5
  18:     0x7f169a8545fd - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::dep_node::DepNode<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind> as rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepNodeExt>::extract_def_id
  19:     0x7f169c0b37ee - rustc_query_impl[e214cefb6de2a99d]::plumbing::force_from_dep_node::<rustc_query_impl[e214cefb6de2a99d]::queries::type_of>
  20:     0x7f169a8bb039 - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  21:     0x7f169a8bafdc - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  22:     0x7f169a8bafdc - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  23:     0x7f169a8bafdc - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  24:     0x7f169a8bafdc - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  25:     0x7f169b04f7d6 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::get_query::<rustc_query_impl[e214cefb6de2a99d]::queries::evaluate_obligation, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  26:     0x7f169aaab337 - <rustc_trait_selection[945b70100f8dd3ec]::traits::fulfill::FulfillProcessor as rustc_data_structures[1026114362f98086]::obligation_forest::ObligationProcessor>::process_obligation
  27:     0x7f169aaa77a2 - <rustc_data_structures[1026114362f98086]::obligation_forest::ObligationForest<rustc_trait_selection[945b70100f8dd3ec]::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection[945b70100f8dd3ec]::traits::fulfill::FulfillProcessor>
  28:     0x7f169b347afb - <rustc_trait_selection[945b70100f8dd3ec]::traits::engine::ObligationCtxt>::select_all_or_error
  29:     0x7f169b8820dd - rustc_hir_analysis[44326659f1d3f01d]::check::wfcheck::check_well_formed
  30:     0x7f169aa6028c - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[83f907612b22699d]::ty::context::TyCtxt, rustc_hir[dd085327af2d6b95]::hir_id::OwnerId, ()>
  31:     0x7f169bfe50e6 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::try_execute_query::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt, rustc_query_system[7dbbccfee5a2d054]::query::caches::VecCache<rustc_hir[dd085327af2d6b95]::hir_id::OwnerId, ()>>
  32:     0x7f169bfe487c - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::force_query::<rustc_query_impl[e214cefb6de2a99d]::queries::check_well_formed, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  33:     0x7f169bfe4730 - rustc_query_impl[e214cefb6de2a99d]::plumbing::force_from_dep_node::<rustc_query_impl[e214cefb6de2a99d]::queries::check_well_formed>
  34:     0x7f169a8bb039 - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  35:     0x7f169aa5f954 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::ensure_must_run::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt, rustc_span[41a321a6411ba4fa]::def_id::LocalDefId, bool>
  36:     0x7f169bc02c8e - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::get_query::<rustc_query_impl[e214cefb6de2a99d]::queries::check_mod_type_wf, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  37:     0x7f169ad5b5e8 - rustc_data_structures[1026114362f98086]::sync::par_for_each_in::<&[rustc_hir[dd085327af2d6b95]::hir_id::OwnerId], <rustc_middle[83f907612b22699d]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[44326659f1d3f01d]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
  38:     0x7f169ad5b3d3 - <rustc_session[b89b9f24749004e7]::session::Session>::track_errors::<rustc_hir_analysis[44326659f1d3f01d]::check_crate::{closure#5}, ()>
  39:     0x7f169ad5aced - rustc_hir_analysis[44326659f1d3f01d]::check_crate
  40:     0x7f169ad5a98b - rustc_interface[65dcc5dffb099e04]::passes::analysis
  41:     0x7f169c01291f - <rustc_query_system[7dbbccfee5a2d054]::dep_graph::graph::DepGraph<rustc_middle[83f907612b22699d]::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle[83f907612b22699d]::ty::context::TyCtxt, (), core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  42:     0x7f169c011a17 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::try_execute_query::<rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt, rustc_query_system[7dbbccfee5a2d054]::query::caches::DefaultCache<(), core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>>
  43:     0x7f169c011470 - rustc_query_system[7dbbccfee5a2d054]::query::plumbing::get_query::<rustc_query_impl[e214cefb6de2a99d]::queries::analysis, rustc_query_impl[e214cefb6de2a99d]::plumbing::QueryCtxt>
  44:     0x7f169ba241b3 - <rustc_interface[65dcc5dffb099e04]::passes::QueryContext>::enter::<rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}::{closure#2}::{closure#2}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  45:     0x7f169ba20733 - <rustc_interface[65dcc5dffb099e04]::interface::Compiler>::enter::<rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}::{closure#2}, core[e6a29f2585b3d454]::result::Result<core[e6a29f2585b3d454]::option::Option<rustc_interface[65dcc5dffb099e04]::queries::Linker>, rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  46:     0x7f169ba1b788 - rustc_span[41a321a6411ba4fa]::with_source_map::<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  47:     0x7f169ba1b275 - <scoped_tls[393dd8f8fd825c8d]::ScopedKey<rustc_span[41a321a6411ba4fa]::SessionGlobals>>::set::<rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  48:     0x7f169ba1a862 - std[359ab902947f5f0b]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[65dcc5dffb099e04]::util::run_in_thread_pool_with_globals<rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>
  49:     0x7f169c113b7a - <<std[359ab902947f5f0b]::thread::Builder>::spawn_unchecked_<rustc_interface[65dcc5dffb099e04]::util::run_in_thread_pool_with_globals<rustc_interface[65dcc5dffb099e04]::interface::run_compiler<core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>, rustc_driver[5c3b90d1fb3964ba]::run_compiler::{closure#1}>::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[e6a29f2585b3d454]::result::Result<(), rustc_errors[d24ea2395205e4f0]::ErrorGuaranteed>>::{closure#1} as core[e6a29f2585b3d454]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  50:     0x7f1699572803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb77d8d72ebcf79c4
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/alloc/src/boxed.rs:2000:9
  51:     0x7f1699572803 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc08c3353e1568487
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/alloc/src/boxed.rs:2000:9
  52:     0x7f1699572803 - std::sys::unix::thread::Thread::new::thread_start::h7168e596cd5e5ce6
                               at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/sys/unix/thread.rs:108:17
  53:     0x7f1699290402 - start_thread
                               at ./nptl/./nptl/pthread_create.c:442:8
  54:     0x7f169931f590 - __GI___clone3
                               at ./misc/../sysdeps/unix/sysv/linux/x86_64/clone3.S:81
  55:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.1 (d5a82bbd2 2023-02-07) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [evaluate_obligation] evaluating trait selection obligation `salsa::derived::slot::Memo<hir_ty::db::ConstEvalQuery>: core::marker::Send`
#1 [check_well_formed] checking that `<impl at crates/ide-db/src/lib.rs:155:1: 155:46>` is well-formed
#2 [analysis] running analysis passes on this crate
end of query stack
error: could not compile `ide-db`
warning: build failed, waiting for other jobs to finish...
error: failed to compile `rust-analyzer v0.0.0 (/home/j3tracey/Code/archive/rust-analyzer/crates/rust-analyzer)`, intermediate artifacts can be found at `/home/j3tracey/Code/archive/rust-analyzer/target`
Error: install server

Caused by:
    command exited with non-zero code `cargo install --path crates/rust-analyzer --locked --force --features force-always-assert`: 101

