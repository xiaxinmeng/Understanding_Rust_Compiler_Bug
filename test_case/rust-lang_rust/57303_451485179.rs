
[01:28:27] {"message":"src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"error: internal compiler error: src/librustc/ty/context.rs:248: node type MacroAttributesTest (id=43) with HirId::owner DefId(0/0:9 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0]::{{impl}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0/0:6 ~ used_underscore_binding_macro[317d]::_IMPL_DESERIALIZE_FOR_MacroAttributesTest[0])\n\n"}
[01:28:27] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:590:9
[01:28:27] stack backtrace:
[01:28:27]    0:     0x7fe8334a8d93 - std::sys::unix::backtrace::tracing::imp::unwind_backtrace::hd98426cd3492fad9
[01:28:27]    1:     0x7fe8334a0d28 - std::sys_common::backtrace::_print::h33dc68ee3471107b
[01:28:27]    2:     0x7fe8334a4b91 - std::panicking::default_hook::{{closure}}::h6e8ca1fbae609b31
[01:28:27]    3:     0x7fe8334a4868 - std::panicking::default_hook::h12d08263585ea668
[01:28:27]    4:     0x7fe8355ba8ef - rustc::util::common::panic_hook::h8664254e5d78c027
[01:28:27]    5:     0x7fe8334a54a4 - std::panicking::rust_panic_with_hook::h80ecd07d9fdfd689
[01:28:27]    6:     0x7fe834319e4c - std::panicking::begin_panic::h606da2437d0c9a78
[01:28:27]    7:     0x7fe83434220e - rustc_errors::Handler::bug::h5f18c9aa3002a3e1
[01:28:27]    8:     0x7fe8350964a3 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::ha70b54c46e545966
[01:28:27]    9:     0x7fe835091a59 - rustc::ty::context::tls::with_opt::{{closure}}::h06335bf584d257c5
[01:28:27]   10:     0x7fe835091974 - rustc::ty::context::tls::with_context_opt::hb4bad76e7a1bb6a7
[01:28:27]   11:     0x7fe835091a06 - rustc::ty::context::tls::with_opt::hd45864d4b45a4b65
[01:28:27]   12:     0x7fe835096384 - rustc::util::bug::opt_span_bug_fmt::h1c27e2cc1d1149c3
[01:28:27]   13:     0x7fe8350962f6 - rustc::util::bug::bug_fmt::h8b515233d6f03333
[01:28:27]   14:     0x7fe834e79b45 - rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}::h686eb8a406d530f8
[01:28:27]   15:     0x7fe834e8f648 - rustc::ty::context::tls::with::{{closure}}::h6680e41cb9e76345
[01:28:27]   16:     0x7fe834e8ddff - rustc::ty::context::tls::with_context::{{closure}}::h219fd61af3d2405c
[01:28:27]   17:     0x7fe834e8dda8 - rustc::ty::context::tls::with_context_opt::h390957d2f03df239
[01:28:27]   18:     0x7fe834e8ddd5 - rustc::ty::context::tls::with_context::h180ddfa25e2e2cf2
[01:28:27]   19:     0x7fe834e8f635 - rustc::ty::context::tls::with::hf144144bba27d185
[01:28:27]   20:     0x7fe834e7ab8d - rustc::ty::context::TypeckTables::node_id_to_type_opt::h22a50a087c54f413
[01:28:27]   21:     0x55d06adadaa0 - <clippy_lints::random_state::Pass as rustc::lint::LateLintPass<'a, 'tcx>>::check_ty::h0e791f3e8996c729
[01:28:27]   22:     0x7fe83557bcbf - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_ty::h72ebdaa9528dee36
[01:28:27]   23:     0x7fe835501e2f - rustc::hir::intravisit::walk_item::he489cc90a12bea7b
[01:28:27]   24:     0x7fe83557a108 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h5728335ea922cc15
[01:28:27]   25:     0x7fe83557c64c - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_block::h72db13ed2a88cb28
[01:28:27]   26:     0x7fe83557aa79 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_expr::hcf6ab50f0c847e80
[01:28:27]   27:     0x7fe835579cdf - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_body::h7e2b7e472ece7e94
[01:28:27]   28:     0x7fe835501c2d - rustc::hir::intravisit::walk_item::he489cc90a12bea7b
[01:28:27]   29:     0x7fe83557a108 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item::h5728335ea922cc15
[01:28:27]   30:     0x7fe83557c0c0 - <rustc::lint::context::LateContext<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_mod::hb9a3c90f0f60db70
[01:28:27]   31:     0x7fe8354f08ae - rustc::hir::intravisit::walk_crate::hde124e87f25baddc
[01:28:27]   32:     0x7fe835582755 - rustc::lint::context::check_crate::h6589d55ae41b1c25
[01:28:27]   33:     0x7fe8365d9d9b - rustc::util::common::time::h7e4a33c762ad15e7
[01:28:27]   34:     0x7fe83667b97a - <std::thread::local::LocalKey<T>>::with::h952b72befc3ce74e
[01:28:27]   35:     0x7fe8365f3a66 - rustc::ty::context::TyCtxt::create_and_enter::h5cc63bc0874b4162
[01:28:27]   36:     0x7fe83665f64e - rustc_driver::driver::compile_input::h03868f75b7316f01
[01:28:27]   37:     0x7fe8365c262a - rustc_driver::run_compiler_with_pool::h741dc60e958f63f8
[01:28:27]   38:     0x7fe8365de9b5 - <scoped_tls::ScopedKey<T>>::set::hc89acbedf3eb8704
[01:28:27]   39:     0x7fe8365c158a - rustc_driver::run_compiler::hce605fc8eee3e069
[01:28:27]   40:     0x55d06acd403c - <scoped_tls::ScopedKey<T>>::set::h9bd1cbe659af503f
[01:28:27]   41:     0x55d06acbdaa2 - std::sys_common::backtrace::__rust_begin_short_backtrace::h79264eee389336bd
[01:28:27]   42:     0x7fe8334b85f9 - __rust_maybe_catch_panic
[01:28:27]   43:     0x55d06acb5792 - std::panicking::try::h5193e9903c6aa0c4
[01:28:27]   44:     0x55d06acbeed5 - <F as alloc::boxed::FnBox<A>>::call_box::hf882d8e28a85db34
[01:28:27]   45:     0x7fe8334b723d - std::sys::unix::thread::Thread::new::thread_start::h013506ce4de301ef
[01:28:27]   46:     0x7fe83321d6b9 - start_thread
[01:28:27]   47:     0x7fe832d3d41c - clone
[01:28:27]   48:                0x0 - <unknown>
[01:28:27] query stack during panic:
[01:28:27] end of query stack
