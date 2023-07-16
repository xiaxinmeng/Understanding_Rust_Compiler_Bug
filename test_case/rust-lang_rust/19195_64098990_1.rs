
<std macros>:3:24: 8:28 error: type `e::builder::Serializer` cannot be dereferenced
<std macros>:3         let dst = &mut *$dst;
<std macros>:4         format_args!(|args| { dst.write_fmt(args) }, $($arg)*)
<std macros>:5     })
<std macros>:6 )
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'index out of bounds: the len is 6 but the index is 6', /Users/rustbuild/src/rust-buildbot/slave/nightly-mac/build/src/libsyntax/lib.rs:1

stack backtrace:
   1:        0x112ca15f2 - rt::backtrace::imp::write::he31e36fe59b39292ugt
   2:        0x112ca46ad - failure::on_fail::h7c2689f722982652nCt
   3:        0x112f0f575 - unwind::begin_unwind_inner::h09866e00e97d31fbI2c
   4:        0x112f0f1bf - unwind::begin_unwind_fmt::h443e47d11b48984fTZc
   5:        0x112f0ef62 - rust_begin_unwind
   6:        0x112f59bfc - panicking::panic_fmt::hc979f6c0b0157f68RQl
   7:        0x112f62341 - panicking::panic_bounds_check::hfad71607541b5567pPl
   8:        0x110c711e5 - codemap::FileMap::get_line::h40447e4b57225531m4E
   9:        0x110c9f8d3 - diagnostic::emit::hd3a2cf9de9dee335wvG
  10:        0x110c9c168 - diagnostic::EmitterWriter.Emitter::emit::h8d15319a02e5e7a2lrG
  11:        0x110c9a955 - diagnostic::Handler::emit::h444f90359ca9a887F8F
  12:        0x110c6d37b - diagnostic::SpanHandler::span_err::he607876660b5f38fEWF
  13:        0x11006b990 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message_str_with_expected::h86874d1701a3af0cTKF
  14:        0x10ffbb991 - middle::typeck::infer::InferCtxt<'a, 'tcx>::type_error_message::hbb15e2e5976d3123dQF
  15:        0x10ffcc5a9 - middle::typeck::check::check_expr_with_unifier::h8837607ae16c10b8Nzm
  16:        0x10ffca9a4 - middle::typeck::check::check_expr_with_unifier::h8837607ae16c10b8Nzm
  17:        0x10fff9123 - middle::typeck::check::check_decl_local::h2f92b49b94b8bcc5gto
  18:        0x10fff9313 - middle::typeck::check::check_stmt::h7d4de28493e7bc88gvo
  19:        0x10ff80258 - middle::typeck::check::check_block_with_expected::he7085b8c57c115ecvzo
  20:        0x10ffc67d0 - middle::typeck::check::check_expr_with_unifier::h8837607ae16c10b8Nzm
  21:        0x10ff804cf - middle::typeck::check::check_block_with_expected::he7085b8c57c115ecvzo
  22:        0x10ff59f70 - middle::typeck::check::check_fn::h604cf1058f7c9abce0i
  23:        0x10ff7cb1e - middle::typeck::check::check_bare_fn::h42d50bc38b24ab59jPi
  24:        0x10ff835bf - middle::typeck::check::check_method_body::h3b2984d8707874d1lkj
  25:        0x10ff79bf8 - middle::typeck::check::check_item::ha935ead12b2c5b2ft9i
  26:        0x10ff7c3a2 - visit::walk_mod::h16226120491347839638
  27:        0x10ff7c3ad - visit::walk_mod::h16226120491347839638
  28:        0x10ff7c84b - middle::typeck::check::check_item_types::h466c6c16edcec5c9tOi
  29:        0x110300896 - util::common::time::h12864813766357117792
  30:        0x1102ffd14 - middle::typeck::check_crate::h1acd5e14d073c903cwM
  31:        0x10f8f29a3 - driver::driver::phase_3_run_analysis_passes::h098de55f4a14acee2eS
  32:        0x10f8e6ecc - driver::driver::compile_input::h814765d870318dcf8VR
  33:        0x10f964e65 - driver::run_compiler::h54cbb685f3e65071gUT
  34:        0x10f96363e - driver::run::closure.59668
  35:        0x10f77b5ae - task::TaskBuilder::try_future::closure.38921
  36:        0x112c2f2e4 - task::TaskBuilder::spawn_internal::closure.24355
  37:        0x112f0cddd - task::Task::spawn::closure.5917
  38:        0x112f716ac - rust_try_inner
  39:        0x112f71696 - rust_try
  40:        0x112f0ceb7 - unwind::try::h207282f5be16daac3Qc
  41:        0x112f0cc8c - task::Task::run::h7eee475ba5444e2993b
  42:        0x112f0c98f - task::Task::spawn::closure.5893
  43:        0x112f0e627 - thread::thread_start::hc58864c152053492Wnc
  44:     0x7fff88b882fc - _pthread_body
  45:     0x7fff88b88279 - _pthread_body
