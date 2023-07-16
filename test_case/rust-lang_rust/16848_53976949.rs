
$ RUST_BACKTRACE=1 ./test.sh
error: internal compiler error: trying to take the sizing type of base::Expression, an unsized type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:784

stack backtrace:
   1:     0x7f92be37e120 - rt::backtrace::imp::write::h3c0523c13636656fOLq
   2:     0x7f92be381310 - failure::on_fail::h9a355b0a53be785cw7q
   3:     0x7f92beb69740 - unwind::begin_unwind_inner::h0ab5484e74ee0101rje
   4:     0x7f92ba5c1310 - unwind::begin_unwind::h13307728250319235624
   5:     0x7f92ba5c1c80 - diagnostic::Handler::bug::h71ea0abb056f036fCdF
   6:     0x7f92bef5ca50 - driver::session::Session::bug::he64b8581f4fba397U5C
   7:     0x7f92bf327a90 - middle::trans::type_of::sizing_type_of::h3b1d93e5efb712d5Qc9
   8:     0x7f92bf357a60 - middle::trans::expr::trans_unadjusted::h838117360a218d5eJj3
   9:     0x7f92bf314030 - middle::trans::expr::trans::hdcfc6f99615b9e0aIy2
  10:     0x7f92bf366310 - middle::trans::expr::trans_addr_of::h477361cd3bceafb4jK4
  11:     0x7f92bf357a60 - middle::trans::expr::trans_unadjusted::h838117360a218d5eJj3
  12:     0x7f92bf312860 - middle::trans::expr::trans_into::h1423af8cb4e82c3dMu2
  13:     0x7f92bf3f94f0 - middle::trans::_match::store_local::closure.121546
  14:     0x7f92bf3f91e0 - middle::trans::_match::mk_binding_alloca::h6724705899647199174
  15:     0x7f92bf3bfff0 - middle::trans::_match::store_local::hf89815ececead9d8G4h
  16:     0x7f92bf311ed0 - middle::trans::base::init_local::heb1cfcf6c59d83fe1Hd
  17:     0x7f92bf3113e0 - middle::trans::controlflow::trans_stmt::he0a1d9a619a4044drhY
  18:     0x7f92bf312e00 - middle::trans::controlflow::trans_block::h7863f52e43c06945CmY
  19:     0x7f92bf3c8e80 - middle::trans::base::trans_closure::ha6113b08bc17793cAAe
  20:     0x7f92bf303840 - middle::trans::base::trans_fn::hb06093de10539f75nMe
  21:     0x7f92bf2feae0 - middle::trans::base::trans_item::hd285e8d770107116n4e
  22:     0x7f92bf2feae0 - middle::trans::base::trans_item::hd285e8d770107116n4e
  23:     0x7f92bf2feae0 - middle::trans::base::trans_item::hd285e8d770107116n4e
  24:     0x7f92bf2feae0 - middle::trans::base::trans_item::hd285e8d770107116n4e
  25:     0x7f92bf3d3b20 - middle::trans::base::trans_crate::hc2a811966285ee0aOYf
  26:     0x7f92bf8083d0 - driver::driver::phase_4_translate_to_llvm::h06435122d6db3633XHB
  27:     0x7f92bf7ffbf0 - driver::driver::compile_input::ha48623fcead7bd7e5jB
  28:     0x7f92bf893890 - driver::run_compiler::h0b258b9c360b66ddjRE
  29:     0x7f92bf8937a0 - driver::main_args::closure.141085
  30:     0x7f92bf8a7dc0 - task::TaskBuilder<S>::try_future::closure.142270
  31:     0x7f92bf8a7bc0 - task::TaskBuilder<S>::spawn_internal::closure.142247
  32:     0x7f92c03120b0 - task::spawn_opts::closure.8370
  33:     0x7f92bebc2490 - rust_try_inner
  34:     0x7f92bebc2480 - rust_try
  35:     0x7f92beb66cf0 - unwind::try::hea1640d217ab7457H7d
  36:     0x7f92beb66a90 - task::Task::run::hb52aad40b24ebf23Ndd
  37:     0x7f92c0311e10 - task::spawn_opts::closure.8316
  38:     0x7f92beb68930 - thread::thread_start::h741158bbfb282555MCd
  39:     0x7f92bde120c0 - start_thread
  40:     0x7f92be82ff89 - __clone
  41:                0x0 - <unknown>
