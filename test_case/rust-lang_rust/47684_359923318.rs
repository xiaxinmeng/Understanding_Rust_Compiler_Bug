
stack backtrace:
   0:     0x7ffc74fb4e79 - _rdl_grow_in_place
   1:     0x7ffc74fc6087 - std::panicking::Location::column::h28b40ec705c5e546
   2:     0x7ffc74fc5cec - std::panicking::Location::column::h28b40ec705c5e546
   3:     0x7ffc74fc66f1 - std::panicking::rust_panic_with_hook::h73a8ba19426dc4bc
   4:     0x7ffc74fc6560 - std::panicking::begin_panic_fmt::h30275337d16f3133
   5:     0x7ffc74fc6441 - std::panicking::begin_panic_fmt::h30275337d16f3133
   6:     0x7ffc74fc63b4 - rust_begin_unwind
   7:     0x7ffc74fd606e - core::panicking::panic_fmt::h2add2073c03e1b21
   8:     0x7ffc74fd60ea - core::option::expect_failed::h62aa32a627994376
   9:     0x7ffc74fb3eb1 - std::time::Instant::duration_since::hf205834512ea3c88
  10:     0x7ffc74fb3f0d - std::time::Instant::elapsed::he1bf1266b3bc527e
  11:     0x7ffc6ebc70c1 - <rustc_trans::abi::attr_impl::ArgAttribute as core::fmt::UpperHex>::fmt::h6b1a1f0f6eb32310
  12:     0x7ffc6ebdfd2e - <rustc_trans::builder::Builder<'a, 'tcx> as core::ops::drop::Drop>::drop::hf57064746f371aa4
  13:     0x7ffc6deaf5bc - rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::ensure::hb6f171946daebc1f
  14:     0x7ffc6e05873c - rustc::dep_graph::graph::DepGraph::in_ignore::hf1d621baac1d7bf2
  15:     0x7ffc6dd65b5a - rustc::util::ppaux::<impl core::fmt::Debug for rustc::ty::Predicate<'tcx>>::fmt::h676d3496cb3708f4
  16:     0x7ffc6e1fece4 - <rustc::ty::_match::Match<'a, 'gcx, 'tcx> as rustc::ty::relate::TypeRelation<'a, 'gcx, 'tcx>>::tys::haf26cce42371d08b
  17:     0x7ffc6deaf673 - rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::ensure::hb6f171946daebc1f
  18:     0x7ffc6deb00e8 - rustc::ty::maps::<impl rustc::ty::maps::queries::symbol_name<'tcx>>::try_get::haa9a36edf6ce9e19
  19:     0x7ffc6e015eb9 - rustc::ty::maps::TyCtxtAt::symbol_name::hf0fbafab8c3ef5d4
  20:     0x7ffc6e24bb8e - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::symbol_name::h0f11095c026f7986
  21:     0x7ffc6eb0b8ad - <rustc_trans::symbol_names_test::SymbolNamesTest<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_impl_item::h0378c6a841615dfc
  22:     0x7ffc6eae818a - <unknown>
  23:     0x7ffc6eb0cd55 - <rustc_trans::symbol_names_test::SymbolNamesTest<'a, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_impl_item::h0378c6a841615dfc
  24:     0x7ffc6df24fff - rustc::ty::maps::<impl rustc::ty::maps::queries::exported_symbols<'tcx>>::ensure::h3028cd1cb66c60e8
  25:     0x7ffc6e07a14b - rustc::dep_graph::graph::DepGraph::in_ignore::hf1d621baac1d7bf2
  26:     0x7ffc6dd7476f - rustc::util::ppaux::<impl core::fmt::Debug for rustc::ty::Predicate<'tcx>>::fmt::h676d3496cb3708f4
  27:     0x7ffc6e20c2a4 - <rustc::ty::_match::Match<'a, 'gcx, 'tcx> as rustc::ty::relate::TypeRelation<'a, 'gcx, 'tcx>>::tys::haf26cce42371d08b
  28:     0x7ffc6df2509a - rustc::ty::maps::<impl rustc::ty::maps::queries::exported_symbols<'tcx>>::ensure::h3028cd1cb66c60e8
  29:     0x7ffc6df258aa - rustc::ty::maps::<impl rustc::ty::maps::queries::exported_symbols<'tcx>>::try_get::h7e5aadbe9d446c86
  30:     0x7ffc6e01b55e - rustc::ty::maps::TyCtxtAt::exported_symbols::h301ae61ce3d093cb
  31:     0x7ffc6e24cb36 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::exported_symbols::hcdf79fa9e881e8b4
  32:     0x7ffc6eb9e836 - rustc_trans::back::write::start_async_translation::hf345ba6a81ddda07
  33:     0x7ffc6ebe6d3d - rustc_trans::base::trans_crate::ha8d10b8471f7723a
  34:     0x7ffc6eb8b8c7 - <rustc_trans::LlvmTransCrate as rustc_trans_utils::trans_crate::TransCrate>::trans_crate::h7a21b6007b1d2e8f
  35:     0x7ffc7541cf98 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h970bab471a268779
  36:     0x7ffc753b9c86 - rustc_driver::driver::default_provide_extern::h36c0e3cf673f237b
  37:     0x7ffc7540a96d - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h970bab471a268779
  38:     0x7ffc7540751e - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h970bab471a268779
  39:     0x7ffc75406a63 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h970bab471a268779
  40:     0x7ffc7536731f - <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt::h75102f2bc4758c9c
  41:     0x7ffc753b2a2b - rustc_driver::driver::compile_input::hfb39c521f477ee8d
  42:     0x7ffc753f6b55 - rustc_driver::run_compiler::h334a80a4b6e644f4
  43:     0x7ffc75350d9d - <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt::h75102f2bc4758c9c
  44:     0x7ffc74fc9f51 - _rust_maybe_catch_panic
  45:     0x7ffc753e9a11 - <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_impl_item::h73b2f332badc5ded
  46:     0x7ffc74fc415b - std::sys::imp::thread::Thread::new::hc076c74f7dafca1e
  47:     0x7ffc8ae78363 - BaseThreadInitThunk
