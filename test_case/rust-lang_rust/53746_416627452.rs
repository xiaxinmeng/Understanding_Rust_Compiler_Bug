
#0  0x00007f22c5d5ade2 in rustc::ty::fold::TypeFoldable::fold_with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5c14820 in rustc::traits::project::normalize_with_depth ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c5728d2a in rustc::traits::select::SelectionContext::match_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d2b185 in rustc::infer::InferCtxt::in_snapshot ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5723c20 in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c571b65c in rustc::traits::select::SelectionContext::select ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c5d28db5 in rustc::infer::InferCtxt::commit_if_ok ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c5c16e14 in rustc::traits::project::opt_normalize_projection_type ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c5c1544f in rustc::traits::project::normalize_projection_type ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c5c14d83 in <rustc::traits::project::AssociatedTypeNormalizer<'a, 'b, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#10 0x00007f22c5d5ad1f in rustc::ty::fold::TypeFoldable::fold_with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#11 0x00007f22c5c149af in rustc::traits::project::normalize_with_depth ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#12 0x00007f22c56da63c in core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#13 0x00007f22c56c757f in <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#14 0x00007f22c569fc51 in <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#15 0x00007f22c5729619 in rustc::traits::select::SelectionContext::impl_or_trait_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#16 0x00007f22c5728325 in rustc::traits::select::SelectionContext::vtable_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#17 0x00007f22c5d2b32b in rustc::infer::InferCtxt::in_snapshot ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#18 0x00007f22c5723c20 in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#19 0x00007f22c571b65c in rustc::traits::select::SelectionContext::select ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#20 0x00007f22c58de236 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#21 0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#22 0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#34 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#35 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#36 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#37 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#38 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#39 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#40 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#41 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#42 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#43 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#44 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#45 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#46 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#47 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#48 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#49 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#50 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#51 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#52 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#53 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#54 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#55 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#56 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#57 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#58 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#59 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#60 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#61 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#62 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#63 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#64 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#65 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#66 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#67 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#68 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#69 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#70 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#71 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#72 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#73 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#74 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#75 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#76 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#77 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#78 std::panicking::try::do_call () at libstd/panicking.rs:310
#79 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#80 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#81 std::panic::catch_unwind () at libstd/panic.rs:392
#82 std::rt::lang_start_internal () at libstd/rt.rs:58
#83 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c58dd7d6 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c58dd7d6 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#3  0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#4  0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#5  0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#6  0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#7  0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#8  0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#9  0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#29 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#30 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#31 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#32 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#33 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#34 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#37 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#44 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#45 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#46 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#47 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#48 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#49 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#50 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#54 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#57 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#58 std::panicking::try::do_call () at libstd/panicking.rs:310
#59 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#60 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#61 std::panic::catch_unwind () at libstd/panic.rs:392
#62 std::rt::lang_start_internal () at libstd/rt.rs:58
#63 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c596b3d8 in rustc::ty::context::tls::with_context_opt ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c596b3d8 in rustc::ty::context::tls::with_context_opt ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5b0a1de in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c572896a in rustc::traits::select::SelectionContext::match_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d2b185 in rustc::infer::InferCtxt::in_snapshot ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5723c20 in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c571b65c in rustc::traits::select::SelectionContext::select ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c58de236 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#9  0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#34 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#43 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#44 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#47 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#49 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#50 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#59 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#60 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#61 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#62 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#63 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#64 std::panicking::try::do_call () at libstd/panicking.rs:310
#65 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#66 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#67 std::panic::catch_unwind () at libstd/panic.rs:392
#68 std::rt::lang_start_internal () at libstd/rt.rs:58
#69 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c94dbb51 in mallocx (size=240, flags=0) at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:2278
2278	/checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c: No such file or directory.
#0  0x00007f22c94dbb51 in mallocx (size=240, flags=0) at /checkout/src/liballoc_jemalloc/../jemalloc/src/jemalloc.c:2278
#1  0x00007f22c56b2b00 in <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c58de7cd in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#5  0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#6  0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#7  0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#8  0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#9  0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#31 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#32 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#33 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#34 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#39 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#44 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#46 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#47 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#48 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#49 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#50 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#56 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#59 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#60 std::panicking::try::do_call () at libstd/panicking.rs:310
#61 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#62 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#63 std::panic::catch_unwind () at libstd/panic.rs:392
#64 std::rt::lang_start_internal () at libstd/rt.rs:58
#65 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c553d0d0 in syntax_pos::symbol::Ident::modern@plt ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c553d0d0 in syntax_pos::symbol::Ident::modern@plt ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5b66482 in rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::adjust_ident ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c5c1a455 in rustc::traits::project::assoc_ty_def ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d2902a in rustc::infer::InferCtxt::commit_if_ok ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5c16e14 in rustc::traits::project::opt_normalize_projection_type ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c5c13981 in rustc::traits::project::project_and_unify_type ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c5d296fa in rustc::infer::InferCtxt::commit_if_ok ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c58ddbdf in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#34 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#35 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#44 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#47 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#49 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#50 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#51 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#59 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#60 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#61 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#62 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#63 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#64 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#65 std::panicking::try::do_call () at libstd/panicking.rs:310
#66 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#67 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#68 std::panic::catch_unwind () at libstd/panic.rs:392
#69 std::rt::lang_start_internal () at libstd/rt.rs:58
#70 0x0000563c427a79b4 in main ()
Quit
#0  0x00007f22c553d0d0 in syntax_pos::symbol::Ident::modern@plt ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5b66482 in rustc::ty::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::adjust_ident ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c5c1a455 in rustc::traits::project::assoc_ty_def ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d2902a in rustc::infer::InferCtxt::commit_if_ok ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5c16e14 in rustc::traits::project::opt_normalize_projection_type ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c5c13981 in rustc::traits::project::project_and_unify_type ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c5d296fa in rustc::infer::InferCtxt::commit_if_ok ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c58ddbdf in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#34 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#35 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#44 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#47 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#49 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#50 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#51 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#59 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#60 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#61 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#62 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#63 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#64 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#65 std::panicking::try::do_call () at libstd/panicking.rs:310
#66 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#67 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#68 std::panic::catch_unwind () at libstd/panic.rs:392
#69 std::rt::lang_start_internal () at libstd/rt.rs:58
#70 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c596b4ab in rustc::ty::context::tls::with_context_opt ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c5637bce in <std::collections::hash::map::HashMap<K, V, S>>::insert ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c5637bce in <std::collections::hash::map::HashMap<K, V, S>>::insert ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5b6a265 in rustc::ty::context::TyCtxt::_intern_substs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c5d31f96 in rustc::infer::InferCtxt::fresh_substs_for_item ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5728c47 in rustc::traits::select::SelectionContext::match_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5d2b185 in rustc::infer::InferCtxt::in_snapshot ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c5723c20 in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c571b65c in rustc::traits::select::SelectionContext::select ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c58de236 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#34 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#42 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#44 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#47 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#49 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#50 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#59 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#60 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#61 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#62 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#63 std::panicking::try::do_call () at libstd/panicking.rs:310
#64 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#65 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#66 std::panic::catch_unwind () at libstd/panic.rs:392
#67 std::rt::lang_start_internal () at libstd/rt.rs:58
#68 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c55dbf65 in <std::collections::hash::map::HashMap<K, V, S>>::try_resize ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c55dbf65 in <std::collections::hash::map::HashMap<K, V, S>>::try_resize ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c5637a75 in <std::collections::hash::map::HashMap<K, V, S>>::insert ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c5b6a265 in rustc::ty::context::TyCtxt::_intern_substs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d5b1c4 in rustc::ty::fold::TypeFoldable::fold_with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5d6e10b in rustc::ty::structural_impls::<impl rustc::ty::fold::TypeFoldable<'tcx> for &'tcx rustc::ty::TyS<'tcx>>::super_fold_with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c573545c in <rustc::ty::subst::SubstFolder<'a, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_ty ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c5d97e85 in <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c5d5b159 in rustc::ty::fold::TypeFoldable::fold_with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c5728cb1 in rustc::traits::select::SelectionContext::match_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c5d2b185 in rustc::infer::InferCtxt::in_snapshot ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#10 0x00007f22c5723c20 in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#11 0x00007f22c571b65c in rustc::traits::select::SelectionContext::select ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#12 0x00007f22c58de236 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#13 0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#14 0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#34 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#35 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#36 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#37 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#38 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#44 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#47 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#49 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#50 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#51 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#52 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#53 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#54 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#59 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#60 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#61 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#62 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#63 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#64 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#65 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#66 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#67 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#68 std::panicking::try::do_call () at libstd/panicking.rs:310
#69 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#70 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#71 std::panic::catch_unwind () at libstd/panic.rs:392
#72 std::rt::lang_start_internal () at libstd/rt.rs:58
#73 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c56da6b3 in core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c56da6b3 in core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &'a mut F>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c56c757f in <core::iter::FlatMap<I, U, F> as core::iter::iterator::Iterator>::next ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c569fc51 in <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5729619 in rustc::traits::select::SelectionContext::impl_or_trait_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c5728325 in rustc::traits::select::SelectionContext::vtable_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#5  0x00007f22c5d2b32b in rustc::infer::InferCtxt::in_snapshot ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c5723c20 in rustc::traits::select::SelectionContext::confirm_candidate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c571b65c in rustc::traits::select::SelectionContext::select ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c58de236 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#10 0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#34 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#43 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#44 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#47 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#49 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#50 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#59 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#60 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#61 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#62 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#63 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#64 std::panicking::try::do_call () at libstd/panicking.rs:310
#65 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#66 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#67 std::panic::catch_unwind () at libstd/panic.rs:392
#68 std::rt::lang_start_internal () at libstd/rt.rs:58
#69 0x0000563c427a79b4 in main ()
Quit
Not killed.
Continuing.

Program received signal SIGINT, Interrupt.
0x00007f22c56040d1 in <std::collections::hash::map::HashMap<K, V, S>>::entry ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#0  0x00007f22c56040d1 in <std::collections::hash::map::HashMap<K, V, S>>::entry ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#1  0x00007f22c58dfaa4 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::register_obligation_at ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#2  0x00007f22c58deec4 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#3  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#4  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#5  0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#6  0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#7  0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#8  0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#9  0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#29 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#30 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#31 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#32 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#33 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#34 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#37 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#42 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#44 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#45 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#46 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#47 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#48 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#49 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#50 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#54 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#57 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#58 std::panicking::try::do_call () at libstd/panicking.rs:310
#59 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#60 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#61 std::panic::catch_unwind () at libstd/panic.rs:392
#62 std::rt::lang_start_internal () at libstd/rt.rs:58
#63 0x0000563c427a79b4 in main ()
Continuing.

Program received signal SIGABRT, Aborted.
0x00007f22c9274d7f in raise () from /usr/lib/libc.so.6
#0  0x00007f22c9274d7f in raise () from /usr/lib/libc.so.6
#1  0x00007f22c925f672 in abort () from /usr/lib/libc.so.6
#2  0x00007f22c9491ae6 in std::sys::unix::abort_internal () at libstd/sys/unix/mod.rs:166
#3  0x00007f22c94a17ed in rust_oom () at libstd/alloc.rs:138
#4  0x00007f22c9507fb6 in alloc::alloc::handle_alloc_error () at liballoc/alloc.rs:230
#5  0x00007f22c5d8d437 in <alloc::raw_vec::RawVec<T, A>>::reserve ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#6  0x00007f22c58dfbab in <rustc_data_structures::obligation_forest::ObligationForest<O>>::register_obligation_at ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#7  0x00007f22c58deec4 in <rustc_data_structures::obligation_forest::ObligationForest<O>>::process_obligations ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#8  0x00007f22c5d5641e in <rustc::traits::fulfill::FulfillmentContext<'tcx> as rustc::traits::engine::TraitEngine<'tcx>>::select_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#9  0x00007f22c7ddbb73 in rustc_typeck::check::FnCtxt::select_obligations_where_possible ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#10 0x00007f22c7dde2c6 in rustc_typeck::check::FnCtxt::expected_inputs_for_expected_output ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#11 0x00007f22c7ddc33b in rustc_typeck::check::FnCtxt::check_method_argument_types ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#12 0x00007f22c7deb517 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#13 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#14 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#15 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#16 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#17 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#18 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#19 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#20 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#21 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#22 0x00007f22c7ddfc2b in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#23 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#24 0x00007f22c7ded279 in rustc_typeck::check::FnCtxt::check_decl_local ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#25 0x00007f22c7ded7e5 in rustc_typeck::check::FnCtxt::check_block_with_expected ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#26 0x00007f22c7ddf784 in rustc_typeck::check::FnCtxt::check_expr_kind ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#27 0x00007f22c7ddf4e1 in rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#28 0x00007f22c7dde51e in rustc_typeck::check::FnCtxt::check_return_expr ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#29 0x00007f22c7dd119a in rustc_typeck::check::check_fn ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#30 0x00007f22c7e64757 in rustc::ty::context::tls::with_related_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#31 0x00007f22c7f2d069 in rustc::infer::InferCtxtBuilder::enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#32 0x00007f22c7dcfbad in rustc_typeck::check::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#33 0x00007f22c59e8233 in rustc::ty::query::__query_compute::typeck_tables_of ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#34 0x00007f22c59eb45d in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_tables_of<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#35 0x00007f22c598120c in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#36 0x00007f22c576db57 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#37 0x00007f22c592b317 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#38 0x00007f22c5a1bcd1 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#39 0x00007f22c5abe170 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#40 0x00007f22c5a3fb8d in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::ensure_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#41 0x00007f22c7dcf70f in rustc_typeck::check::typeck_item_bodies ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#42 0x00007f22c59eb439 in rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors<'tcx> for rustc::ty::query::queries::typeck_item_bodies<'tcx>>::compute () from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#43 0x00007f22c59ad715 in rustc::ty::context::tls::with_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#44 0x00007f22c57b6c39 in rustc::dep_graph::graph::DepGraph::with_task_impl ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#45 0x00007f22c5936ae4 in <rustc::ty::query::plumbing::JobOwner<'a, 'tcx, Q>>::start ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#46 0x00007f22c5a1088b in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::force_query_with_job ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#47 0x00007f22c5a9fdd7 in rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::get_query ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc-0a8ec1acd81a7e53.so
#48 0x00007f22c7f5ba3a in rustc_typeck::check_crate ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/../lib/librustc_typeck-17c1818f27d716ff.so
#49 0x00007f22c9801adb in rustc::ty::context::tls::enter_context ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#50 0x00007f22c98924ab in <std::thread::local::LocalKey<T>>::with ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#51 0x00007f22c98fe0da in rustc::ty::context::TyCtxt::create_and_enter ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#52 0x00007f22c9847dd9 in rustc_driver::driver::compile_input ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#53 0x00007f22c98e9c06 in rustc_driver::run_compiler_with_pool ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#54 0x00007f22c989ecfa in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#55 0x00007f22c98e8bf1 in rustc_driver::run_compiler ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#56 0x00007f22c989ee9d in <scoped_tls::ScopedKey<T>>::set ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#57 0x00007f22c9807912 in <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#58 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#59 0x00007f22c98e6d6d in rustc_driver::run ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#60 0x00007f22c98f48fb in rustc_driver::main ()
   from /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/../lib/librustc_driver-1e4654136c2b48b4.so
#61 0x0000563c427a7953 in std::rt::lang_start::{{closure}} ()
#62 0x00007f22c9497df3 in std::rt::lang_start_internal::{{closure}} () at libstd/rt.rs:59
#63 std::panicking::try::do_call () at libstd/panicking.rs:310
#64 0x00007f22c94d6eca in __rust_maybe_catch_panic () at libpanic_unwind/lib.rs:103
#65 0x00007f22c94ae996 in std::panicking::try () at libstd/panicking.rs:289
#66 std::panic::catch_unwind () at libstd/panic.rs:392
#67 std::rt::lang_start_internal () at libstd/rt.rs:58
#68 0x0000563c427a79b4 in main ()
Detaching from program: /home/lampam/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc, process 19407
