
thread '<unnamed>' panicked at 'Forcing query with already existing DepNode.
- query-key: Canonical { variables: [], value: ParamEnvAnd { param_env: ParamEnv { caller_bounds: [Binder(TraitPredicate(<T as my_project::server::Node>)), Binder(TraitPredicate(<T as std::clone::Clone>)), Binder(TraitPredicate(<T as std::marker::Sized>))], reveal: UserFacing }, value: Binder(TraitPredicate(<tower_grpc::Encode<<T as my_project::server::Node>::GetBlocksStream> as std::marker::Sized>)) } }
- dep-node: EvaluateObligation(90e109f1c3230ad0-34a2ace33b8d0e55)', librustc/ty/query/plumbing.rs:531:9
stack backtrace:
   0:     0x7f8b6a0e97ce - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::h15749a98198ba25f
                               at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1:     0x7f8b6a0c2966 - std::sys_common::backtrace::print::h0ba65c811294dbf0
                               at libstd/sys_common/backtrace.rs:71
                               at libstd/sys_common/backtrace.rs:59
   2:     0x7f8b6a0c05bd - std::panicking::default_hook::{{closure}}::h42bb31757f0fee64
                               at libstd/panicking.rs:211
   3:     0x7f8b6a0c0330 - std::panicking::default_hook::hebc8848d7aa7dbd9
                               at libstd/panicking.rs:227
   4:     0x7f8b6bfba6a1 - rustc::util::common::panic_hook::h9a39743bb116fb66
   5:     0x7f8b6a0c0cc3 - std::panicking::rust_panic_with_hook::h86e619a638415243
                               at libstd/panicking.rs:481
   6:     0x7f8b6a0c0829 - std::panicking::continue_panic_fmt::h2d2d2bfb065a70b2
                               at libstd/panicking.rs:391
   7:     0x7f8b6a0c078d - std::panicking::begin_panic_fmt::ha5097e2056ef3d5a
                               at libstd/panicking.rs:346
   8:     0x7f8b6bcd2677 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::h018b25c334abb104
   9:     0x7f8b6bd94ebd - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query::h592ae51f91676520
  10:     0x7f8b6bf93d58 - rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::evaluate_obligation::hc018e0004aa12d4c
  11:     0x7f8b6bf93ddc - rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::evaluate_obligation_no_overflow::h4f5a161529dc323f
  12:     0x7f8b6bf93c05 - rustc::traits::query::evaluate_obligation::<impl rustc::infer::InferCtxt<'cx, 'gcx, 'tcx>>::predicate_may_hold::h87cecf81af11bfbf
  13:     0x7f8b6ca576c5 - rustc::infer::InferCtxt::probe::hbcaee7fe8d272818
  14:     0x7f8b6c9c7351 - <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter::h33b1bcee07d0a922
  15:     0x7f8b6ca2263a - rustc_typeck::check::method::probe::ProbeContext::pick_method::h08a8c3a9cc26863b
  16:     0x7f8b6ca21c69 - rustc_typeck::check::method::probe::ProbeContext::pick_core::he86401ef57a4908e
  17:     0x7f8b6ca21100 - rustc_typeck::check::method::probe::ProbeContext::pick::h6b5bdaea8ecf81be
  18:     0x7f8b6ca58626 - rustc::infer::InferCtxt::probe::he1cda8c6528f2271
  19:     0x7f8b6c8c2ad8 - rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::probe_for_name::h923cc02460a5a199
  20:     0x7f8b6c8e0da8 - rustc_typeck::check::FnCtxt::check_expr_kind::h13c56357a12906f8
  21:     0x7f8b6c8e0590 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h053c7550e4cc99ef
  22:     0x7f8b6c8ee638 - rustc_typeck::check::FnCtxt::check_decl_local::hba9044b2f3b9fd7c
  23:     0x7f8b6c8eeba4 - rustc_typeck::check::FnCtxt::check_block_with_expected::hc24b59e8c50e6115
  24:     0x7f8b6c8e0833 - rustc_typeck::check::FnCtxt::check_expr_kind::h13c56357a12906f8
  25:     0x7f8b6c8e0590 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h053c7550e4cc99ef
  26:     0x7f8b6c8e40ac - rustc_typeck::check::FnCtxt::check_expr_kind::h13c56357a12906f8
  27:     0x7f8b6c8e0590 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h053c7550e4cc99ef
  28:     0x7f8b6c8eec14 - rustc_typeck::check::FnCtxt::check_block_with_expected::hc24b59e8c50e6115
  29:     0x7f8b6c8e0833 - rustc_typeck::check::FnCtxt::check_expr_kind::h13c56357a12906f8
  30:     0x7f8b6c8e0590 - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs::h053c7550e4cc99ef
  31:     0x7f8b6c8df54d - rustc_typeck::check::FnCtxt::check_return_expr::h8f4830379570cb21
  32:     0x7f8b6c8d1fdb - rustc_typeck::check::check_fn::hd91f2933739f4611
  33:     0x7f8b6c98ccd6 - rustc::ty::context::tls::with_related_context::h05167d9a9ce47d4a
  34:     0x7f8b6ca4ef28 - rustc::infer::InferCtxtBuilder::enter::h6ec88876adecbbfd
  35:     0x7f8b6c8d09c0 - rustc_typeck::check::typeck_tables_of::h614e0bda432b789a
  36:     0x7f8b6bcac3c5 - rustc::ty::query::__query_compute::typeck_tables_of::h4bcc5f038960d678
  37:     0x7f8b6bcb277c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute::h77c5a99d2d9bc2b3
  38:     0x7f8b6bbf435b - rustc::ty::context::tls::with_context::h3b04e00272090c24
  39:     0x7f8b6b9db85a - rustc::dep_graph::graph::DepGraph::with_task_impl::h1fe3db85e5fd6290
  40:     0x7f8b6bc4d9e5 - rustc::ty::context::tls::with_related_context::h5348af5f2224289c
  41:     0x7f8b6bcdb7fc - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job::h222241c2b29a60b4
  42:     0x7f8b6bd5f1d5 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query::h1d90206fa12bb2a9
  43:     0x7f8b6be427ae - rustc::ty::query::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_tables_of::h4c63aa19f1bf3f3b
  44:     0x7f8b6cd2ca08 - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O>>::process_method::hab8819bf5b516e76
  45:     0x7f8b6cd3b00b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  46:     0x7f8b6cd3b09b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  47:     0x7f8b6cd3b09b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  48:     0x7f8b6cd3b09b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  49:     0x7f8b6cd3b09b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  50:     0x7f8b6cd3b09b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  51:     0x7f8b6cd3b09b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_item::h0bc83381f1ca497c
  52:     0x7f8b6cd3716b - <rustc_save_analysis::dump_visitor::DumpVisitor<'l, 'tcx, 'll, O> as syntax::visit::Visitor<'l>>::visit_mod::h6ff6d1081ab6c29a
  53:     0x7f8b6cd9eec4 - <rustc_save_analysis::CallbackHandler<'b> as rustc_save_analysis::SaveHandler>::save::h4de705bac6974a08
  54:     0x555affc95ed5 - rustc::ty::context::tls::with_context::heee7815d346fc491
  55:     0x555affda9104 - <rls::build::rustc::RlsRustcCalls as rustc_driver::CompilerCalls<'a>>::build_controller::{{closure}}::ha15d6c082771c7a4
  56:     0x7f8b6d148dac - rustc::ty::context::tls::with_context::h2067fd8571d90aa3
  57:     0x7f8b6d153c7e - rustc_driver::driver::compile_input::{{closure}}::h5e6f20613e2b6c66
  58:     0x7f8b6d14b79c - rustc::ty::context::tls::enter_context::h61029341b2639fbc
  59:     0x7f8b6d10daca - <std::thread::local::LocalKey<T>>::with::h3f2d5deb8c8ae448
  60:     0x7f8b6d176d4e - rustc::ty::context::TyCtxt::create_and_enter::hd436a3b31e0d4e24
  61:     0x7f8b6d0b24b8 - rustc_driver::driver::compile_input::hb59e592e87a2a624
  62:     0x7f8b6d1589dc - rustc_driver::run_compiler_with_pool::hcbbe45bf792adfde
  63:     0x7f8b6d0b0284 - rustc_driver::driver::spawn_thread_pool::hfe25d62d50ef401b
  64:     0x7f8b6d1579f1 - rustc_driver::run_compiler::hbcd18c2f5ac41846
  65:     0x555affdd90d2 - <scoped_tls::ScopedKey<T>>::set::hb8ec9f4f33d9cc0d
  66:     0x555affceccab - syntax::with_globals::h33509c8a6cd5d138
  67:     0x555affd1d506 - std::panicking::try::do_call::h7cc66fbda113b25a
  68:     0x7f8b6a1005b9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:103
  69:     0x555affcd266f - rustc_driver::run::h5f2bf7f57781aa63
  70:     0x555affd1cc7c - std::panicking::try::do_call::h36a279773dd0f092
  71:     0x7f8b6a1005b9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:103
  72:     0x555affda7045 - rls::build::rustc::rustc::h21bab17bd4f6f559
  73:     0x555affdfc872 - rls::build::plan::JobQueue::execute::ha786033099f80946
  74:     0x555affdaae39 - rls::build::BuildQueue::run_thread::h845686f7d229864f
  75:     0x555affdd9fae - std::sys_common::backtrace::__rust_begin_short_backtrace::ha6180aa2886e0b73
  76:     0x7f8b6a1005b9 - __rust_maybe_catch_panic
                               at libpanic_unwind/lib.rs:103
  77:     0x555affc59537 - <F as alloc::boxed::FnBox<A>>::call_box::ha0eb1c0e7d8894cf
  78:     0x7f8b6a0edcca - std::sys_common::thread::start_thread::h75d887c7d2cc4479
                               at liballoc/boxed.rs:656
                               at libstd/sys_common/thread.rs:24
  79:     0x7f8b6a0c18f5 - std::sys::unix::thread::Thread::new::thread_start::h8414caee632bf9ed
                               at libstd/sys/unix/thread.rs:90
  80:     0x7f8b6a02558d - start_thread
  81:     0x7f8b69f37592 - clone
  82:                0x0 - <unknown>
query stack during panic:
end of query stack
