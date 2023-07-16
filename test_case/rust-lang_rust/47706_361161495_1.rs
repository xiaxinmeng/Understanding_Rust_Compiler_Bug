
thread 'rustc' panicked at 'non-FnLike node found: NodeTraitItem(TraitItem { id: NodeId(6), name: f, hir_id: HirId { owner: DefIndex(0:4), local_id: ItemLocalId(0) }, attrs: [], generics: Generics { params: [], where_clause: WhereClause { id: NodeId(7), predicates: [] }, span: src\main.rs:1:1: 1:1 }, node: Method(MethodSig { unsafety: Normal, constness: NotConst, abi: Rust, decl: FnDecl { inputs: [type(&Self), type(())], output: DefaultReturn(src\main.rs:2:24: 2:24), variadic: false, has_implicit_self: true } }, Provided(BodyId { node_id: NodeId(25) })), span: src\main.rs:2:5: 4:6 })', librustc\traits\error_reporting.rs:846:18
stack backtrace:
   0:     0x7ffedcc846dd - std::rt::lang_start_internal::h6a4891cd6ad9df3d
   1:     0x7ffedcc872ef - std::fs::FileType::is_dir::hf1c2aa54924656a2
   2:     0x7ffedcc9082f - std::panicking::Location::column::hdc2cb9a904b38b88
   3:     0x7ffedcc904d5 - std::panicking::Location::column::hdc2cb9a904b38b88
   4:     0x7ffedcc90dfc - std::panicking::rust_panic_with_hook::h722955029fa85c2e
   5:     0x7ffedcc90c00 - std::panicking::begin_panic_fmt::hdf8109b479eeb360
   6:     0x7ffedcc90b51 - std::panicking::begin_panic_fmt::hdf8109b479eeb360
   7:     0x7ffeccc0b739 - rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_selection_error::hc013179b5351d84d
   8:     0x7ffeccc00069 - rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_errors::h4433be3ed2c84ec8
   9:     0x7ffee2e425ca - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  10:     0x7ffee2e4387a - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  11:     0x7ffee2e43016 - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  12:     0x7ffee2e52da9 - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  13:     0x7ffee2e46a06 - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  14:     0x7ffee2e5930e - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  15:     0x7ffee2e46fb8 - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  16:     0x7ffee2e46a06 - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  17:     0x7ffee2e4595f - <rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx> as rustc_typeck::astconv::AstConv<'gcx, 'tcx>>::record_ty::h9b01acacbcbaff91
  18:     0x7ffee2e3678d - <rustc_typeck::check::GatherLocalsVisitor<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_pat::hb7b0709391a3fd8d
  19:     0x7ffee2ebe159 - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::h0e25c0a16f74e38c
  20:     0x7ffee2e76746 - <rustc_typeck::check::Diverges as core::fmt::Debug>::fmt::h1c65dfb30c7b3c51
  21:     0x7ffee2e350c2 - <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::hf39656246afa540d
  22:     0x7ffecca7c126 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::hba2ab45518988f23
  23:     0x7ffecd074c9a - rustc::dep_graph::graph::DepGraph::assert_ignored::h3d788a943cdbafc4
  24:     0x7ffeccb5c046 - rustc::ty::maps::<impl rustc::ty::maps::queries::instance_def_size_estimate<'tcx>>::try_get::h23404c885dbf382f
  25:     0x7ffeccd3fe27 - <rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx> as rustc::ty::layout::HasTyCtxt<'gcx>>::tcx::h93da4ee47aa7f377
  26:     0x7ffecca7c1d6 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::hba2ab45518988f23
  27:     0x7ffecca7c9ba - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get::h96e5020f96e0297c
  28:     0x7ffeccd95b6f - rustc::ty::maps::TyCtxtAt::typeck_tables_of::h4054dbb665aa1f74
  29:     0x7ffecca7c09c - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::hba2ab45518988f23
  30:     0x7ffee2f3af66 - <rustc_typeck::variance::terms::InferredIndex as core::fmt::Debug>::fmt::h774c8f2cd3b8f384
  31:     0x7ffee2e34c4d - <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::hf39656246afa540d
  32:     0x7ffecd0936ae - rustc::dep_graph::graph::DepGraph::assert_ignored::h3d788a943cdbafc4
  33:     0x7ffeccb646b7 - rustc::ty::maps::<impl rustc::ty::maps::queries::instance_def_size_estimate<'tcx>>::try_get::h23404c885dbf382f
  34:     0x7ffeccd0ea17 - <rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx> as rustc::ty::layout::HasTyCtxt<'gcx>>::tcx::h93da4ee47aa7f377
  35:     0x7ffecca7ab8a - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::ensure::h5a89529d361a30e7
  36:     0x7ffecca7b24f - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get::hcd0c852dadf7529b
  37:     0x7ffeccd95a57 - rustc::ty::maps::TyCtxtAt::typeck_item_bodies::h58611989c649d001
  38:     0x7ffeccd921a2 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies::h585d4c02703c78ad
  39:     0x7ffee2f6d5c3 - rustc_typeck::check_crate::h05100ae6a011b747
  40:     0x7ffee6cfe07d - <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt::hae9e730b8990bdb8
  41:     0x7ffee6d010f0 - <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt::hae9e730b8990bdb8
  42:     0x7ffee6d45fb0 - <rustc_driver::derive_registrar::Finder as rustc::hir::itemlikevisit::ItemLikeVisitor<'v>>::visit_item::h34e7b02473305531
  43:     0x7ffee6c80397 - rustc_driver::driver::compile_input::hc6aa0b074e7a66c3
  44:     0x7ffee6d5ee9d - rustc_driver::run_compiler::hf2ae78daa657a043
  45:     0x7ffee6c3ee2a - <unknown>
  46:     0x7ffedccbd271 - _rust_maybe_catch_panic
  47:     0x7ffee6cb490a - <rustc_driver::pretty::UserIdentifiedItem as core::fmt::Debug>::fmt::hae9e730b8990bdb8
  48:     0x7ffedccbc1cb - <<std::sys_common::remutex::ReentrantMutex<T> as core::fmt::Debug>::fmt::LockedPlaceholder as core::fmt::Debug>::fmt::h6a90c306f5c106da
  49:     0x7ffedcc8ce36 - std::sys::windows::thread::Thread::new::h62f006ffe7a96a67
  50:     0x7fff1b1a1fe3 - BaseThreadInitThunk
  51:     0x7fff1d5aefb0 - RtlUserThreadStart

error: internal compiler error: unexpected panic
