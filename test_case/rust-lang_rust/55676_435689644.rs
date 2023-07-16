
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +nightly 55676.rs 
thread 'main' panicked at 'expected fn type', libcore/option.rs:1008:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (cae6efc37 2018-10-27) running on x86_64-unknown-linux-gnu

zmd@ReflectiveCoherence:~/Code/Misc$ RUST_BACKTRACE=full rustc +nightly 55676.rs 
thread 'main' panicked at 'expected fn type', libcore/option.rs:1008:5
stack backtrace:
   0:     0x7f30ed4f03b3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hff1a9ae0cb8dd1a6
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f30ed4e7b48 - std::sys_common::backtrace::_print::h352f3fc07c43b69e
                               at libstd/sys_common/backtrace.rs:71
   2:     0x7f30ed4ece24 - std::panicking::default_hook::{{closure}}::h294506be82a457d8
                               at libstd/sys_common/backtrace.rs:59
                               at libstd/panicking.rs:211
   3:     0x7f30ed4ecb8d - std::panicking::default_hook::haabbabad3c760f6d
                               at libstd/panicking.rs:227
   4:     0x7f30e9a3d1b1 - rustc::util::common::panic_hook::hca778634c7f8e078
   5:     0x7f30ed4ed566 - std::panicking::rust_panic_with_hook::hd1dd9cfd4c34cc90
                               at libstd/panicking.rs:480
   6:     0x7f30ed4ed0b1 - std::panicking::continue_panic_fmt::ha2cb49e747ff6ee6
                               at libstd/panicking.rs:390
   7:     0x7f30ed4ecf95 - rust_begin_unwind
                               at libstd/panicking.rs:325
   8:     0x7f30ed5439fc - core::panicking::panic_fmt::h81c3c187c4305910
                               at libcore/panicking.rs:77
   9:     0x7f30ed543b12 - core::option::expect_failed::h92549066814fc353
                               at libcore/option.rs:1008
  10:     0x7f30ebbc51fb - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_tuple_struct::hc5ee5644a55aec78
  11:     0x7f30ebbc20ac - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_walk::h3ff9e35a1c1a9737
  12:     0x7f30ebbc4b9f - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_tuple_struct::hc5ee5644a55aec78
  13:     0x7f30ebbc20ac - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_walk::h3ff9e35a1c1a9737
  14:     0x7f30ebbf37a0 - rustc_typeck::check::FnCtxt::check_expr_kind::hdfed46c376966d5d
  15:     0x7f30ebbef980 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h79c852f76020d2ce
  16:     0x7f30ebbfe624 - rustc_typeck::check::FnCtxt::check_block_with_expected::hd30258448a8f4c5b
  17:     0x7f30ebbefe95 - rustc_typeck::check::FnCtxt::check_expr_kind::hdfed46c376966d5d
  18:     0x7f30ebbef980 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h79c852f76020d2ce
  19:     0x7f30ebbeeafd - rustc_typeck::check::FnCtxt::check_return_expr::ha8dd119c1bb08d85
  20:     0x7f30ebbdf954 - rustc_typeck::check::check_fn::h2386e15a28825b1f
  21:     0x7f30ebc99a5f - rustc::ty::context::tls::with_related_context::h54d83b1dcfcbdbaf
  22:     0x7f30ebd67e1a - rustc::infer::InferCtxtBuilder::enter::h46f975ab9992befa
  23:     0x7f30ebbde443 - rustc_typeck::check::typeck_tables_of::h8d77c68e628eea1c
  24:     0x7f30e9715a85 - rustc::ty::query::__query_compute::typeck_tables_of::had2bccc1feaa18f9
  25:     0x7f30e9641e0c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute::hd1765820547ad976
  26:     0x7f30e943cd1c - rustc::dep_graph::graph::DepGraph::with_task_impl::h0ac9837e76cdc2d0
  27:     0x7f30e95d8835 - rustc::ty::context::tls::with_related_context::h20531e1db31349aa
  28:     0x7f30e976852d - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::hec61b924bf3cfd24
  29:     0x7f30e989a3e4 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query::he7d19cd044a7275e
  30:     0x7f30e976ce5c - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query::h0de4f1391bef0e27
  31:     0x7f30ebc2cb01 - rustc::session::Session::track_errors::h621b18a6d05e9419
  32:     0x7f30ebbddf2b - rustc_typeck::check::typeck_item_bodies::h350c2aebdabb7019
  33:     0x7f30e9641de8 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute::h59eed17483d4272c
  34:     0x7f30e9472605 - rustc::dep_graph::graph::DepGraph::with_task_impl::hb33c6cf2b3062f10
  35:     0x7f30e96392c5 - rustc::ty::context::tls::with_related_context::hfc7862cd15657b16
  36:     0x7f30e975b1c6 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::habb5cf9e656459e9
  37:     0x7f30e98a1a38 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query::hf5408248f8504e0e
  38:     0x7f30ebc4bec7 - rustc::util::common::time::h7b2e62ef0ad279f4
  39:     0x7f30ebc33e8b - rustc_typeck::check_crate::hedc4b5073ff6f472
  40:     0x7f30ed81a73a - rustc::ty::context::tls::enter_context::h28e96223c62c8c56
  41:     0x7f30ed8be991 - <std::thread::local::LocalKey<T>>::with::h1ffab1d5e57611a3
  42:     0x7f30ed912c10 - rustc::ty::context::TyCtxt::create_and_enter::h98298e4f4254e3fb
  43:     0x7f30ed863d04 - rustc_driver::driver::compile_input::h827ff002b759d122
  44:     0x7f30ed930bd4 - rustc_driver::run_compiler_with_pool::h5bb22abd8efd3251
  45:     0x7f30ed861d1b - rustc_driver::driver::spawn_thread_pool::h72bbcce0d9a2e7ed
  46:     0x7f30ed92fc07 - rustc_driver::run_compiler::h9cbc223af46659be
  47:     0x7f30ed84c4a4 - <scoped_tls::ScopedKey<T>>::set::h10d326a323b77f70
  48:     0x7f30ed83ea32 - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h631bec0d0df9c696
  49:     0x7f30ed50a729 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  50:     0x7f30ed92dd35 - rustc_driver::run::h0ae909aca51139d8
  51:     0x7f30ed93b86b - rustc_driver::main::h69a05b4b6529a359
  52:     0x556305cf1962 - std::rt::lang_start::{{closure}}::h77442e4e4ddad9f4
  53:     0x7f30ed4ecf32 - std::panicking::try::do_call::he8dfa1f97317484c
                               at libstd/rt.rs:59
                               at libstd/panicking.rs:310
  54:     0x7f30ed50a729 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:102
  55:     0x7f30ed4ed903 - std::rt::lang_start_internal::hb5cea8bed57f3dc2
                               at libstd/panicking.rs:289
                               at libstd/panic.rs:392
                               at libstd/rt.rs:58
  56:     0x556305cf1954 - main
  57:     0x7f30ed0d982f - __libc_start_main
  58:     0x556305cf1808 - <unknown>
query stack during panic:
#0 [typeck_tables_of] processing `main`
#1 [typeck_item_bodies] type-checking all item bodies
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.31.0-nightly (cae6efc37 2018-10-27) running on x86_64-unknown-linux-gnu
