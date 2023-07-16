bash
RUST_BACKTRACE=full rustc rustc_test.rs --verbose
warning: the feature `inline_const` is incomplete and may not be safe to use and/or cause compiler crashes
 --> rustc_test.rs:1:12
  |
1 | #![feature(inline_const)]
  |            ^^^^^^^^^^^^
  |
  = note: `#[warn(incomplete_features)]` on by default
  = note: see issue #76001 <https://github.com/rust-lang/rust/issues/76001> for more information

error: internal compiler error: compiler/rustc_mir/src/borrow_check/universal_regions.rs:768:36: cannot convert `ReErased` to a region vid

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0:     0x7fd29c957dd7 - std::backtrace_rs::backtrace::libunwind::trace::h8bc78f7bc1f75bdb
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fd29c957dd7 - std::backtrace_rs::backtrace::trace_unsynchronized::ha4c961c9576f95ba
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fd29c957dd7 - std::sys_common::backtrace::_print_fmt::hc20cfdc233d6eb02
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fd29c957dd7 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h04bea14549780a0f
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fd29c9c8eac - core::fmt::write::h3868db8542c90941
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/core/src/fmt/mod.rs:1096:17
   5:     0x7fd29c94a602 - std::io::Write::write_fmt::h55dd75636eef6a5c
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/io/mod.rs:1568:15
   6:     0x7fd29c95bd35 - std::sys_common::backtrace::_print::h6439e8525077c7da
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fd29c95bd35 - std::sys_common::backtrace::print::h02b685f728ce3931
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fd29c95bd35 - std::panicking::default_hook::{{closure}}::h7c29625065fafae1
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/panicking.rs:208:50
   9:     0x7fd29c95b893 - std::panicking::default_hook::hba441710bb71f4d5
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/panicking.rs:225:9
  10:     0x7fd29d1a6e7b - rustc_driver::report_ice::h37fd6cf635aca33e
  11:     0x7fd29c95c636 - std::panicking::rust_panic_with_hook::ha936eb00e51bc4d9
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/panicking.rs:595:17
  12:     0x7fd29e98934e - std::panicking::begin_panic::{{closure}}::h8c774868ca681eb9
  13:     0x7fd29e989189 - std::sys_common::backtrace::__rust_end_short_backtrace::hc65b9c179e1c4891
  14:     0x7fd29e9892c2 - std::panicking::begin_panic::h0d9e7df699875d27
  15:     0x7fd29e9a3ed1 - rustc_errors::HandlerInner::bug::hbaa090ab1ded7bb6
  16:     0x7fd29e9a39a3 - rustc_errors::Handler::bug::h784002127398962f
  17:     0x7fd29e6404d3 - rustc_middle::ty::context::tls::with_opt::h22c3633dadd4a591
  18:     0x7fd29ffb3fb0 - rustc_middle::util::bug::opt_span_bug_fmt::he0f6a69007322a94
  19:     0x7fd29e642a46 - rustc_middle::util::bug::bug_fmt::hb9092427708f5234
  20:     0x7fd29eeffe04 - rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::to_region_vid::hcd54ce08222ec0f2
  21:     0x7fd29fada447 - rustc_mir::borrow_check::universal_regions::UniversalRegionIndices::fold_to_region_vids::{{closure}}::h8cd5196ba3171e1b
  22:     0x7fd29ef881bb - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::fold::TypeFoldable for &rustc_middle::ty::TyS>::super_fold_with::h11c9ef08789b2f51
  23:     0x7fd29eefded2 - rustc_mir::borrow_check::universal_regions::UniversalRegions::new::h639e92b61a2055f1
  24:     0x7fd29ef93e90 - rustc_mir::borrow_check::nll::replace_regions_in_mir::h54860e27391f494c
  25:     0x7fd29f0675b9 - rustc_mir::borrow_check::do_mir_borrowck::hdf528544d54d063d
  26:     0x7fd29eeec523 - rustc_infer::infer::InferCtxtBuilder::enter::h59b5500562cb32ad
  27:     0x7fd29f0671f7 - rustc_mir::borrow_check::mir_borrowck::hebc4eb598255ce37
  28:     0x7fd29f04d2e2 - core::ops::function::FnOnce::call_once::h09638d016250ed9a
  29:     0x7fd29fb2ec3c - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_borrowck>::compute::h395844f5fc7d77d7
  30:     0x7fd29faee203 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc48c4be8e5312fe3
  31:     0x7fd29fb1f557 - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::hbfc12dc99fb15ddc
  32:     0x7fd29f0276c8 - rustc_query_system::query::plumbing::force_query_with_job::h5eee9b637db5e65a
  33:     0x7fd29f00ccba - rustc_query_system::query::plumbing::get_query_impl::h0ea6e59e6b1658e3
  34:     0x7fd29f024f61 - rustc_query_system::query::plumbing::ensure_query_impl::hbd1276df36ffc8a1
  35:     0x7fd29efa8a40 - rustc_mir::transform::mir_drops_elaborated_and_const_checked::h10b586d8efb1755d
  36:     0x7fd29fb2f4c5 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_drops_elaborated_and_const_checked>::compute::h62f89f0d06eda4d1
  37:     0x7fd29ef3f4bb - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h4411aaff5650393f
  38:     0x7fd29ef79604 - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::h71131bf848f5cfc8
  39:     0x7fd29f025c2e - rustc_query_system::query::plumbing::force_query_with_job::h0bb86e0bc100ae08
  40:     0x7fd29f01a350 - rustc_query_system::query::plumbing::get_query_impl::hbabb98d3c0c3d028
  41:     0x7fd29fb41568 - rustc_mir::transform::inner_mir_for_ctfe::h9d103d40459c733d
  42:     0x7fd29fb412ff - rustc_mir::transform::mir_for_ctfe::h406ee07acdcf9544
  43:     0x7fd29fb89643 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::mir_for_ctfe>::compute::he1b87282b60170af
  44:     0x7fd29fae7a59 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h19c7a79d519afa82
  45:     0x7fd29fb1f3c0 - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::hae63b09be8b6045c
  46:     0x7fd29fb77939 - rustc_query_system::query::plumbing::force_query_with_job::hd2b7966b4328f9ac
  47:     0x7fd29f023221 - rustc_query_system::query::plumbing::get_query_impl::hfa017622789aa922
  48:     0x7fd29f03d829 - <rustc_mir::const_eval::machine::CompileTimeInterpreter as rustc_mir::interpret::machine::Machine>::load_mir::h23d591fce92bfeff
  49:     0x7fd29efb5b36 - rustc_mir::interpret::eval_context::InterpCx<M>::load_mir::h9e30eaeaf493e920
  50:     0x7fd29ef97e81 - rustc_mir::const_eval::eval_queries::eval_to_allocation_raw_provider::h056681c51754c5da
  51:     0x7fd29ef8ad40 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_allocation_raw>::compute::h0820a60fd123ae22
  52:     0x7fd29ef401b2 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h6a074bb31b9c6ce2
  53:     0x7fd29ef797dc - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::hfd23ec28a714d469
  54:     0x7fd29f029acc - rustc_query_system::query::plumbing::force_query_with_job::he00bf651452da528
  55:     0x7fd29f01f47d - rustc_query_system::query::plumbing::get_query_impl::he79688a38a45fcc4
  56:     0x7fd29ef975f4 - rustc_mir::const_eval::eval_queries::eval_to_const_value_raw_provider::hec70fc8111a3ff16
  57:     0x7fd2a00aa527 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::eval_to_const_value_raw>::compute::h6f2dcf7532e99e98
  58:     0x7fd2a00c22f6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h94ced709c6eb6cb2
  59:     0x7fd29ff73df6 - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::h7b6ec1f7b0d27337
  60:     0x7fd2a002aa6e - rustc_query_system::query::plumbing::force_query_with_job::hebd3a0671de4abfb
  61:     0x7fd29f465a8a - rustc_query_system::query::plumbing::get_query_impl::hedcea7d23f0ed699
  62:     0x7fd29f49835e - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_global_id::h46d4c7b460fb5749
  63:     0x7fd29f4981e6 - rustc_middle::mir::interpret::queries::<impl rustc_middle::ty::context::TyCtxt>::const_eval_resolve::hc08f3f02088b61e5
  64:     0x7fd29ff00106 - rustc_infer::infer::InferCtxt::const_eval_resolve::hed73d794d672f37f
  65:     0x7fd29feaa062 - rustc_trait_selection::traits::const_evaluatable::is_const_evaluatable::hae95f3c9066f1bfd
  66:     0x7fd29f321e83 - rustc_trait_selection::traits::fulfill::FulfillProcessor::progress_changed_obligations::hcafff64f0c4d9d16
  67:     0x7fd29f31fecc - <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible::hcdf3963565990d57
  68:     0x7fd29ec09158 - rustc_typeck::check::fn_ctxt::_impl::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::resolve_vars_with_obligations::h902692cb0e094ab2
  69:     0x7fd29ebfdc9e - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h9c8f6c6a31c336ec
  70:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  71:     0x7fd29ebf404a - rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::demand_scrutinee_type::he12879f368bab552
  72:     0x7fd29ebf18cd - rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_match::hfcfe78d89037d14f
  73:     0x7fd29ebfe29a - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h9c8f6c6a31c336ec
  74:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  75:     0x7fd29ebfdb9b - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h9c8f6c6a31c336ec
  76:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  77:     0x7fd29ec10ed6 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::h99ba64ec1cf9c2b1
  78:     0x7fd29ebf68c1 - rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call::h153cccd6a4c12dca
  79:     0x7fd29ebf5036 - rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call::hf104b007f2699d43
  80:     0x7fd29ebfd660 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h9c8f6c6a31c336ec
  81:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  82:     0x7fd29ec10ed6 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_argument_types::h99ba64ec1cf9c2b1
  83:     0x7fd29ebf68c1 - rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::confirm_builtin_call::h153cccd6a4c12dca
  84:     0x7fd29ebf5036 - rustc_typeck::check::callee::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_call::hf104b007f2699d43
  85:     0x7fd29ebfd660 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h9c8f6c6a31c336ec
  86:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  87:     0x7fd29ec13296 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_stmt::h1afb3e5a89e5a4e7
  88:     0x7fd29ec13bde - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::h83176d6382f0d989
  89:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  90:     0x7fd29ebf1dd4 - rustc_typeck::check::_match::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_match::hfcfe78d89037d14f
  91:     0x7fd29ebfe29a - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind::h9c8f6c6a31c336ec
  92:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  93:     0x7fd29ec13c12 - rustc_typeck::check::fn_ctxt::checks::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_block_with_expected::h83176d6382f0d989
  94:     0x7fd29ebfcca5 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_with_expectation::h668aacf1445c1039
  95:     0x7fd29ec08de8 - rustc_typeck::check::expr::<impl rustc_typeck::check::fn_ctxt::FnCtxt>::check_return_expr::h54463ee81938e3d6
  96:     0x7fd29ec37347 - rustc_typeck::check::check::check_fn::he2baa889e325e382
  97:     0x7fd29ed00c95 - rustc_infer::infer::InferCtxtBuilder::enter::h18be5003a101277b
  98:     0x7fd29ed14972 - rustc_typeck::check::typeck::h8f3495e9bce67997
  99:     0x7fd29ecd8d1c - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck>::compute::h607d355617009d88
 100:     0x7fd29ec2d0d3 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h93385f5a8af9d1dc
 101:     0x7fd29ec66377 - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::hd1327402485358fc
 102:     0x7fd29eccba65 - rustc_query_system::query::plumbing::force_query_with_job::ha22b6ece93ab92d4
 103:     0x7fd29ecb2b3b - rustc_query_system::query::plumbing::get_query_impl::hae5fe0b1b43ec4d0
 104:     0x7fd29ecc94cd - rustc_query_system::query::plumbing::ensure_query_impl::h84c59d2b2ea1889d
 105:     0x7fd29ed14ebc - rustc_typeck::check::typeck_item_bodies::h96adcded8516ae58
 106:     0x7fd29f8b712c - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::typeck_item_bodies>::compute::h03033854948d4022
 107:     0x7fd29f859c63 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h8072307f7fb390aa
 108:     0x7fd29f89e7c0 - rustc_data_structures::stack::ensure_sufficient_stack::hdedc74cf40422b9f
 109:     0x7fd29f8e355a - rustc_query_system::query::plumbing::force_query_with_job::h9c9e33b0c144ba91
 110:     0x7fd29f8cc6fd - rustc_query_system::query::plumbing::get_query_impl::h439524394e85f7c3
 111:     0x7fd29f8c3082 - rustc_typeck::check_crate::h9096345493cae2cf
 112:     0x7fd29f59496a - rustc_interface::passes::analysis::he1e5a586aa49fab5
 113:     0x7fd29f533639 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute::hf86d967bbfab6f9f
 114:     0x7fd29f53146e - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hfae081e53842e2a9
 115:     0x7fd29f533717 - rustc_middle::ty::query::plumbing::<impl rustc_query_system::query::QueryContext for rustc_middle::ty::context::TyCtxt>::start_query::{{closure}}::{{closure}}::{{closure}}::h20150e26d5cd5b61
 116:     0x7fd29f518ff9 - rustc_query_system::query::plumbing::force_query_with_job::hd80e52c22fb6a699
 117:     0x7fd29f5183f0 - rustc_query_system::query::plumbing::get_query_impl::h7fcc3cef27e15e0d
 118:     0x7fd29f5338da - rustc_interface::passes::QueryContext::enter::h891cf4937b4b12df
 119:     0x7fd29f53d4a3 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h91b3a5be8f054df8
 120:     0x7fd29f533044 - rustc_span::with_source_map::h05af58868046c383
 121:     0x7fd29f53e49a - rustc_interface::interface::create_compiler_and_run::hd6bcde8a021d388e
 122:     0x7fd29f533d1f - std::sys_common::backtrace::__rust_begin_short_backtrace::h079be7663f9274fd
 123:     0x7fd29f519b6a - core::ops::function::FnOnce::call_once{{vtable.shim}}::had52829d2d210307
 124:     0x7fd29c96cb4a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8ffdf8dc1f37e360
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/alloc/src/boxed.rs:1487:9
 125:     0x7fd29c96cb4a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h74f6ec149ce6acc8
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/alloc/src/boxed.rs:1487:9
 126:     0x7fd29c96cb4a - std::sys::unix::thread::Thread::new::thread_start::h565bef3956c58d58
                               at /rustc/a2f8f6281817d430e20726128b739d3c6708561c/library/std/src/sys/unix/thread.rs:71:17
 127:     0x7fd29c87d590 - start_thread
 128:     0x7fd29c790223 - clone
 129:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (a2f8f6281 2021-01-27) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [mir_borrowck] borrow-checking `main::{constant#0}`
#1 [mir_drops_elaborated_and_const_checked] elaborating drops for `main::{constant#0}`
#2 [mir_for_ctfe] caching mir of `main::{constant#0}` for CTFE
#3 [eval_to_allocation_raw] const-evaluating + checking `main::{constant#0}`
#4 [eval_to_const_value_raw] simplifying constant for the type system `main::{constant#0}`
#5 [typeck] type-checking `main`
#6 [typeck_item_bodies] type-checking all item bodies
#7 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error; 1 warning emitted
