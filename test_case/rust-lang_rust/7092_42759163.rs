
ice.rs:6:9: 6:20 error: mismatched types: expected `Whatever` but found `core::option::Option<<generic #3>>` (expected enum Whatever but found enum core::option::Option)
ice.rs:6         Some(field) => field.access(),
                 ^~~~~~~~~~~
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 0 but the index is 0', /Users/erickt/rust/rust-master/src/librustc/lib.rs:1
stack backtrace:
   1:        0x108c945b5 - rt::backtrace::imp::write::hae9e0c8649bd0651E1F::v0.11.pre
   2:        0x108c016ae - rt::unwind::begin_unwind_inner::hd5684ca19e9518eaIBF::v0.11.pre
   3:        0x108c940eb - rt::unwind::begin_unwind::h10884007455834358118::v0.11.pre
   4:        0x108c9404d - rust_fail_bounds_check
   5:        0x108ca5e8e - failure::fail_bounds_check::h775bc9e8b5d0892aEwF::v0.11.pre
   6:        0x106193aa0 - fmt::secret_poly::h6854559833159012233::v0.11.pre
   7:        0x10619218a - middle::typeck::check::autoderef::h14491325893577732316::v0.11.pre
   8:        0x10618ee03 - middle::typeck::check::method::LookupContext<'a>::push_bound_candidates::h7551bbe3b0247cd3mm6::v0.11.pre
   9:        0x10618f14f - middle::typeck::check::method::lookup_in_trait::ha897a1a0808c04aeG45::v0.11.pre
  10:        0x10618fd76 - middle::typeck::check::try_overloaded_deref::h0523ebb93abe16ad9da::v0.11.pre
  11:        0x1061922b4 - middle::typeck::check::autoderef::h14491325893577732316::v0.11.pre
  12:        0x10618d607 - middle::typeck::check::method::lookup::hfa80a1cd76ce18fcbX5::v0.11.pre
  13:        0x1061cc95d - middle::typeck::check::check_expr_with_unifier::h4e2897dbe4ea38cclIa::v0.11.pre
  14:        0x10612620e - middle::typeck::check::_match::check_match::h7a88d4ad16195f4381Z::v0.11.pre
  15:        0x1061cd570 - middle::typeck::check::check_expr_with_unifier::h4e2897dbe4ea38cclIa::v0.11.pre
  16:        0x1061ac665 - middle::typeck::check::check_block_with_expected::hc40e54c79cbf136ak7c::v0.11.pre
  17:        0x1061a7c6a - middle::typeck::check::check_fn::h110e9312e887d6depv8::v0.11.pre
  18:        0x1061a745a - middle::typeck::check::check_bare_fn::h63ca38b0c972d0e4Wk8::v0.11.pre
  19:        0x1061a008f - middle::typeck::check::check_item::hb9ceb87f27759b1a0R8::v0.11.pre
  20:        0x1061a71dd - middle::typeck::check::check_item_types::hb17499c25494b6f0ek8::v0.11.pre
  21:        0x1062df3d6 - util::common::time::h17646179309215505883::v0.11.pre
  22:        0x1062de44d - middle::typeck::check_crate::hc361b33fe1242173Qgw::v0.11.pre
  23:        0x106707529 - driver::driver::phase_3_run_analysis_passes::h0f47e246146a1253bei::v0.11.pre
  24:        0x10670cfe2 - driver::driver::compile_input::h2c3dbe74f799efa1gEi::v0.11.pre
  25:        0x106733211 - run_compiler::h6db406e36b756e80saq::v0.11.pre
  26:        0x10674ad8d - main_args::closure.93972
  27:        0x106749502 - monitor::closure.93850
  28:        0x10674405b - task::TaskBuilder::try::closure.93616
  29:        0x1082d7b6c - task::spawn_opts::closure.7397
  30:        0x108c8bc18 - rt::task::Task::run::closure.28413
  31:        0x108cacb7c - rust_try
  32:        0x108c8ba97 - rt::task::Task::run::hf99ca2c1546bbfb0XqD::v0.11.pre
  33:        0x1082d79ef - task::spawn_opts::closure.7369
  34:        0x108c92f16 - rt::thread::thread_start::h61d07f070734480188D::v0.11.pre
  35:     0x7fff90436899 - _pthread_body
  36:     0x7fff9043672a - _pthread_struct_init
