
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'assertion failed: !r0.is_bound()', /Users/erickt/rust/rust-master/src/librustc/middle/typeck/infer/lub.rs:154
stack backtrace:
   1:        0x112b345b5 - rt::backtrace::imp::write::hae9e0c8649bd0651E1F::v0.11.pre
   2:        0x112aa16ae - rt::unwind::begin_unwind_inner::hd5684ca19e9518eaIBF::v0.11.pre
   3:        0x112aa01d7 - rt::unwind::begin_unwind_fmt::hac2a7e1b9194970bSAF::v0.11.pre
   4:        0x1100948dc - middle::typeck::infer::lub::Lub<'f>.Combine::fn_sigs::closure.70100
   5:        0x10ff35076 - middle::ty_fold::RegionFolder<'a>.TypeFolder::fold_region::h7dda58a08c21a343l0Q::v0.11.pre
   6:        0x10ff347c5 - middle::ty_fold::super_fold_sty::h13291029136304019429::v0.11.pre
   7:        0x10fefb2a0 - middle::ty_fold::RegionFolder<'a>.TypeFolder::fold_ty::h6392d01fc77bc3084XQ::v0.11.pre
   8:        0x10ff237e0 - iter::Iterator::collect::h15992120228646370080::v0.11.pre
   9:        0x10ff35160 - middle::ty_fold::super_fold_sig::h13577425788296859040::v0.11.pre
  10:        0x10ff34cad - middle::ty_fold::super_fold_sty::h13291029136304019429::v0.11.pre
  11:        0x10fefb2a0 - middle::ty_fold::RegionFolder<'a>.TypeFolder::fold_ty::h6392d01fc77bc3084XQ::v0.11.pre
  12:        0x10ff237e0 - iter::Iterator::collect::h15992120228646370080::v0.11.pre
  13:        0x10ff34ea3 - middle::ty_fold::TypeFolder::fold_substs::h17351997724182444375::v0.11.pre
  14:        0x10ff34604 - middle::ty_fold::super_fold_sty::h13291029136304019429::v0.11.pre
  15:        0x10fefb2a0 - middle::ty_fold::RegionFolder<'a>.TypeFolder::fold_ty::h6392d01fc77bc3084XQ::v0.11.pre
  16:        0x10ff237e0 - iter::Iterator::collect::h15992120228646370080::v0.11.pre
  17:        0x110093e20 - middle::typeck::infer::lub::Lub<'f>.Combine::fn_sigs::h2881151a6d6777f5NMi::v0.11.pre
  18:        0x11009c9d8 - middle::typeck::infer::combine::Combine::closure_tys::h14040719990868578090::v0.11.pre
  19:        0x110096d9b - middle::typeck::infer::lattice::super_lattice_tys::h7287454358332737396::v0.11.pre
  20:        0x110084823 - middle::typeck::infer::lattice::ty..t.LatticeValue::lub::hcf901c13c4c1fce75Bh::v0.11.pre
  21:        0x1100848cc - middle::typeck::infer::lattice::LatticeValue::lub::as_closure.69820
  22:        0x110084437 - middle::typeck::infer::lattice::CombineFields<'f>.CombineFieldsLatticeMethods::merge_bnd::h13068228610379944350::v0.11.pre
  23:        0x11008385f - middle::typeck::infer::lattice::CombineFields<'f>.CombineFieldsLatticeMethods::set_var_to_merged_bounds::h2086516640415841397::v0.11.pre
  24:        0x1100765a7 - middle::typeck::infer::sub::Sub<'f>.Combine::tys::h175f11cc5dc2597d0fm::v0.11.pre
  25:        0x11007a63c - middle::typeck::infer::sub::Sub<'f>.Combine::contratys::h22fafc5b276ecf36w6l::v0.11.pre
  26:        0x1100bfd65 - middle::typeck::infer::combine::eq_tys::closure.70748
  27:        0x110072bcc - middle::typeck::infer::InferCtxt<'a>::try::h18173051383935161273::v0.11.pre
  28:        0x1100bfc61 - middle::typeck::infer::combine::eq_tys::h14490634581668630721::v0.11.pre
  29:        0x1100be580 - middle::typeck::infer::combine::Combine::substs::h8806601970722992653::v0.11.pre
  30:        0x1100770c1 - middle::typeck::infer::sub::Sub<'f>.Combine::tys::h175f11cc5dc2597d0fm::v0.11.pre
  31:        0x1100c6e0b - middle::typeck::infer::coercion::Coerce<'f>::subtype::h62ea9fdfc572f0d8sfn::v0.11.pre
  32:        0x1100c6cb7 - middle::typeck::infer::coercion::Coerce<'f>::tys::closure.70831
  33:        0x1100c38d1 - middle::typeck::infer::coercion::Coerce<'f>::unpack_actual_value::h2dfaecaef773b72d1fn::v0.11.pre
  34:        0x1100c36cd - middle::typeck::infer::coercion::Coerce<'f>::tys::h2efd240de30beacfb8m::v0.11.pre
  35:        0x1100eac9d - middle::typeck::infer::mk_coercety::closure.71497
  36:        0x1100ea7d3 - middle::typeck::infer::InferCtxt<'a>::commit::closure.71487
  37:        0x1100e97e0 - util::common::indent::h14231954806218317178::v0.11.pre
  38:        0x1100ea5f4 - middle::typeck::infer::mk_coercety::closure.71483
  39:        0x1100e97e0 - util::common::indent::h14231954806218317178::v0.11.pre
  40:        0x110053613 - middle::typeck::infer::mk_coercety::h0c90b6b9e175a0187aq::v0.11.pre
  41:        0x110015da1 - middle::typeck::check::FnCtxt<'a>::mk_assignty::hb3de0ef94ed252baG59::v0.11.pre
  42:        0x110015c28 - middle::typeck::check::demand::coerce::h9b03fd3cf1701d89lS5::v0.11.pre
  43:        0x11005d25a - middle::typeck::check::check_expr_coercable_to_type::closure.69083
  44:        0x110054604 - middle::typeck::check::check_expr_with_unifier::h4e2897dbe4ea38cclIa::v0.11.pre
  45:        0x11005e31c - middle::typeck::check::check_expr_with_unifier::check_argument_types::h16c021d4d2f9109aAMa::v0.11.pre
  46:        0x11005965a - middle::typeck::check::check_expr_with_unifier::h4e2897dbe4ea38cclIa::v0.11.pre
  47:        0x110070413 - middle::typeck::check::check_stmt::hfaddb728054d31d4n3c::v0.11.pre
  48:        0x1100353d3 - middle::typeck::check::check_block_with_expected::hc40e54c79cbf136ak7c::v0.11.pre
  49:        0x110030c6a - middle::typeck::check::check_fn::h110e9312e887d6depv8::v0.11.pre
  50:        0x11003045a - middle::typeck::check::check_bare_fn::h63ca38b0c972d0e4Wk8::v0.11.pre
  51:        0x11002908f - middle::typeck::check::check_item::hb9ceb87f27759b1a0R8::v0.11.pre
  52:        0x1100301dd - middle::typeck::check::check_item_types::hb17499c25494b6f0ek8::v0.11.pre
  53:        0x1101683d6 - util::common::time::h17646179309215505883::v0.11.pre
  54:        0x11016744d - middle::typeck::check_crate::hc361b33fe1242173Qgw::v0.11.pre
  55:        0x110590529 - driver::driver::phase_3_run_analysis_passes::h0f47e246146a1253bei::v0.11.pre
  56:        0x110595fe2 - driver::driver::compile_input::h2c3dbe74f799efa1gEi::v0.11.pre
  57:        0x1105bc211 - run_compiler::h6db406e36b756e80saq::v0.11.pre
  58:        0x1105d3d8d - main_args::closure.93972
  59:        0x1105d2502 - monitor::closure.93850
  60:        0x1105cd05b - task::TaskBuilder::try::closure.93616
  61:        0x112163b6c - task::spawn_opts::closure.7397
  62:        0x112b2bc18 - rt::task::Task::run::closure.28413
  63:        0x112b4cb7c - rust_try
  64:        0x112b2ba97 - rt::task::Task::run::hf99ca2c1546bbfb0XqD::v0.11.pre
  65:        0x1121639ef - task::spawn_opts::closure.7369
  66:        0x112b32f16 - rt::thread::thread_start::h61d07f070734480188D::v0.11.pre
  67:     0x7fff90436899 - _pthread_body
  68:     0x7fff9043672a - _pthread_struct_init
