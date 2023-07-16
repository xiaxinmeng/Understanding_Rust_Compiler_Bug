text
error: internal compiler error: src/librustc/traits/select.rs:2933: Where clause `Binder(<base::default_allocator::DefaultAllocator as base::allocator::Allocator<N, R, C>>)` was applicable to `Obligation(predicate=Binder(TraitPredicate(<base::default_allocator::DefaultAllocator as base::allocator::Allocator<_, _, _>>)), depth=1)` but now is not

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
stack backtrace:
   0:     0x7f66dc54be94 - backtrace::backtrace::libunwind::trace::h3af7d8f0cad3d88e
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
   1:     0x7f66dc54be94 - backtrace::backtrace::trace_unsynchronized::hcec51d2fabb72b74
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/mod.rs:66
   2:     0x7f66dc54be94 - std::sys_common::backtrace::_print_fmt::hcd1780c700b0133c
                               at src/libstd/sys_common/backtrace.rs:84
   3:     0x7f66dc54be94 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he6a1ce49d42ba791
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f66dc5845ac - core::fmt::write::hd8162e8147f9c30a
                               at src/libcore/fmt/mod.rs:1025
   5:     0x7f66dc5401c7 - std::io::Write::write_fmt::hafa84d2501c82b1a
                               at src/libstd/io/mod.rs:1426
   6:     0x7f66dc55033e - std::sys_common::backtrace::_print::hb630961a44ec5c08
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f66dc55033e - std::sys_common::backtrace::print::h8721ef57e620a95d
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f66dc55033e - std::panicking::default_hook::{{closure}}::h830d8b6321244162
                               at src/libstd/panicking.rs:193
   9:     0x7f66dc550031 - std::panicking::default_hook::h483ff228a4236089
                               at src/libstd/panicking.rs:210
  10:     0x7f66dcad72c3 - rustc_driver::report_ice::h6c459048104da02e
  11:     0x7f66dc550af0 - std::panicking::rust_panic_with_hook::hf9b4de76bea4ec49
                               at src/libstd/panicking.rs:475
  12:     0x7f66decb4a6d - std::panicking::begin_panic::h11dacb2a580cfc13
  13:     0x7f66dece9770 - rustc_errors::HandlerInner::bug::h6630f664722f00a9
  14:     0x7f66dece84da - rustc_errors::Handler::bug::hcd6102dbbb94a270
  15:     0x7f66de7fbb02 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h958ad6a6d585dbab
  16:     0x7f66de7f26c3 - rustc::ty::context::tls::with_opt::{{closure}}::hffb1b73c4cf34c3e
  17:     0x7f66de7f2618 - rustc::ty::context::tls::with_opt::h2f9609d99125a138
  18:     0x7f66de7fba18 - rustc::util::bug::opt_span_bug_fmt::h341a618023e54cd9
  19:     0x7f66de7fb982 - rustc::util::bug::bug_fmt::hee90e2fc57db382e
  20:     0x7f66de77bcdc - rustc::traits::select::SelectionContext::confirm_candidate::h9a41ea5dfb60b800
  21:     0x7f66de76ef1d - rustc::traits::select::SelectionContext::select::h3fcfb6a38fb36eea
  22:     0x7f66de3e76ed - rustc::infer::InferCtxt::commit_if_ok::h989729c049a84245
  23:     0x7f66de673b10 - rustc::traits::project::opt_normalize_projection_type::habfea972e4de8462
  24:     0x7f66de672066 - rustc::traits::project::normalize_projection_type::h1513d65033c29310
  25:     0x7f66de671dcf - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h8c90ff7aa86152db
  26:     0x7f66de673070 - rustc::traits::project::opt_normalize_projection_type::habfea972e4de8462
  27:     0x7f66de672066 - rustc::traits::project::normalize_projection_type::h1513d65033c29310
  28:     0x7f66de671dcf - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h8c90ff7aa86152db
  29:     0x7f66de434879 - <smallvec::SmallVec<A> as core::iter::traits::collect::FromIterator<<A as smallvec::Array>::Item>>::from_iter::h5c195ee5fa843a52
  30:     0x7f66de597ed0 - rustc::ty::fold::TypeFoldable::fold_with::h494171898eb4d2b9
  31:     0x7f66de5a8acf - rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable for &rustc::ty::TyS>::super_fold_with::h90eb496dc4436138
  32:     0x7f66de671c06 - <rustc::traits::project::AssocTypeNormalizer as rustc::ty::fold::TypeFolder>::fold_ty::h8c90ff7aa86152db
  33:     0x7f66dd4d4b46 - rustc::traits::project::normalize::h78c1fa95b6f240f0
  34:     0x7f66dd3ed080 - rustc_typeck::check::method::probe::ProbeContext::assemble_inherent_impl_probe::h494ffc1498e51dc6
  35:     0x7f66dd3ebf5e - rustc_typeck::check::method::probe::ProbeContext::assemble_inherent_candidates::h56005b43ad7cc91b
  36:     0x7f66dd3a615e - rustc::infer::InferCtxt::probe::hd652857fa9508021
  37:     0x7f66dd2ed944 - rustc_typeck::check::method::probe::<impl rustc_typeck::check::FnCtxt>::probe_for_return_type::ha7aedd7ef8784adb
  38:     0x7f66dd31d8d5 - rustc_typeck::check::FnCtxt::suggest_ref_or_into::hcd8cb08de7ed55de
  39:     0x7f66dd2dd169 - rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt>::emit_coerce_suggestions::hb99f7ac0a38f7858
  40:     0x7f66dd349d7e - rustc_typeck::check::coercion::CoerceMany<E>::coerce_inner::h728474070fc6e1e1
  41:     0x7f66dd31c06a - rustc_typeck::check::FnCtxt::check_block_with_expected::hbc8145eb75063594
  42:     0x7f66dd2e2e74 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_kind::hdfa545abcd1f4f9a
  43:     0x7f66dd2e220b - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation_and_needs::h394f3bf285701128
  44:     0x7f66dd2ecafb - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_return_expr::he6756fd8b7df94fd
  45:     0x7f66dd30782f - rustc_typeck::check::check_fn::h5fcb7eecce1a7cc8
  46:     0x7f66dd4c1261 - rustc::ty::context::tls::with_context::{{closure}}::hf245d6ba2539ac8e
  47:     0x7f66dd3065f2 - rustc_typeck::check::typeck_tables_of::h0dff56b11c3b1dc9
  48:     0x7f66dd50528a - rustc::ty::query::__query_compute::typeck_tables_of::h6fbd03e165500945
  49:     0x7f66dd42a44b - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::typeck_tables_of>::compute::hee42e90da8c65856
  50:     0x7f66dd3c92a2 - rustc::dep_graph::graph::DepGraph::with_task_impl::he626bbd887e9bd71
  51:     0x7f66dd45abb5 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h40bbc3f422f79644
  52:     0x7f66dd426a7f - rustc::ty::<impl rustc::ty::context::TyCtxt>::par_body_owners::hfe2b7e757907435a
  53:     0x7f66dd305f0d - rustc_typeck::check::typeck_item_bodies::hc448723e529dff79
  54:     0x7f66dd5056d5 - rustc::ty::query::__query_compute::typeck_item_bodies::h3b3cd5a8a42a51ab
  55:     0x7f66dd3b9154 - rustc::dep_graph::graph::DepGraph::with_task_impl::h31afe2e2781df8eb
  56:     0x7f66dd4807cb - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h8ca4233134fb396f
  57:     0x7f66dd4d421e - rustc::util::common::time::hcf4211da7181e73a
  58:     0x7f66dd4f8e0d - rustc_typeck::check_crate::hd9bed871ae68fcef
  59:     0x7f66dcc41174 - rustc_interface::passes::analysis::hf458fdc494d55942
  60:     0x7f66dca6abb1 - rustc::ty::query::__query_compute::analysis::hb40ee54599ba8f81
  61:     0x7f66dcab87a9 - rustc::dep_graph::graph::DepGraph::with_task_impl::hfa107e8e634564ca
  62:     0x7f66dcae276d - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hef9bfd321d7b5f9f
  63:     0x7f66dca886c5 - rustc::ty::context::tls::enter_global::he916f073b76f98f8
  64:     0x7f66dca9e094 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h809fe2cdffb27eea
  65:     0x7f66dca6e892 - std::thread::local::LocalKey<T>::with::hdd7a4f7dd5973d77
  66:     0x7f66dca67dde - scoped_tls::ScopedKey<T>::set::h1a7050e3b333ae25
  67:     0x7f66dcae35e4 - syntax::with_globals::h5728c0a6d54600d2
  68:     0x7f66dca68a40 - std::sys_common::backtrace::__rust_begin_short_backtrace::hd043e51745c2002a
  69:     0x7f66dc5615ca - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:78
  70:     0x7f66dca7f079 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h3e9570487439b1c8
  71:     0x7f66dc5321cf - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h66f513834129f65f
                               at /rustc/c8ea4ace9213ae045123fdfeb59d1ac887656d31/src/liballoc/boxed.rs:1022
  72:     0x7f66dc55fff0 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hae4f8b730c770e10
                               at /rustc/c8ea4ace9213ae045123fdfeb59d1ac887656d31/src/liballoc/boxed.rs:1022
  73:     0x7f66dc55fff0 - std::sys_common::thread::start_thread::hade1fccd694a6ed2
                               at src/libstd/sys_common/thread.rs:13
  74:     0x7f66dc55fff0 - std::sys::unix::thread::Thread::new::thread_start::hcbccf366a29cb6fb
                               at src/libstd/sys/unix/thread.rs:80
  75:     0x7f66dc49d669 - start_thread
  76:     0x7f66dc3b2323 - clone
  77:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (c8ea4ace9 2019-12-14) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `base::conversion::<impl std::convert::From<base::matrix::Matrix<N, R, C, base::matrix_slice::SliceStorage<'a, N, R, C, RStride, CStride>>> for base::matrix::Matrix<N, R, C, base::array_storage::ArrayStorage<N, R, C>>>::from`
#1 [typeck_item_bodies] type-checking all item bodies
#2 [analysis] running analysis passes on this crate
end of query stack
