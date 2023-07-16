
   Compiling rngerr v0.1.0 (file:///D:/rustproj/rngerr)
     Running `rustc --crate-name rngerr src\main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=90b29a1b474d9a34 -C extra-filename=-90b29a1b474d9a34 --out-dir D:\rustproj\rngerr\target\debug\deps -C incremental=D:\rustproj\rngerr\target\debug\incremental -L dependency=D:\rustproj\rngerr\target\debug\deps --extern rand=D:\rustproj\rngerr\target\debug\deps\librand-39fb58d4902b5813.rlib`
error[E0432]: unresolved import `rand::Thread_rng`
 --> src\main.rs:3:16
  |
3 | use rand::{Rng,Thread_rng};
  |                ^^^^^^^^^^ no `Thread_rng` in the root. Did you mean to use `thread_rng`?

error[E0425]: cannot find function `thread_rng` in this scope
 --> src\main.rs:6:34
  |
6 |     println!("Hello, world! {}",*thread_rng().choose(&[0, 1, 2, 3]).unwrap());
  |                                  ^^^^^^^^^^ not found in this scope
help: possible candidate is found in another module, you can import it into scope
  |
3 | use rand::thread_rng;
  |

warning: unused import: `Thread_rng`
 --> src\main.rs:3:16
  |
3 | use rand::{Rng,Thread_rng};
  |                ^^^^^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error: internal compiler error: librustc\ich\impls_ty.rs:907: ty::TypeVariants::hash_stable() - Unexpected variant TyInfer(?0).

thread 'rustc' panicked at 'Box<Any>', librustc_errors\lib.rs:540:9
stack backtrace:
   0:     0x7ff9c9a361fa - _rdl_shrink_in_place
   1:     0x7ff9c9a39e52 - std::panicking::take_hook::h7e6bff1200b4d3f9
   2:     0x7ff9c9a39ab3 - std::panicking::take_hook::h7e6bff1200b4d3f9
   3:     0x7ff9c8614add - <unknown>
   4:     0x7ff9c9a3a456 - std::panicking::rust_panic_with_hook::h17a5bc6b5077376c
   5:     0x7ff9ce391a5e - <unknown>
   6:     0x7ff9ce3b5da8 - rustc_errors::Handler::bug::hb8b71f117d2421da
   7:     0x7ff9c88f45de - rustc::session::bug_fmt::h12a1661df948d163
   8:     0x7ff9c89c94cb - rustc::ty::context::tls::span_debug::h4d0feb810a227a29
   9:     0x7ff9c89c929e - rustc::ty::context::tls::span_debug::h4d0feb810a227a29
  10:     0x7ff9c86128fe - <unknown>
  11:     0x7ff9c8612859 - <unknown>
  12:     0x7ff9c89c9239 - rustc::ty::context::tls::span_debug::h4d0feb810a227a29
  13:     0x7ff9c89c9471 - rustc::ty::context::tls::span_debug::h4d0feb810a227a29
  14:     0x7ff9c88f44ec - rustc::session::bug_fmt::h12a1661df948d163
  15:     0x7ff9c88f4452 - rustc::session::bug_fmt::h12a1661df948d163
  16:     0x7ff9c88135d3 - rustc::ich::impls_hir::<impl rustc_data_structures::stable_hasher::ToStableHashKey<rustc::ich::hcx::StableHashingContext<'a>> for rustc::hir::TraitCandidate>::to_stable_hash_key::h9eed4c898167a695
  17:     0x7ff9c88131af - rustc::ich::impls_hir::<impl rustc_data_structures::stable_hasher::ToStableHashKey<rustc::ich::hcx::StableHashingContext<'a>> for rustc::hir::TraitCandidate>::to_stable_hash_key::h9eed4c898167a695
  18:     0x7ff9c89eaa2c - rustc::dep_graph::dep_node::DepNode::new::haaa5ab305ec2c44d
  19:     0x7ff9c8bd5f70 - rustc::ty::maps::<impl rustc::ty::maps::queries::dropck_outlives<'tcx>>::try_get::h213e7ce9ff55095c
  20:     0x7ff9c8bef4e6 - rustc::ty::maps::TyCtxtAt::dropck_outlives::h78ac1228027cebbb
  21:     0x7ff9c894f7f1 - rustc::traits::query::dropck_outlives::<impl rustc::infer::at::At<'cx, 'gcx, 'tcx>>::dropck_outlives::hd82e3d06092885ba
  22:     0x7ff9cdd1cc7c - <rustc_typeck::check::autoderef::Autoderef<'a, 'gcx, 'tcx> as core::iter::iterator::Iterator>::next::h9e3684d66782109d
  23:     0x7ff9cdd2af96 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  24:     0x7ff9cdd2887a - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  25:     0x7ff9cdd28eb7 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  26:     0x7ff9cdce557e - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  27:     0x7ff9cdd292d1 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  28:     0x7ff9cdce557e - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  29:     0x7ff9cdd292d1 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  30:     0x7ff9cdd29a3f - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  31:     0x7ff9cdd28eb7 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  32:     0x7ff9cdce57fe - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  33:     0x7ff9cdd29109 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  34:     0x7ff9cdce5725 - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  35:     0x7ff9cdd29453 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  36:     0x7ff9cdd28eb7 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  37:     0x7ff9cdce570e - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  38:     0x7ff9cdd29771 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  39:     0x7ff9cdce570e - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  40:     0x7ff9cdd29771 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  41:     0x7ff9cdce5900 - <rustc_typeck::outlives::test::OutlivesTest<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_impl_item::h5212b77bfc33afaf
  42:     0x7ff9cdd29109 - <rustc_typeck::check::regionck::RegionCtxt<'a, 'gcx, 'tcx> as rustc::hir::intravisit::Visitor<'gcx>>::visit_expr::hcb39bebdbb74d460
  43:     0x7ff9cdd27306 - <rustc_typeck::check::writeback::Resolver<'cx, 'gcx, 'tcx> as rustc::ty::fold::TypeFolder<'gcx, 'tcx>>::fold_region::hb1622b89d3435e74
  44:     0x7ff9cdd869be - <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::h540dd53913ae351e
  45:     0x7ff9cdd82bb6 - <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::h540dd53913ae351e
  46:     0x7ff9c873fbba - rustc::dep_graph::graph::DepGraph::assert_ignored::h144efdc901c11500
  47:     0x7ff9c8aa6c76 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::h2b2d41c86f9a6e6e
  48:     0x7ff9c8aa7dad - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::try_get::h0bc6807c7004a3fd
  49:     0x7ff9c8be81df - rustc::ty::maps::TyCtxtAt::typeck_tables_of::hc49daa61101a1c23
  50:     0x7ff9c8aa6881 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_tables_of<'tcx>>::ensure::h2b2d41c86f9a6e6e
  51:     0x7ff9cdd821c2 - <rustc_typeck::check::CheckItemTypesVisitor<'a, 'tcx> as rustc::hir::itemlikevisit::ItemLikeVisitor<'tcx>>::visit_item::h540dd53913ae351e
  52:     0x7ff9c875a7dc - rustc::dep_graph::graph::DepGraph::assert_ignored::h144efdc901c11500
  53:     0x7ff9c8aa43d1 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::ensure::ha12c0309982046bf
  54:     0x7ff9c8aa5444 - rustc::ty::maps::<impl rustc::ty::maps::queries::typeck_item_bodies<'tcx>>::try_get::hdf04a0cb4267327c
  55:     0x7ff9c8be80c7 - rustc::ty::maps::TyCtxtAt::typeck_item_bodies::h7155964f4ae16ed6
  56:     0x7ff9c8be4412 - rustc::ty::maps::<impl rustc::ty::context::TyCtxt<'a, 'tcx, 'lcx>>::typeck_item_bodies::hc8981e99eb7d74ce
  57:     0x7ff9cdde526e - rustc_typeck::check_crate::h82688740b0ec7d1c
  58:     0x7ff9da8fb6e7 - <env_logger::Logger as log::Log>::flush::he47d2badcc01211e
  59:     0x7ff9da972064 - rustc_driver::driver::compile_input::hd50b5e33b4a9e551
  60:     0x7ff9da997f80 - rustc_driver::run_compiler::h271536ce91a9abed
  61:     0x7ff9da894b60 - <unknown>
  62:     0x7ff9c9a4ae11 - _rust_maybe_catch_panic
  63:     0x7ff9da8d4f94 - <env_logger::Logger as log::Log>::flush::he47d2badcc01211e
  64:     0x7ff9c9a48b4e - std::sys::windows::thread::Thread::new::he6cb72092aa085f2
  65:     0x7ffa068a1fe3 - BaseThreadInitThunk
  66:     0x7ffa0942efc0 - RtlUserThreadStart

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.26.0-nightly (521d91c6b 2018-03-14) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `rngerr`.

Caused by:
  process didn't exit successfully: `rustc --crate-name rngerr src\main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=90b29a1b474d9a34 -C extra-filename=-90b29a1b474d9a34 --out-dir D:\rustproj\rngerr\target\debug\deps -C incremental=D:\rustproj\rngerr\target\debug\incremental -L dependency=D:\rustproj\rngerr\target\debug\deps --extern rand=D:\rustproj\rngerr\target\debug\deps\librand-39fb58d4902b5813.rlib` (exit code: 101)

Process finished with exit code 101
