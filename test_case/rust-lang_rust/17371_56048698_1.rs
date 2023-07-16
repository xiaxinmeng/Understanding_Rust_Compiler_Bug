
lifetimes.rs:6:18: 6:19 error: internal compiler error: expected object type
lifetimes.rs:6   let s : &T = &{S as T};
                                ^
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/ast_util.rs:694

stack backtrace:
   1:        0x10ee50029 - rt::backtrace::imp::write::he5743b41cdf00d03RPq
   2:        0x10ee53381 - failure::on_fail::h60a3c41a8a2f283dv6q
   3:        0x10f0dae65 - unwind::begin_unwind_inner::h7bb0299ab4bd6f7cPQd
   4:        0x10e5d5f17 - unwind::begin_unwind::h17978203292574916498
   5:        0x10e5d5e95 - diagnostic::SpanHandler::span_bug::h66f8d57f2d59a9157HD
   6:        0x10bd08387 - driver::session::Session::span_bug::h6106d5d788b20d1dUxx
   7:        0x10c07a558 - middle::typeck::check::vtable2::check_object_cast::h511862f19955be43rHK
   8:        0x10c0ecb27 - middle::typeck::check::check_cast::h5a102a8d4faf245fArT
   9:        0x10c118d44 - middle::typeck::check::check_expr_with_unifier::h3965963c41915f16alV
  10:        0x10c0dcfce - middle::typeck::check::check_block_with_expected::h08d5256a250e08843tX
  11:        0x10c119433 - middle::typeck::check::check_expr_with_unifier::h3965963c41915f16alV
  12:        0x10c11c23b - middle::typeck::check::check_expr_with_unifier::h3965963c41915f16alV
  13:        0x10c169d5c - middle::typeck::check::check_decl_local::hf7060244e24078ceInX
  14:        0x10c169ed5 - middle::typeck::check::check_stmt::h5650755903445ba5QpX
  15:        0x10c0dcc92 - middle::typeck::check::check_block_with_expected::h08d5256a250e08843tX
  16:        0x10c0d8142 - middle::typeck::check::check_fn::hd2684edf98104a26VhS
  17:        0x10c0d76e5 - middle::typeck::check::check_bare_fn::ha6164748b16cb8a5Y6R
  18:        0x10c0d17c4 - middle::typeck::check::check_item::h4c7a561062fe5d43DES
  19:        0x10c0d4c99 - visit::Visitor::visit_mod::h946532746211506472
  20:        0x10c0d754f - middle::typeck::check::check_item_types::h95ebad4be12b6800W5R
  21:        0x10bb603f6 - util::common::time::h8376823042161399267
  22:        0x10c2d8114 - middle::typeck::check_crate::hdfd3e86e41b822bb5Zi
  23:        0x10c3a034c - driver::driver::phase_3_run_analysis_passes::h7fcea340b471389f5Ow
  24:        0x10c39ad5f - driver::driver::compile_input::hd720f974cc40bf4b9ww
  25:        0x10c41a181 - driver::run_compiler::hee0994bc6ee5a397FnA
  26:        0x10c4183b6 - driver::main_args::closure.146253
  27:        0x10bb7928b - task::TaskBuilder<S>::try_future::closure.101952
  28:        0x10bb79183 - task::TaskBuilder<S>::spawn_internal::closure.101923
  29:        0x10baf834d - task::spawn_opts::closure.8525
  30:        0x10f13f5ac - rust_try_inner
  31:        0x10f13f596 - rust_try
  32:        0x10f0d8217 - unwind::try::heb45a9f3ad06986fxFd
  33:        0x10f0d808c - task::Task::run::h8b6131c80bb869e4PVc
  34:        0x10baf81a2 - task::spawn_opts::closure.8464
  35:        0x10f0d9d4a - thread::thread_start::h6c831d0134a78f631fd
  36:     0x7fff8d6ed899 - _pthread_body
  37:     0x7fff8d6ed72a - _pthread_struct_init
