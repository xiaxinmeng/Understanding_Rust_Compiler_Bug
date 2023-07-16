
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:912:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1028
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::to_region_vid::{{closure}}
  22: rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::fold_to_region_vids::{{closure}}
  23: rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with
  24: rustc_mir::borrow_check::nll::universal_regions::UniversalRegions::new
  25: rustc_mir::borrow_check::nll::replace_regions_in_mir
  26: rustc_mir::borrow_check::do_mir_borrowck
  27: rustc::ty::context::GlobalCtxt::enter_local
  28: rustc_mir::borrow_check::mir_borrowck
  29: rustc::ty::query::__query_compute::mir_borrowck
  30: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute
  31: rustc::dep_graph::graph::DepGraph::with_task_impl
  32: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  33: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query
  34: rustc_mir::transform::optimized_mir
  35: rustc::ty::query::__query_compute::optimized_mir
  36: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute
  37: rustc::dep_graph::graph::DepGraph::with_task_impl
  38: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  39: rustc_mir::interpret::eval_context::InterpCx<M>::load_mir
  40: rustc_mir::const_eval::const_eval_raw_provider
  41: rustc::ty::query::__query_compute::const_eval_raw
  42: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute
  43: rustc::dep_graph::graph::DepGraph::with_task_impl
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: rustc_mir::const_eval::const_eval_provider
  46: rustc::ty::query::__query_compute::const_eval
  47: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  48: rustc::dep_graph::graph::DepGraph::with_task_impl
  49: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  50: rustc_mir::const_eval::const_eval_provider
  51: rustc::ty::query::__query_compute::const_eval
  52: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute
  53: rustc::dep_graph::graph::DepGraph::with_task_impl
  54: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  55: rustc::ty::sty::Const::eval
  56: <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter
  57: rustc::ty::fold::TypeFoldable::fold_with
  58: <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty
  59: rustc::traits::project::normalize
  60: rustc_typeck::check::FnCtxt::instantiate_value_path
  61: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  62: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  63: rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call
  64: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  65: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  66: rustc_typeck::check::FnCtxt::check_block_with_expected
  67: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind
  68: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs
  69: rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr
  70: rustc_typeck::check::check_fn
  71: rustc::ty::context::GlobalCtxt::enter_local
  72: rustc_typeck::check::typeck_tables_of
  73: rustc::ty::query::__query_compute::typeck_tables_of
  74: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute
  75: rustc::dep_graph::graph::DepGraph::with_task_impl
  76: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  77: rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners
  78: rustc_typeck::check::typeck_item_bodies
  79: rustc::ty::query::__query_compute::typeck_item_bodies
  80: rustc::dep_graph::graph::DepGraph::with_task_impl
  81: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  82: rustc::util::common::time
  83: rustc_typeck::check_crate
  84: rustc_interface::passes::analysis
  85: rustc::ty::query::__query_compute::analysis
  86: rustc::dep_graph::graph::DepGraph::with_task_impl
  87: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  88: rustc::ty::context::tls::enter_global
  89: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  90: rustc_interface::passes::create_global_ctxt::{{closure}}
  91: rustc_interface::interface::run_compiler_in_existing_thread_pool
  92: std::thread::local::LocalKey<T>::with
  93: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (084beb83e 2019-09-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1165:5
stack backtrace:
   0:     0x7fee13d27af4 - backtrace::backtrace::libunwind::trace::ha5983032afb4d10c
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1:     0x7fee13d27af4 - backtrace::backtrace::trace_unsynchronized::hcde1d0d574f77864
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2:     0x7fee13d27af4 - std::sys_common::backtrace::_print_fmt::he5e6e380d77c46da
                               at src/libstd/sys_common/backtrace.rs:76
   3:     0x7fee13d27af4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2828e17b2f96be03
                               at src/libstd/sys_common/backtrace.rs:60
   4:     0x7fee13d6013c - core::fmt::write::h56881cea5b0fc863
                               at src/libcore/fmt/mod.rs:1028
   5:     0x7fee13d1bd17 - std::io::Write::write_fmt::h7e23e2336d419271
                               at src/libstd/io/mod.rs:1412
   6:     0x7fee13d2c325 - std::sys_common::backtrace::_print::h17d82b35dd2dffa0
                               at src/libstd/sys_common/backtrace.rs:64
   7:     0x7fee13d2c325 - std::sys_common::backtrace::print::h8a39b56b63e48ce4
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7fee13d2c325 - std::panicking::default_hook::{{closure}}::h5eeec7a4ddaaba52
                               at src/libstd/panicking.rs:196
   9:     0x7fee13d2c016 - std::panicking::default_hook::h66510563f6c6226f
                               at src/libstd/panicking.rs:210
  10:     0x7fee142585b3 - rustc_driver::report_ice::h1c629111e9f307b8
  11:     0x7fee13d2cb0c - std::panicking::rust_panic_with_hook::h12ec097c8ec6e99f
                               at src/libstd/panicking.rs:477
  12:     0x7fee13d2c5c2 - std::panicking::continue_panic_fmt::h5bf5b33e69b73cec
                               at src/libstd/panicking.rs:380
  13:     0x7fee13d2c4b6 - rust_begin_unwind
                               at src/libstd/panicking.rs:307
  14:     0x7fee13d59aca - core::panicking::panic_fmt::he80091724924a82c
                               at src/libcore/panicking.rs:84
  15:     0x7fee13d59d07 - core::result::unwrap_failed::h72740bd67ffc7cd7
                               at src/libcore/result.rs:1165
  16:     0x7fee1605eea5 - rustc_errors::Handler::force_print_diagnostic::h39e643eb1cc2a78a
  17:     0x7fee15934031 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack::h050a2688d5964137
  18:     0x7fee142591c0 - rustc_driver::report_ice::h1c629111e9f307b8
  19:     0x7fee13d2cb0c - std::panicking::rust_panic_with_hook::h12ec097c8ec6e99f
                               at src/libstd/panicking.rs:477
  20:     0x7fee1604589d - std::panicking::begin_panic::h4199802c8e4d790b
  21:     0x7fee1605f7c3 - rustc_errors::HandlerInner::bug::h9cd50550a7a56234
  22:     0x7fee1605e48a - rustc_errors::Handler::bug::hafbfb033a9f794cc
  23:     0x7fee158f48f3 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::ha31ac8110405284c
  24:     0x7fee158efbb3 - rustc::ty::context::tls::with_opt::{{closure}}::h28854b2634b6f3b7
  25:     0x7fee158efb23 - rustc::ty::context::tls::with_context_opt::h03c933b0a2d32a88
  26:     0x7fee158efb67 - rustc::ty::context::tls::with_opt::ha89131ef0a05072b
  27:     0x7fee158f4808 - rustc::util::bug::opt_span_bug_fmt::h9b3e8e6451c1aa0d
  28:     0x7fee158f4772 - rustc::util::bug::bug_fmt::h5291243634e000d6
  29:     0x7fee14d5f252 - rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::to_region_vid::{{closure}}::h8612057275a898c1
  30:     0x7fee14c723f8 - rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::fold_to_region_vids::{{closure}}::h0953af38a7078e37
  31:     0x7fee14c7c934 - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with::h8b02d7fb2dcab4b0
  32:     0x7fee14d5d536 - rustc_mir::borrow_check::nll::universal_regions::UniversalRegions::new::h7fcc25cec5b78174
  33:     0x7fee14ee654c - rustc_mir::borrow_check::nll::replace_regions_in_mir::he9d731fff3d7a090
  34:     0x7fee14ca7ebf - rustc_mir::borrow_check::do_mir_borrowck::hc39a57967360e334
  35:     0x7fee14c6d871 - rustc::ty::context::GlobalCtxt::enter_local::h29949b025c5ba6d1
  36:     0x7fee14ca7b7d - rustc_mir::borrow_check::mir_borrowck::h018e7e1ada6edfeb
  37:     0x7fee14cc7412 - rustc::ty::query::__query_compute::mir_borrowck::h8dc8b1616af15f3a
  38:     0x7fee14be798c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::hdcb9f6976a4a08bb
  39:     0x7fee14d38eb4 - rustc::dep_graph::graph::DepGraph::with_task_impl::hde7fb5c62185a36b
  40:     0x7fee14c457e5 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hb4708141f0981130
  41:     0x7fee14be7f42 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query::h7547adcea9b278af
  42:     0x7fee14bd2206 - rustc_mir::transform::optimized_mir::hf03249ac0924248a
  43:     0x7fee14cc7877 - rustc::ty::query::__query_compute::optimized_mir::h794b24a7edaf132b
  44:     0x7fee14be7a3b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute::h3f30d6f9c2410248
  45:     0x7fee14d33a7a - rustc::dep_graph::graph::DepGraph::with_task_impl::ha52cfdd56dfba685
  46:     0x7fee14c6917b - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hfd16edc3b4feea06
  47:     0x7fee14d69187 - rustc_mir::interpret::eval_context::InterpCx<M>::load_mir::h2de2515521415ee7
  48:     0x7fee14bd934e - rustc_mir::const_eval::const_eval_raw_provider::h671d68a4defeaffb
  49:     0x7fee14cc7b83 - rustc::ty::query::__query_compute::const_eval_raw::hf13b2828fb4ba75b
  50:     0x7fee14be7aba - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::hb9ed27d8ad9172ec
  51:     0x7fee14d27d9f - rustc::dep_graph::graph::DepGraph::with_task_impl::h1c6091fc643d2537
  52:     0x7fee14c3be1d - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h91e0da907a6a694d
  53:     0x7fee14bd83a1 - rustc_mir::const_eval::const_eval_provider::h8e2a58add2486388
  54:     0x7fee14cc6f73 - rustc::ty::query::__query_compute::const_eval::hfd2ea6f750148679
  55:     0x7fee14be78ca - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::h6c2a76ec5e5ab95d
  56:     0x7fee14d360ca - rustc::dep_graph::graph::DepGraph::with_task_impl::hcb79c6e82af5d3c7
  57:     0x7fee14c2650b - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h5229152ade8d4d0a
  58:     0x7fee14bd814a - rustc_mir::const_eval::const_eval_provider::h8e2a58add2486388
  59:     0x7fee157ddcea - rustc::ty::query::__query_compute::const_eval::hc34072f292851a88
  60:     0x7fee15afcf8a - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::h6c2a76ec5e5ab95d
  61:     0x7fee1568e596 - rustc::dep_graph::graph::DepGraph::with_task_impl::heefbde2df5b44b1a
  62:     0x7fee15a17d64 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hf11cd80153774b60
  63:     0x7fee15b5329e - rustc::ty::sty::Const::eval::h0eae12e23a55375c
  64:     0x7fee1571e5bd - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h8d0b1b3ea78f477f
  65:     0x7fee15b40abb - rustc::ty::fold::TypeFoldable::fold_with::hb1bda0c820fbf271
  66:     0x7fee15b36de5 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h747449a17c0dab0b
  67:     0x7fee149c5402 - rustc::traits::project::normalize::hd2067db985d0231a
  68:     0x7fee147f1cf7 - rustc_typeck::check::FnCtxt::instantiate_value_path::h50ee1884851af0d7
  69:     0x7fee147ba5d7 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::he47250f016a8eef2
  70:     0x7fee147b744b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::h0c40a8d4b8ce32dc
  71:     0x7fee147d15b9 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::hbcf8c36ff5442110
  72:     0x7fee147b806f - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::he47250f016a8eef2
  73:     0x7fee147b744b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::h0c40a8d4b8ce32dc
  74:     0x7fee147eddd8 - rustc_typeck::check::FnCtxt::check_block_with_expected::h96f4b7ca7329cd3f
  75:     0x7fee147b808a - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::he47250f016a8eef2
  76:     0x7fee147b744b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::h0c40a8d4b8ce32dc
  77:     0x7fee147c153b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr::h97fe5415cfd05f0b
  78:     0x7fee147daefd - rustc_typeck::check::check_fn::h859fc3661b72bd16
  79:     0x7fee1494b9e8 - rustc::ty::context::GlobalCtxt::enter_local::hac2f7370bf276850
  80:     0x7fee147da038 - rustc_typeck::check::typeck_tables_of::h59f8043f3f17ac24
  81:     0x7fee1482b88a - rustc::ty::query::__query_compute::typeck_tables_of::h42163918d08f8266
  82:     0x7fee148c29db - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::h09b0f5088548e8ae
  83:     0x7fee14852eda - rustc::dep_graph::graph::DepGraph::with_task_impl::h723db1099faa82fb
  84:     0x7fee1492ea6b - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hc60a06724fc07dfe
  85:     0x7fee148bf76f - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::hcfb519f359bb6929
  86:     0x7fee147d9aad - rustc_typeck::check::typeck_item_bodies::hc913bf0ca937a1f4
  87:     0x7fee1482bcd5 - rustc::ty::query::__query_compute::typeck_item_bodies::h758eadccaa8a2c0c
  88:     0x7fee148547fe - rustc::dep_graph::graph::DepGraph::with_task_impl::h976b38fc2413d3fd
  89:     0x7fee148de27b - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h27900a70a9748c27
  90:     0x7fee148305ee - rustc::util::common::time::hcd7c3e9497174d75
  91:     0x7fee14a0b1ef - rustc_typeck::check_crate::h892282f507cd6731
  92:     0x7fee143043ca - rustc_interface::passes::analysis::hd64918a47f1f7d3a
  93:     0x7fee14216c61 - rustc::ty::query::__query_compute::analysis::h0229eceff6d5a7c8
  94:     0x7fee141fa8fd - rustc::dep_graph::graph::DepGraph::with_task_impl::h8bf215e7a8cc2169
  95:     0x7fee1420cd9e - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h8e7e99bf128d8deb
  96:     0x7fee1424fa05 - rustc::ty::context::tls::enter_global::h534d47119b4c622e
  97:     0x7fee1422ccc7 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::hcffbc94c8465e346
  98:     0x7fee143356db - rustc_interface::passes::create_global_ctxt::{{closure}}::hca029ea5891e3746
  99:     0x7fee1422de2b - rustc_interface::interface::run_compiler_in_existing_thread_pool::h25b2e9493fee162d
 100:     0x7fee14259b52 - std::thread::local::LocalKey<T>::with::hcac2a20d00f0f99f
 101:     0x7fee142737d0 - syntax::with_globals::h7fb4835074222545
 102:     0x7fee14212e40 - std::sys_common::backtrace::__rust_begin_short_backtrace::h098e301179af4344
 103:     0x7fee13d3d25a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
 104:     0x7fee14218fe9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h395db1c737913721
 105:     0x7fee13d0e1ff - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hb018c40cf7dc8a37
                               at /rustc/084beb83e0e87d673d5fabc844d28e8e8ae2ab4c/src/liballoc/boxed.rs:922
 106:     0x7fee13d3bf00 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h10e62b586f634658
                               at /rustc/084beb83e0e87d673d5fabc844d28e8e8ae2ab4c/src/liballoc/boxed.rs:922
 107:     0x7fee13d3bf00 - std::sys_common::thread::start_thread::h6ada4f3bef71d083
                               at src/libstd/sys_common/thread.rs:13
 108:     0x7fee13d3bf00 - std::sys::unix::thread::Thread::new::thread_start::h65408e9b6d37b640
                               at src/libstd/sys/unix/thread.rs:79
 109:     0x7fee13aa86db - start_thread
 110:     0x7fee133c588f - __clone
 111:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.40.0-nightly (084beb83e 2019-09-27) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread panicked while processing panic. aborting.
error: could not compile `playground`.
