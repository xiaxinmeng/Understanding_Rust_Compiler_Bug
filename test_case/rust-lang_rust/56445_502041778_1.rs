
error: internal compiler error: src/librustc_mir/borrow_check/nll/universal_regions.rs:741: cannot convert `ReEarlyBound(0, 'a)` to a region vid

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:643:9
stack backtrace:
   0:     0x7ffb4696718b - backtrace::backtrace::libunwind::trace::h41595b2657878dbc
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/libunwind.rs:88
   1:     0x7ffb4696718b - backtrace::backtrace::trace_unsynchronized::h4efe511900bf5e12
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.29/src/backtrace/mod.rs:66
   2:     0x7ffb4696718b - std::sys_common::backtrace::_print::h7275b63687b21f98
                               at src/libstd/sys_common/backtrace.rs:47
   3:     0x7ffb4696718b - std::sys_common::backtrace::print::h790c12384440cac4
                               at src/libstd/sys_common/backtrace.rs:36
   4:     0x7ffb4696718b - std::panicking::default_hook::{{closure}}::he6cba1bdf748f1c3
                               at src/libstd/panicking.rs:198
   5:     0x7ffb46966e8c - std::panicking::default_hook::h1296d9a476e7a9c2
                               at src/libstd/panicking.rs:212
   6:     0x7ffb446bb3f1 - rustc::util::common::panic_hook::ha2e96a0b919e5a1a
   7:     0x7ffb469679e9 - std::panicking::rust_panic_with_hook::h8d2408723e9a2bd4
                               at src/libstd/panicking.rs:479
   8:     0x7ffb433253ad - std::panicking::begin_panic::h234a5167f045a72d
   9:     0x7ffb4334544f - rustc_errors::Handler::bug::h3246723a0b2031e4
  10:     0x7ffb44420643 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::hb8ea6d6e88ca54f4
  11:     0x7ffb444125da - rustc::ty::context::tls::with_opt::{{closure}}::h9f1dda041ea3d8ca
  12:     0x7ffb444124f5 - rustc::ty::context::tls::with_context_opt::h84ca529c2ea4a682
  13:     0x7ffb44412587 - rustc::ty::context::tls::with_opt::h875054e282c744c1
  14:     0x7ffb44420558 - rustc::util::bug::opt_span_bug_fmt::h24d0b9fdc514170c
  15:     0x7ffb444204c2 - rustc::util::bug::bug_fmt::hc0ddc16f6cc56100
  16:     0x7ffb459c72e2 - rustc_mir::borrow_check::nll::universal_regions::UniversalRegionIndices::to_region_vid::{{closure}}::h8f1191649fe952ee
  17:     0x7ffb45a9f866 - rustc_mir::borrow_check::nll::type_check::constraint_conversion::ConstraintConversion::convert_all::hff38c8376a2f82d5
  18:     0x7ffb45c4bd1c - rustc_mir::borrow_check::nll::type_check::TypeChecker::fully_perform_op::h78748fd6d92e387d
  19:     0x7ffb45c46a4b - rustc_mir::borrow_check::nll::type_check::type_check::hea7e4d9e23e5ba76
  20:     0x7ffb458bcd6f - rustc_mir::borrow_check::nll::compute_regions::h80ed7c7d15df620d
  21:     0x7ffb45976f83 - rustc_mir::borrow_check::do_mir_borrowck::hd7e464220fc5eca7
  22:     0x7ffb45cfe746 - rustc::ty::context::GlobalCtxt::enter_local::heaa8c87e8f8669e9
  23:     0x7ffb459765da - rustc_mir::borrow_check::mir_borrowck::hae96041a19836cd8
  24:     0x7ffb45954cb7 - rustc::ty::query::__query_compute::mir_borrowck::hc7e574fc48878e80
  25:     0x7ffb45c735a5 - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::mir_borrowck>::compute::h4a666fc8e52be0d8
  26:     0x7ffb459ed121 - rustc::dep_graph::graph::DepGraph::with_task_impl::hd0890e01d4a2bb3e
  27:     0x7ffb45cf234c - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hf2f4510df37bed60
  28:     0x7ffb45c73d25 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::ensure_query::hf9d9f7fd3e34fbdf
  29:     0x7ffb458d1164 - rustc_mir::transform::optimized_mir::hcdb77108b4828931
  30:     0x7ffb4595507d - rustc::ty::query::__query_compute::optimized_mir::h2402952c578cb24c
  31:     0x7ffb45c7365d - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::optimized_mir>::compute::ha863b92e11001b35
  32:     0x7ffb459f1497 - rustc::dep_graph::graph::DepGraph::with_task_impl::hf4b7b9c362d667eb
  33:     0x7ffb45c915ad - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h2ec66e6851178136
  34:     0x7ffb45aafff4 - rustc_mir::interpret::eval_context::InterpretCx<M>::load_mir::haeda8af7b0b03ee9
  35:     0x7ffb45c65bee - rustc_mir::const_eval::const_eval_raw_provider::hf339294a1d76412a
  36:     0x7ffb459553b1 - rustc::ty::query::__query_compute::const_eval_raw::hf79a48c673afa965
  37:     0x7ffb45c7370c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval_raw>::compute::ha61cce162d227846
  38:     0x7ffb459ea072 - rustc::dep_graph::graph::DepGraph::with_task_impl::ha02743b6acac21ff
  39:     0x7ffb45c9dd2b - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h40523fc999af8bae
  40:     0x7ffb45c64e79 - rustc_mir::const_eval::const_eval_provider::h8ecd0d4acacbded6
  41:     0x7ffb442b17f8 - rustc::ty::query::__query_compute::const_eval::h1119198b82c92e58
  42:     0x7ffb4464f7bc - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::const_eval>::compute::hbc876f0f390d6c25
  43:     0x7ffb4420173a - rustc::dep_graph::graph::DepGraph::with_task_impl::h35ad45147a002223
  44:     0x7ffb444796ab - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h0caac3b3a703e7cc
  45:     0x7ffb446a81e9 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_const::h09fb34e6e6e164e6
  46:     0x7ffb44415311 - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with::h13b491c0c3eac0bf
  47:     0x7ffb446a7975 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h233440fce128939b
  48:     0x7ffb442c0659 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h590f39025e063bfa
  49:     0x7ffb443fa94c - rustc::ty::fold::TypeFoldable::fold_with::hb694e0d0547dc004
  50:     0x7ffb444154bd - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with::h13b491c0c3eac0bf
  51:     0x7ffb446a7975 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h233440fce128939b
  52:     0x7ffb442c0659 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h590f39025e063bfa
  53:     0x7ffb443fa94c - rustc::ty::fold::TypeFoldable::fold_with::hb694e0d0547dc004
  54:     0x7ffb444152dd - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with::h13b491c0c3eac0bf
  55:     0x7ffb446a7975 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h233440fce128939b
  56:     0x7ffb41701954 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::he9a71a7d2a8e41a4
  57:     0x7ffb4155e9bc - rustc::ty::fold::TypeFoldable::fold_with::hb877186dcdde4a8a
  58:     0x7ffb4169fc64 - rustc::traits::project::normalize::hd55b9356461a2c54
  59:     0x7ffb414c0e58 - rustc_typeck::check::FnCtxt::normalize_associated_types_in::hd4ed4b1269b53182
  60:     0x7ffb4163c92d - rustc::ty::context::GlobalCtxt::enter_local::ha314227e251792e5
  61:     0x7ffb41713dd0 - rustc_typeck::check::wfcheck::check_associated_item::h7a5ca14b80bab58e
  62:     0x7ffb414e147e - rustc::ty::query::__query_compute::check_impl_item_well_formed::hca2766a1fa632057
  63:     0x7ffb415ab05d - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_impl_item_well_formed>::compute::h04a3209f07ea56a6
  64:     0x7ffb4150f69b - rustc::dep_graph::graph::DepGraph::with_task_impl::h4963b94af07ea903
  65:     0x7ffb41620270 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::he56dc8058550bfc1
  66:     0x7ffb41716e67 - <rustc_typeck::check::wfcheck::CheckTypeWellFormedVisitor as rustc::hir::itemlikevisit::ParItemLikeVisitor>::visit_impl_item::h77cafc90c30a5ec3
  67:     0x7ffb469788da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  68:     0x7ffb41520dc9 - rustc_data_structures::sync::par_for_each_in::h550ca05f0ca541fa
  69:     0x7ffb469788da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  70:     0x7ffb416d8699 - rustc::hir::Crate::par_visit_all_item_likes::h2fa6afd0da02bfad
  71:     0x7ffb414e4c5d - rustc::util::common::time::he5d849132f13a9ee
  72:     0x7ffb416f8e08 - rustc_typeck::check_crate::h261fa85c0eefa2e6
  73:     0x7ffb46346855 - rustc_interface::passes::analysis::hd7064ffdb52b6f45
  74:     0x7ffb46cc71e6 - rustc::ty::query::__query_compute::analysis::hc63e7bbcf2ef51b7
  75:     0x7ffb46c32462 - rustc::dep_graph::graph::DepGraph::with_task_impl::h6ecc24de8ad82664
  76:     0x7ffb46c2a0ad - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h3d879e5498479529
  77:     0x7ffb46c3b606 - rustc::ty::context::tls::enter_global::h429817b273ef3027
  78:     0x7ffb46c5a407 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h0a59fddc00963551
  79:     0x7ffb463a3b05 - rustc_interface::passes::create_global_ctxt::{{closure}}::h7a059b6cf199597d
  80:     0x7ffb46c5b7aa - rustc_interface::interface::run_compiler_in_existing_thread_pool::h35eddbfcfbbe0eb4
  81:     0x7ffb46ccd5f6 - std::thread::local::LocalKey<T>::with::hea85159711765771
  82:     0x7ffb46c65fb5 - scoped_tls::ScopedKey<T>::set::he08d6211f459724f
  83:     0x7ffb46c98ad4 - syntax::with_globals::h2c63be024f0c1f38
  84:     0x7ffb46cb789d - std::sys_common::backtrace::__rust_begin_short_backtrace::hfc7fb053a9ed90ec
  85:     0x7ffb469788da - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:82
  86:     0x7ffb46c436d9 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h0cd7d8fc1b0a39a3
  87:     0x7ffb4694aeef - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::he71721d2d956d451
                               at /rustc/0e4a56b4b04ea98bb16caada30cb2418dd06e250/src/liballoc/boxed.rs:746
  88:     0x7ffb469775b0 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::he520045b8d28ce5c
                               at /rustc/0e4a56b4b04ea98bb16caada30cb2418dd06e250/src/liballoc/boxed.rs:746
  89:     0x7ffb469775b0 - std::sys_common::thread::start_thread::h2e98d1272dc6d74b
                               at src/libstd/sys_common/thread.rs:13
  90:     0x7ffb469775b0 - std::sys::unix::thread::Thread::new::thread_start::h18485805666ccd3c
                               at src/libstd/sys/unix/thread.rs:79
  91:     0x7ffb468bf58e - start_thread
                               at /usr/src/debug/glibc-2.28-60-g4d7af7815a/nptl/pthread_create.c:486
  92:     0x7ffb467de6a3 - clone
  93:                0x0 - <unknown>
query stack during panic:
#0 [mir_borrowck] processing `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#1 [optimized_mir] processing `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
   --> src/fat.rs:276:56
    |
276 |     fn lfn_contents(&self) -> Option<(bool, u8, [char; Self::LFN_FRAGMENT_LEN])> {
    |                                                        ^^^^^^^^^^^^^^^^^^^^^^
#2 [const_eval_raw] const-evaluating `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#3 [const_eval] const-evaluating + checking `fat::OnDiskDirEntry::<'a>::lfn_contents::{{constant}}#0`
#4 [check_impl_item_well_formed] processing `fat::OnDiskDirEntry::<'a>::lfn_contents`
#5 [analysis] running analysis passes on this crate
end of query stack
error: aborting due to previous error
