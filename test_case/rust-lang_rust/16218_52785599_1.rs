
baz2.rs:1:1: 1:1 error: internal compiler error: Type parameter out of range when substituting in region 'a (root type='aint) (space=FnSpace, index=0)
baz2.rs:1 trait Foo<T> { }
          ^
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/erickt/rust/rust-master/src/libsyntax/ast_util.rs:776

stack backtrace:
   1:        0x10db80df5 - rt::backtrace::imp::write::he623acc81864f201rGr
   2:        0x10db83fc3 - failure::on_fail::h8f9f19a91c24052dkXr
   3:        0x10de4d695 - unwind::begin_unwind_inner::h5d3cfad59626c9bcufe
   4:        0x10bc190a2 - unwind::begin_unwind::h6605622708525903273
   5:        0x10bc19035 - diagnostic::SpanHandler::span_bug::he3e556404dcd363fjuF
   6:        0x10a7fe8b5 - driver::session::Session::span_bug::h513f935bd5a535ef2nB
   7:        0x10a9f5d2c - middle::subst::SubstFolder<'a>.TypeFolder::fold_region::hc0e73481797b47e5FPX
   8:        0x10a8dfd6c - middle::subst::SubstFolder<'a>.TypeFolder::fold_ty::hcbee2fbe0178d03aySX
   9:        0x10a9f6ddf - middle::subst::VecPerParamSpace<T>::map::h6071544764689056965
  10:        0x10a9f5f25 - middle::ty_fold::TypeFolder::fold_substs::h12490128756786921515
  11:        0x10ab6c055 - middle::subst::T.Subst::subst_spanned::h12811249184876261003
  12:        0x10ac12686 - middle::typeck::check::compare_impl_method::h2c4dc46ebf448831WwU
  13:        0x10abfd0be - middle::typeck::check::check_item::h03dd381d5ee24cf84aU
  14:        0x10ac034fd - middle::typeck::check::check_item_types::h66154ed2b2471f75dBT
  15:        0x10a62bfe6 - util::common::time::h8593819081890839475
  16:        0x10ae00aac - middle::typeck::check_crate::h01f2334d92065f26Fel
  17:        0x10aecce60 - driver::driver::phase_3_run_analysis_passes::h875833fb36cd3ebdMPz
  18:        0x10aec7e53 - driver::driver::compile_input::h6f91988ab025da07SBz
  19:        0x10af6fb82 - driver::run_compiler::h7fbb31eb141b1300M9C
  20:        0x10af6e266 - driver::main_args::closure.137573
  21:        0x10af803bb - task::TaskBuilder<S>::try_future::closure.138734
  22:        0x10af802c5 - task::TaskBuilder<S>::spawn_internal::closure.138711
  23:        0x10ba6d86c - task::spawn_opts::closure.8427
  24:        0x10deb45cc - rust_try_inner
  25:        0x10deb45b6 - rust_try
  26:        0x10de4aa6b - unwind::try::hb02e6a03665b7aa7K3d
  27:        0x10de4a80b - task::Task::run::hbb7219e7e81ccdc1Bad
  28:        0x10ba6d6ca - task::spawn_opts::closure.8372
  29:        0x10de4c626 - thread::thread_start::h41227a904179836dnzd
  30:     0x7fff82db2899 - _pthread_body
  31:     0x7fff82db272a - _pthread_struct_init
