
env RUST_BACKTRACE=full cargo run
   Compiling foo v0.1.0 (/tmp/tmp.pzK1QddPz6/foo)
error: internal compiler error: src/librustc_typeck/check/mod.rs:825: can't type-check body of DefId(0/1:10 ~ foo[bcdd]::Foo[0]::Bar[0]::{{constructor}}[0])
 --> src/main.rs:2:5
  |
2 |     Bar(usize),
  |     ^^^^^^^^^^

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:570:9
stack backtrace:
   0:     0x7f7c2ec65dc3 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h2f2e88c6ec1c5c72
                               at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
   1:     0x7f7c2ec5dc4b - std::sys_common::backtrace::_print::h944bf1840a0a0e67
                               at src/libstd/sys_common/backtrace.rs:71
   2:     0x7f7c2ec62026 - std::panicking::default_hook::{{closure}}::hf00b55dd98cf4c0b
                               at src/libstd/sys_common/backtrace.rs:59
                               at src/libstd/panicking.rs:197
   3:     0x7f7c2ec61db9 - std::panicking::default_hook::h91c25d81f72ffd4b
                               at src/libstd/panicking.rs:211
   4:     0x7f7c2c9e5d20 - rustc::util::common::panic_hook::h12a29cd81311b496
   5:     0x7f7c2ec62818 - std::panicking::rust_panic_with_hook::h38a6248b079915f7
                               at src/libstd/panicking.rs:478
   6:     0x7f7c2996675c - std::panicking::begin_panic::h2b120496cb278894
   7:     0x7f7c2994726c - rustc_errors::Handler::span_bug::h1dfe556f382bf234
   8:     0x7f7c299bf991 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hdeaab18a2ccdd1ad
   9:     0x7f7c299bd519 - rustc::ty::context::tls::with_opt::{{closure}}::h8d0a293f12fda0ca
  10:     0x7f7c299bd47f - rustc::ty::context::tls::with_context_opt::h3b769282f399b79c
  11:     0x7f7c299bd4c6 - rustc::ty::context::tls::with_opt::h7a4737f4d9acfaa2
  12:     0x7f7c299bf887 - rustc::util::bug::opt_span_bug_fmt::h8f055e82b5b0735e
  13:     0x7f7c299bf839 - rustc::util::bug::span_bug_fmt::hf655060d769a6592
  14:     0x7f7c29a52163 - rustc_typeck::check::typeck_tables_of::{{closure}}::he8925ed2486d8f56
  15:     0x7f7c29a520c0 - rustc_typeck::check::typeck_tables_of::he3395a732aef7759
  16:     0x7f7c299baebf - rustc::ty::query::__query_compute::typeck_tables_of::haa965cd0ae21dd19
  17:     0x7f7c29a7d7ac - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::h2f7328bafbd21c5e
  18:     0x7f7c29b447a8 - rustc::dep_graph::graph::DepGraph::with_task_impl::hbc64dfda2477aeea
  19:     0x7f7c29ab0f60 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h502ba2f7c6ccb035
  20:     0x7f7c29a4e9e4 - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::add_type_neq_err_label::h83659d6437c45fc1
  21:     0x7f7c29a4dc25 - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_overloaded_binop::h8321fcbf00750311
  22:     0x7f7c29a4cbfe - rustc_typeck::check::op::<impl rustc_typeck::check::FnCtxt>::check_binop::h61cac264c93b2652
  23:     0x7f7c29a659c1 - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  24:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  25:     0x7f7c29a6605f - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  26:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  27:     0x7f7c29a63d9b - rustc_typeck::check::FnCtxt::check_expr_meets_expectation_or_error::h7821bd5d4bbf06e6
  28:     0x7f7c29a66ab8 - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  29:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  30:     0x7f7c29a70604 - rustc_typeck::check::FnCtxt::check_block_with_expected::h2db4e51ec0d448a8
  31:     0x7f7c29a65e88 - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  32:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  33:     0x7f7c29a2d3d0 - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt>::check_match::hff8f57ec786d428a
  34:     0x7f7c29a66189 - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  35:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  36:     0x7f7c29a70604 - rustc_typeck::check::FnCtxt::check_block_with_expected::h2db4e51ec0d448a8
  37:     0x7f7c29a65e88 - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  38:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  39:     0x7f7c29a70123 - rustc_typeck::check::FnCtxt::check_stmt::hb59198c5ed0e7edc
  40:     0x7f7c29a705ca - rustc_typeck::check::FnCtxt::check_block_with_expected::h2db4e51ec0d448a8
  41:     0x7f7c29a65e88 - rustc_typeck::check::FnCtxt::check_expr_kind::h382693a1fca7cc44
  42:     0x7f7c29a65324 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::hd6c077a4b60cce19
  43:     0x7f7c29a643f9 - rustc_typeck::check::FnCtxt::check_return_expr::ha3439f1ad9d4f31f
  44:     0x7f7c29a53536 - rustc_typeck::check::check_fn::h6e1a464ddbe5dcfb
  45:     0x7f7c29afa838 - rustc::ty::context::GlobalCtxt::enter_local::hc2d03235e4a0576f
  46:     0x7f7c29a51fac - rustc_typeck::check::typeck_tables_of::he3395a732aef7759
  47:     0x7f7c299baebf - rustc::ty::query::__query_compute::typeck_tables_of::haa965cd0ae21dd19
  48:     0x7f7c29a7d7ac - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::h2f7328bafbd21c5e
  49:     0x7f7c29b447a8 - rustc::dep_graph::graph::DepGraph::with_task_impl::hbc64dfda2477aeea
  50:     0x7f7c29ab0f60 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h502ba2f7c6ccb035
  51:     0x7f7c29a7b1dc - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::hb36a144a1c2a7bdb
  52:     0x7f7c29a51a17 - rustc_typeck::check::typeck_item_bodies::h380acc11ea427f32
  53:     0x7f7c299bb369 - rustc::ty::query::__query_compute::typeck_item_bodies::he1753b4dbe75b961
  54:     0x7f7c29a7d888 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_item_bodies>::compute::hc1231572a88486d7
  55:     0x7f7c29b3cc7c - rustc::dep_graph::graph::DepGraph::with_task_impl::h57253e9998588a7b
  56:     0x7f7c29aabb4b - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h4b243e4a6042aa09
  57:     0x7f7c29b2e9d8 - rustc::util::common::time::h156b034c4f46616d
  58:     0x7f7c29b7241e - rustc_typeck::check_crate::hf56ec03e7ab04612
  59:     0x7f7c2e6520e3 - rustc_interface::passes::analysis::hfb36b85add05e7ac
  60:     0x7f7c2ef2ab95 - rustc::ty::query::__query_compute::analysis::hbbd7cf219c941643
  61:     0x7f7c2ef71ee8 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::hc2d14f51c435b551
  62:     0x7f7c2ef4f972 - rustc::dep_graph::graph::DepGraph::with_task_impl::h7048f32b9e37fbb1
  63:     0x7f7c2ef72db1 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h9a66c6c41336316a
  64:     0x7f7c2ef2aea5 - rustc::ty::context::tls::enter_global::hab73e8d01597b9eb
  65:     0x7f7c2ef366b6 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h99c4ee2ef186de4c
  66:     0x7f7c2e6a67c4 - rustc_interface::passes::create_global_ctxt::{{closure}}::hf73126db9faf36f1
  67:     0x7f7c2ef3650e - rustc_interface::passes::BoxedGlobalCtxt::enter::hd110766f966ad8c2
  68:     0x7f7c2ef46093 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h2b040aec050db1e5
  69:     0x7f7c2ef1bb83 - std::thread::local::LocalKey<T>::with::h99e20e81d5aa58b9
  70:     0x7f7c2ef7e4c4 - scoped_tls::ScopedKey<T>::set::hce855187a6e6eca8
  71:     0x7f7c2efb2b2f - syntax::with_globals::hae2cf851d88da20b
  72:     0x7f7c2ef1d31c - std::sys_common::backtrace::__rust_begin_short_backtrace::h35adf960dc57fe78
  73:     0x7f7c2ec735e9 - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:85
  74:     0x7f7c2ef37728 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h1c87f9921616cd42
  75:     0x7f7c2ec446fe - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h1d75a25d0df78a83
                               at /rustc/aa99abeb262307d5e9aa11a792312fd620b7f89a/src/liballoc/boxed.rs:704
  76:     0x7f7c2ec7225f - std::sys::unix::thread::Thread::new::thread_start::h447e6585d0cd6312
                               at /rustc/aa99abeb262307d5e9aa11a792312fd620b7f89a/src/liballoc/boxed.rs:704
                               at src/libstd/sys_common/thread.rs:13
                               at src/libstd/sys/unix/thread.rs:79
  77:     0x7f7c2ebb3181 - start_thread
  78:     0x7f7c2eac9b1e - __clone
  79:                0x0 - <unknown>
query stack during panic:
#0 [typeck_tables_of] processing `Foo::Bar`
#1 [typeck_tables_of] processing `main`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.35.0-nightly (aa99abeb2 2019-04-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin
