 rust
ice.rs:5:34: 5:35 error: internal compiler error: unbound path path(T)
ice.rs:5     fn get_bar() -> <Self as Foo<T>>::Bar;
                                          ^
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /var/tmp/paludis/build/dev-lang-rust-scm/work/rust-scm/src/libsyntax/ast_util.rs:751

stack backtrace:
   1:     0x7f2c448aebd0 - rt::backtrace::imp::write::hc0f5116acb0fa00fiHq
   2:     0x7f2c448b1d90 - <unknown>
   3:     0x7f2c4506dd30 - unwind::begin_unwind_inner::hf187fcd15f4370d8MTd
   4:     0x7f2c43337e90 - <unknown>
   5:     0x7f2c43337de0 - diagnostic::SpanHandler::span_bug::hca7ebac09aed6025CPE
   6:     0x7f2c456210a0 - driver::session::Session::span_bug::h9bbd70985dcc1a1cYVx
   7:     0x7f2c456f77e0 - middle::typeck::astconv::ast_ty_to_prim_ty::h5f832d51acb46f2dMY0
   8:     0x7f2c45b87670 - <unknown>
   9:     0x7f2c45a29c90 - <unknown>
  10:     0x7f2c45b8bf30 - <unknown>
  11:     0x7f2c45b8be20 - <unknown>
  12:     0x7f2c45b8e980 - <unknown>
  13:     0x7f2c45b87670 - <unknown>
  14:     0x7f2c45b766a0 - <unknown>
  15:     0x7f2c45b58090 - <unknown>
  16:     0x7f2c45b31330 - middle::typeck::collect::convert::hde892ddbd942c261z3f
  17:     0x7f2c45b2a110 - middle::typeck::collect::collect_item_types::h34839250c10cab43q4e
  18:     0x7f2c45460220 - <unknown>
  19:     0x7f2c45cfa650 - middle::typeck::check_crate::h0dbc94139bb624b2WIk
  20:     0x7f2c45d674a0 - driver::driver::phase_3_run_analysis_passes::h777eed77d7f01cb9Ucx
  21:     0x7f2c45d62910 - driver::driver::compile_input::h1e1f68cd3c0db39eRTw
  22:     0x7f2c45de7990 - <unknown>
  23:     0x7f2c45de7870 - <unknown>
  24:     0x7f2c45479a10 - <unknown>
  25:     0x7f2c45479800 - <unknown>
  26:     0x7f2c467495b0 - <unknown>
  27:     0x7f2c450bcb00 - <unknown>
  28:     0x7f2c450bcaf0 - rust_try
  29:     0x7f2c4506b330 - unwind::try::h3fd431c8ae9db728uId
  30:     0x7f2c4506b190 - task::Task::run::h8df699a6a6fb1c6ecYc
  31:     0x7f2c46749320 - <unknown>
  32:     0x7f2c4506cd80 - <unknown>
  33:     0x7f2c44371fb0 - start_thread
  34:     0x7f2c44d3bf99 - clone
  35:                0x0 - <unknown>
