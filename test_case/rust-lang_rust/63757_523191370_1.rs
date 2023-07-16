
stack backtrace:
   0:     0x7fb80251536b - backtrace::backtrace::libunwind::trace::hfe5db90796807973
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1:     0x7fb80251536b - backtrace::backtrace::trace_unsynchronized::h34b865a835594335
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2:     0x7fb80251536b - std::sys_common::backtrace::_print::h527254ae44989167
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7fb80251536b - std::sys_common::backtrace::print::he85dd5ddddf46503
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7fb80251536b - std::panicking::default_hook::{{closure}}::h847a2eb38b396f14
                               at src/libstd/panicking.rs:200
   5:     0x7fb802515047 - std::panicking::default_hook::h2ca0f9a30a0e206b
                               at src/libstd/panicking.rs:214
   6:     0x7fb8002af211 - rustc::util::common::panic_hook::hd4598aac709d8bd1
   7:     0x7fb802515bc9 - std::panicking::rust_panic_with_hook::hffcefc09751839d1
                               at src/libstd/panicking.rs:481
   8:     0x7fb7fefc028d - std::panicking::begin_panic::hbdbc29ebf0daafe7
   9:     0x7fb7fefe504f - rustc_errors::Handler::bug::hac03def1bed26580
  10:     0x7fb7fff57d93 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h8da2753ce5ce84a1
  11:     0x7fb7fff5742a - rustc::ty::context::tls::with_opt::{{closure}}::h7fd59b8903c62a0f
  12:     0x7fb7fff56e98 - rustc::ty::context::tls::with_context_opt::h2b67407fb5157e28
  13:     0x7fb7fff56ed7 - rustc::ty::context::tls::with_opt::h7257a6f6cf9ce1f6
  14:     0x7fb7fff57ca8 - rustc::util::bug::opt_span_bug_fmt::h51b5f4b1a8f84076
  15:     0x7fb7fff57c12 - rustc::util::bug::bug_fmt::hd38e959312cbb387
  16:     0x7fb80022cdf0 - rustc::ty::context::TypeckTables::node_type::{{closure}}::h2a684910b509756e
  17:     0x7fb80022cd1b - rustc::ty::context::TypeckTables::node_type::h5b18e9e9ac8683d4
  18:     0x7fb7fd711afe - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_ty::hd07128605fe89579
  19:     0x7fb7fd6c7c39 - rustc::hir::intravisit::walk_generic_args::h093ae524efd47efe
  20:     0x7fb7fd6c3341 - rustc::hir::intravisit::walk_qpath::haa793573d506b50c
  21:     0x7fb7fd711bca - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_ty::hd07128605fe89579
  22:     0x7fb7fd6c32e1 - rustc::hir::intravisit::walk_qpath::haa793573d506b50c
  23:     0x7fb7fd7121fa - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_expr::h084e0e1ae6d0e007
  24:     0x7fb7fd6d03a5 - rustc::hir::intravisit::walk_expr::h356c97deb8483ded
  25:     0x7fb7fd7121fa - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_expr::h084e0e1ae6d0e007
  26:     0x7fb7fd712846 - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_local::hc5a43f0ca7adc87d
  27:     0x7fb7fd6d061f - rustc::hir::intravisit::walk_expr::h356c97deb8483ded
  28:     0x7fb7fd7121fa - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_expr::h084e0e1ae6d0e007
  29:     0x7fb7fd711aae - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_nested_body::h86389157ebf84257
  30:     0x7fb7fd6c562f - rustc::hir::intravisit::walk_impl_item::ha54f4b043f4bcdd1
  31:     0x7fb7fd70ad1f - rustc::hir::intravisit::Visitor::visit_nested_impl_item::h38c546e4449f9849
  32:     0x7fb7fd6d21b5 - rustc::hir::intravisit::walk_item::h501c6ab140b7aa01
  33:     0x7fb7fd71294c - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_item::h25613f840b53b971
  34:     0x7fb7fd6d061f - rustc::hir::intravisit::walk_expr::h356c97deb8483ded
  35:     0x7fb7fd7121fa - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_expr::h084e0e1ae6d0e007
  36:     0x7fb7fd711aae - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_nested_body::h86389157ebf84257
  37:     0x7fb7fd6d22af - rustc::hir::intravisit::walk_item::h501c6ab140b7aa01
  38:     0x7fb7fd71294c - <rustc_privacy::TypePrivacyVisitor as rustc::hir::intravisit::Visitor>::visit_item::h25613f840b53b971
  39:     0x7fb7fd71570e - rustc_privacy::check_mod_privacy::hf7d6406783fd7924
  40:     0x7fb801f8359e - rustc::ty::query::__query_compute::check_mod_privacy::h914acb0b8a18cdef
  41:     0x7fb801f93ebd - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_privacy>::compute::h3374e77b45c3593c
  42:     0x7fb801fc516b - rustc::dep_graph::graph::DepGraph::with_task_impl::h8e282d89d86a6b2a
  43:     0x7fb801fa4ff0 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h9423711ccc041fbc
  44:     0x7fb801f553d0 - rustc_interface::passes::analysis::{{closure}}::{{closure}}::{{closure}}::hfa200fe680f150af
  45:     0x7fb801f50570 - rustc::util::common::time::h3fa170c316ff0b81
  46:     0x7fb80252648a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  47:     0x7fb801f55601 - rustc_interface::passes::analysis::{{closure}}::h7f98554b222098ac
  48:     0x7fb801f50617 - rustc::util::common::time::h43ab6f0cd13f3932
  49:     0x7fb801f0c250 - rustc_interface::passes::analysis::h39d33eb2a40812a3
  50:     0x7fb8027d9246 - rustc::ty::query::__query_compute::analysis::hf896d9719452bef1
  51:     0x7fb8027d8f99 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::analysis>::compute::h314df7d2740153f0
  52:     0x7fb80286e138 - rustc::dep_graph::graph::DepGraph::with_task_impl::h5bfe7e398dba7700
  53:     0x7fb8027de3be - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hf803a152be45b4ee
  54:     0x7fb8027dfda6 - rustc::ty::context::tls::enter_global::h7f57e6c49574182e
  55:     0x7fb8027e42f7 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h04fd22292950fbf4
  56:     0x7fb801f5443a - rustc_interface::passes::create_global_ctxt::{{closure}}::h70f679c0b82043da
  57:     0x7fb8027e55f8 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h1ee6f3bf0ba37faf
  58:     0x7fb802807d42 - std::thread::local::LocalKey<T>::with::h83b0a2026f4a2c6e
  59:     0x7fb80281b891 - scoped_tls::ScopedKey<T>::set::h47171c9859321621
  60:     0x7fb8028338b4 - syntax::with_globals::h7dd1a1171ce8a4bf
  61:     0x7fb80285eb1d - std::sys_common::backtrace::__rust_begin_short_backtrace::h1ed9f0eaa05c0996
  62:     0x7fb80252648a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  63:     0x7fb8027e7cc9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h3b9a37e8ad92df6e
  64:     0x7fb8024f8e0f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h42806b83647d4c79
                               at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b/src/liballoc/boxed.rs:746
  65:     0x7fb802525170 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h83c921c8e826dd1d
                               at /rustc/eae3437dfe991621e8afdc82734f4a172d7ddf9b/src/liballoc/boxed.rs:746
  66:     0x7fb802525170 - std::sys_common::thread::start_thread::h2613204ce513782e
                               at src/libstd/sys_common/thread.rs:13
  67:     0x7fb802525170 - std::sys::unix::thread::Thread::new::thread_start::h4570080769500bcd
                               at src/libstd/sys/unix/thread.rs:79
  68:     0x7fb802464fa3 - start_thread
  69:     0x7fb8023844cf - clone
  70:                0x0 - <unknown>
