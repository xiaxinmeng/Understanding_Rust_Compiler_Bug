
foo.rs:4:49: 4:50 error: internal compiler error: unbound path path(S)
foo.rs:4     fn transfer(&mut self) -> <Self as Transfer<S>>::R;
                                                         ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/diagnostic.rs:116

stack backtrace:
   1:     0x7f41fc192820 - rt::backtrace::imp::write::h8a05a2225574bd5ascq
   2:     0x7f41fc1958a0 - failure::on_fail::h3903c5b5788bc95c3xq
   3:     0x7f42006dea00 - unwind::begin_unwind_inner::h0919efe752d6fd357Rd
   4:     0x7f41fccdde40 - unwind::begin_unwind::h12448235468951990776
   5:     0x7f41fccdddc0 - diagnostic::SpanHandler::span_bug::h05cbb84cec88209aH6F
   6:     0x7f4200f96310 - middle::typeck::astconv::ast_ty_to_prim_ty::h9f95d12e1d973d81rR4
   7:     0x7f42013a38d0 - middle::typeck::astconv::ast_ty_to_ty::h12861914180600525724
   8:     0x7f4201068d60 - iter::Iterator::collect::h13337330953596778502
   9:     0x7f42013a6610 - middle::typeck::astconv::ast_path_substs::h13947801815788576546
  10:     0x7f42013a88c0 - middle::typeck::astconv::ast_path_to_trait_ref::h17933483232567598950
  11:     0x7f42013a9380 - middle::typeck::astconv::associated_ty_to_ty::h8004787093255996260
  12:     0x7f42013a38d0 - middle::typeck::astconv::ast_ty_to_ty::h12861914180600525724
  13:     0x7f420139b470 - middle::typeck::astconv::ty_of_method_or_bare_fn::h17568311308738250480
  14:     0x7f420137b230 - middle::typeck::collect::collect_trait_methods::ty_method_of_trait_method::hf39311a07abed50b7dk
  15:     0x7f4201357dd0 - middle::typeck::collect::convert::hde1a9421fe94c3ef0Uk
  16:     0x7f4201351640 - middle::typeck::collect::collect_item_types::h2377a8e9bb3d5ad9GUj
  17:     0x7f4200d3fbc0 - util::common::time::h6545073167695776246
  18:     0x7f42015378d0 - middle::typeck::check_crate::h0f9c9a0fae05a0bfJEp
  19:     0x7f42015a0940 - driver::driver::phase_3_run_analysis_passes::h46245b66b26614aeJoC
  20:     0x7f420159b7a0 - driver::driver::compile_input::he3b1257e173b8860u5B
  21:     0x7f420161e900 - driver::run_compiler::hd836d04808dd8d5eLVF
  22:     0x7f420161e7f0 - driver::run::closure.146540
  23:     0x7f4200d58b00 - task::TaskBuilder<S>::try_future::closure.104852
  24:     0x7f4200d588f0 - task::TaskBuilder<S>::spawn_internal::closure.104823
  25:     0x7f4200a25630 - task::NativeSpawner.Spawner::spawn::closure.8456
  26:     0x7f42007338c0 - rust_try_inner
  27:     0x7f42007338b0 - rust_try
  28:     0x7f42006dc380 - unwind::try::h32143614f9ac9c88PGd
  29:     0x7f42006dc210 - task::Task::run::h8c90cb04437a131dFMc
  30:     0x7f4200a25370 - task::NativeSpawner.Spawner::spawn::closure.8394
  31:     0x7f42006dda20 - thread::thread_start::h7bbfc46247c32113U7c
  32:     0x7f41fb544250 - start_thread
  33:     0x7f42003b83b9 - clone
  34:                0x0 - <unknown>
