
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'OwnedSlice: index out of bounds', /Users/erickt/rust/rust-master/src/libstd/option.rs:164
stack backtrace:
   1:        0x10c1bf5b5 - rt::backtrace::imp::write::hae9e0c8649bd0651E1F::v0.11.pre
   2:        0x10c12c6ae - rt::unwind::begin_unwind_inner::hd5684ca19e9518eaIBF::v0.11.pre
   3:        0x109412428 - rt::unwind::begin_unwind::h6306384670831263120::v0.11.pre
   4:        0x1095d0e5c - middle::subst::ty..Region.Subst::subst_spanned::ha6be338a3f2341abYkR::v0.11.pre
   5:        0x1096b96e2 - middle::typeck::check::method::LookupContext<'a>::push_inherent_candidates_from_object::closure.68413
   6:        0x1096ba801 - middle::typeck::check::method::LookupContext<'a>::push_inherent_candidates_from_bounds_inner::closure.68423
   7:        0x10959c523 - middle::ty::each_bound_trait_and_supertraits::hf62d6da34182ab08B3P::v0.11.pre
   8:        0x1096b6b97 - middle::typeck::check::method::LookupContext<'a>::push_inherent_candidates::closure.68388
   9:        0x1096b618a - middle::typeck::check::autoderef::h14491325893577732316::v0.11.pre
  10:        0x1096b1607 - middle::typeck::check::method::lookup::hfa80a1cd76ce18fcbX5::v0.11.pre
  11:        0x109703b6b - middle::typeck::check::check_expr_with_unifier::check_field::hfdc64548ea59d3e8KFb::v0.11.pre
  12:        0x1096f1f74 - middle::typeck::check::check_expr_with_unifier::h4e2897dbe4ea38cclIa::v0.11.pre
  13:        0x1096d0665 - middle::typeck::check::check_block_with_expected::hc40e54c79cbf136ak7c::v0.11.pre
  14:        0x1096cbc6a - middle::typeck::check::check_fn::h110e9312e887d6depv8::v0.11.pre
  15:        0x1096cb45a - middle::typeck::check::check_bare_fn::h63ca38b0c972d0e4Wk8::v0.11.pre
  16:        0x1096c408f - middle::typeck::check::check_item::hb9ceb87f27759b1a0R8::v0.11.pre
  17:        0x1096cb1dd - middle::typeck::check::check_item_types::hb17499c25494b6f0ek8::v0.11.pre
  18:        0x1098033d6 - util::common::time::h17646179309215505883::v0.11.pre
  19:        0x10980244d - middle::typeck::check_crate::hc361b33fe1242173Qgw::v0.11.pre
  20:        0x109c2b529 - driver::driver::phase_3_run_analysis_passes::h0f47e246146a1253bei::v0.11.pre
  21:        0x109c30fe2 - driver::driver::compile_input::h2c3dbe74f799efa1gEi::v0.11.pre
  22:        0x109c57211 - run_compiler::h6db406e36b756e80saq::v0.11.pre
  23:        0x109c6ed8d - main_args::closure.93972
  24:        0x109c6d502 - monitor::closure.93850
  25:        0x109c6805b - task::TaskBuilder::try::closure.93616
  26:        0x10b7f7b6c - task::spawn_opts::closure.7397
  27:        0x10c1b6c18 - rt::task::Task::run::closure.28413
  28:        0x10c1d7b7c - rust_try
  29:        0x10c1b6a97 - rt::task::Task::run::hf99ca2c1546bbfb0XqD::v0.11.pre
  30:        0x10b7f79ef - task::spawn_opts::closure.7369
  31:        0x10c1bdf16 - rt::thread::thread_start::h61d07f070734480188D::v0.11.pre
  32:     0x7fff90436899 - _pthread_body
  33:     0x7fff9043672a - _pthread_struct_init
