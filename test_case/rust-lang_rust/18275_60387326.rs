
% RUST_BACKTRACE=1 ./objdir-dbgopt/x86_64-apple-darwin/stage2/bin/rustc /tmp/bar.rs --version=verbose
rustc 0.13.0-dev (00cc6d240 2014-10-24 03:22:29 +0000)
binary: rustc
commit-hash: 00cc6d24099eb93ecfeb9bf807ab9e5130a01749
commit-date: 2014-10-24 03:22:29 +0000
host: x86_64-apple-darwin
release: 0.13.0-dev
% RUST_BACKTRACE=1 ./objdir-dbgopt/x86_64-apple-darwin/stage2/bin/rustc /tmp/bar.rs
<std macros>:8:12: 13:14 error: cannot apply unary operator `!` to type `BytePos`
<std macros>:8         if !$cond {
<std macros>:9             fail!($($arg),+)
<std macros>:10         }
<std macros>:11     );
<std macros>:12 )
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 12 but the index is 12', /Users/fklock/Dev/Mozilla/rust-snapshot/src/libsyntax/lib.rs:1

stack backtrace:
   1:        0x1123871a9 - rt::backtrace::imp::write::h450ad0303b268c20mlq
   2:        0x11238a487 - failure::on_fail::h954f7ba7ddf2485bVBq
   3:        0x1125e7575 - unwind::begin_unwind_inner::h64282a3c01d28525Dud
   4:        0x1125e723c - unwind::begin_unwind_fmt::h43c98bc3fdde71815rd
   5:        0x1125e6f82 - rust_begin_unwind
   6:        0x11264191c - failure::fail_fmt::h545424e5ef794da8C6j
   7:        0x1126417ee - failure::fail_bounds_check::hea6bb9ff86576e82b5j
   8:        0x111afe4f5 - codemap::FileMap::get_line::h9b8f29fe6dd6b3ddqRE
   9:        0x111b32a44 - diagnostic::emit::h381793e700cfb553NgG
  10:        0x111b2f978 - diagnostic::EmitterWriter.Emitter::emit::h935056a4ed05f506CcG
  11:        0x111b2e055 - diagnostic::Handler::emit::ha88a06e85ece44e43UF
  12:        0x111afa5fb - diagnostic::SpanHandler::span_err::h2e9c6026028b1e1a3IF
  13:        0x10f59f402 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message_str_with_expected::hbcc6cf7fa1686a4dZ5g
  14:        0x10f6355a7 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message::he5d4cd4372aca221ich
  15:        0x10f66044d - middle::typeck::check::check_expr_with_unifier::check_user_unop::closure.132236
  16:        0x10f65fa5b - middle::typeck::check::check_expr_with_unifier::lookup_op_method::h6c2097b51e57bbb4pDY
  17:        0x10f644638 - middle::typeck::check::check_expr_with_unifier::check_user_unop::hb4fa96664bfafe23sRY
  18:        0x10f64047c - middle::typeck::check::check_expr_with_unifier::he64cbf20279ad3bbUrY
  19:        0x10f63b4bb - middle::typeck::check::check_expr_with_unifier::he64cbf20279ad3bbUrY
  20:        0x10f68a118 - middle::typeck::check::check_stmt::ha32bf38ebd3265f16v0
  21:        0x10f604252 - middle::typeck::check::check_block_with_expected::h63d5b8578d681e04jA0
  22:        0x10f5ffdd4 - middle::typeck::check::check_fn::he2d996f1a6aef5f5bfV
  23:        0x10f5ff29b - middle::typeck::check::check_bare_fn::h83e28ac62974598bj4U
  24:        0x10f5fa950 - middle::typeck::check::check_item::h141096c04d62cfc8koV
  25:        0x10f5ff0ef - middle::typeck::check::check_item_types::h1dc42b2287b341b3t3U
  26:        0x10f0ca6d6 - util::common::time::h18025842159856833784
  27:        0x10f8cb75e - middle::typeck::check_crate::h17e534da1afab501ZKn
  28:        0x10f935acf - driver::driver::phase_3_run_analysis_passes::hea279093693c255aPoA
  29:        0x10f930a7e - driver::driver::compile_input::h4542a76382479c93A5z
  30:        0x10f9b4fdf - driver::run_compiler::hacf02147e815207aLSD
  31:        0x10f9b3166 - driver::run::closure.144840
  32:        0x10f0e3d9b - task::TaskBuilder<S>::try_future::closure.103258
  33:        0x10f0e3c83 - task::TaskBuilder<S>::spawn_internal::closure.103229
  34:        0x10f06188d - task::NativeSpawner.Spawner::spawn::closure.8522
  35:        0x112651a3c - rust_try_inner
  36:        0x112651a26 - rust_try
  37:        0x1125e4be7 - unwind::try::h7804299545ba8120ljd
  38:        0x1125e4a6c - task::Task::run::hec134634f5f379cfwzc
  39:        0x10f0616b2 - task::NativeSpawner.Spawner::spawn::closure.8461
  40:        0x1125e642a - thread::thread_start::ha303fb9747920e39HTc
  41:     0x7fff903c4899 - _pthread_body
  42:     0x7fff903c472a - _pthread_struct_init

% 
