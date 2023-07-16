 python
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'explicit failure', /home/robert/rust/src/librbml/lib.rs:246

stack backtrace:
   1:     0x7fb8a01f7d20 - rt::backtrace::imp::write::h3bf50c6342044859pxr
   2:     0x7fb8a01fadf0 - failure::on_fail::hf2af25ddfb61c6166Sr
   3:     0x7fb8a09d5570 - unwind::begin_unwind_inner::h270bb14ed5ac9f3bDje
   4:     0x7fb89d27e1e0 - unwind::begin_unwind::h6560570121837697204
   5:     0x7fb89d27d210 - reader::get_doc::h237bf80ea65bd6f7nSa
   6:     0x7fb8a164d370 - metadata::decoder::get_type::h63ccb7ae3ad1b1c6BCu
   7:     0x7fb8a108b5e0 - middle::ty::lookup_item_type::hb024590ac0e678bcqRI
   8:     0x7fb8a13e8dd0 - middle::typeck::check::check_expr_with_unifier::he78a23726fbcaf50DxW
   9:     0x7fb8a1448740 - middle::typeck::check::check_decl_local::heef8e1b61c708c61EAY
  10:     0x7fb8a1448960 - middle::typeck::check::check_stmt::h8de414f234b1a15bMCY
  11:     0x7fb8a13b2c70 - middle::typeck::check::check_block_with_expected::h17706bf76c83a2f8ZGY
  12:     0x7fb8a13aea00 - middle::typeck::check::check_fn::h064a0d44e9dce305oOT
  13:     0x7fb8a13ae720 - middle::typeck::check::check_bare_fn::hc2b8bc7e443848ff3CT
  14:     0x7fb8a13a7880 - middle::typeck::check::check_item::h73a8feb360d03f6ebcU
  15:     0x7fb8a13ae520 - middle::typeck::check::check_item_types::hecc74b4cd245d819kCT
  16:     0x7fb8a0dbb9a0 - util::common::time::h15153310036782750645
  17:     0x7fb8a15bc150 - middle::typeck::check_crate::hed3fed93a6fab9e6Nfl
  18:     0x7fb8a1689d00 - driver::driver::phase_3_run_analysis_passes::h83579db2878d0392UQz
  19:     0x7fb8a1684e40 - driver::driver::compile_input::h5f1cded471f161260Cz
  20:     0x7fb8a1733380 - driver::run_compiler::hf3375986adbeb8b5UaD
  21:     0x7fb8a1733290 - driver::main_args::closure.137700
  22:     0x7fb8a1745cc0 - task::TaskBuilder<S>::try_future::closure.138861
  23:     0x7fb8a1745ac0 - task::TaskBuilder<S>::spawn_internal::closure.138838
  24:     0x7fb8a2195730 - task::spawn_opts::closure.8450
  25:     0x7fb8a0a372a0 - rust_try_inner
  26:     0x7fb8a0a37290 - rust_try
  27:     0x7fb8a09d2bd0 - unwind::try::h461f8c6e0791ff8aT7d
  28:     0x7fb8a09d2970 - task::Task::run::h33f4c45a9024a975ded
  29:     0x7fb8a21954f0 - task::spawn_opts::closure.8396
  30:     0x7fb8a09d47b0 - thread::thread_start::hc36a7b331026829dZCd
  31:     0x7fb89fc87fe0 - start_thread
  32:     0x7fb8a069e019 - __clone
  33:                0x0 - <unknown>
