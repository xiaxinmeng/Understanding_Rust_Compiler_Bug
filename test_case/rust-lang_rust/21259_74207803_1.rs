
error: internal compiler error: fictitious type [type error] in sizing_type_of()
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:189

stack backtrace:
   1:     0x7f745d486750 - sys::backtrace::write::h49b06c4e5fa1765dZTA
   2:     0x7f745d4ab3e0 - failure::on_fail::h128a61ca90111ec2JFJ
   3:     0x7f745d403350 - rt::unwind::begin_unwind_inner::h142cc6242e7988934jJ
   4:     0x7f745a7bbd10 - rt::unwind::begin_unwind::h6758701799544434784
   5:     0x7f745a7bc4d0 - diagnostic::Handler::bug::h6b59efb60e2e8c41q9E
   6:     0x7f745b4acf80 - session::Session::bug::h91c4ad8e6f835df8c8r
   7:     0x7f745c12ea50 - trans::type_of::sizing_type_of::h616bd8b16165aa4eE1o
   8:     0x7f745c14f390 - trans::common::type_is_immediate::h0c9a91d259ddb754yZk
   9:     0x7f745c1dd720 - trans::type_of::type_of_rust_fn::h9c5be9dff297850dpWo
  10:     0x7f745c1e5240 - trans::base::decl_rust_fn::hc8875771a49123769gs
  11:     0x7f745c120610 - trans::monomorphize::monomorphic_fn::closure.41058
  12:     0x7f745c113bd0 - trans::monomorphize::monomorphic_fn::h8d7ab143ba747de3RAd
  13:     0x7f745c156110 - trans::callee::trans_fn_ref_with_substs::h5d21745cc9bd25c4DDg
  14:     0x7f745c16ae90 - trans::meth::trans_method_callee::h808997605d1e44d99bz
  15:     0x7f745c18c630 - trans::expr::trans_overloaded_op::h0ba9ff848f516939X7j
  16:     0x7f745c1a8da0 - trans::expr::deref_once::h172799ba74706b76QAk
  17:     0x7f745c187440 - trans::expr::trans_unary::h00268a8f6f7d4e54oIj
  18:     0x7f745c16fd10 - trans::expr::trans_unadjusted::h431aa1c51b9ba5e9tri
  19:     0x7f745c128a20 - trans::expr::trans::hb58ad7514ee2fb5fsJh
  20:     0x7f745c188500 - trans::expr::trans_addr_of::h1ae47fbb9dd8c4d8aPj
  21:     0x7f745c16fd10 - trans::expr::trans_unadjusted::h431aa1c51b9ba5e9tri
  22:     0x7f745c127890 - trans::expr::trans_into::h28252bdf2f790a2fYFh
  23:     0x7f745c126d30 - trans::controlflow::trans_stmt_semi::h2d011dfa6faacbfbZde
  24:     0x7f745c127ea0 - trans::controlflow::trans_block::hfb3d01e31adc3da5Qee
  25:     0x7f745c1f3030 - trans::base::trans_closure::hd99a1fe118eadd85ofu
  26:     0x7f745c113670 - trans::base::trans_fn::hb1bc39659fd54f79Equ
  27:     0x7f745c10e6f0 - trans::base::trans_item::hf71bcbd6c9a9283ftPu
  28:     0x7f745c1fa5f0 - trans::base::trans_crate::h3465dd947f56ed22NMv
  29:     0x7f745dabb0e0 - driver::phase_4_translate_to_llvm::h130c1ed11c0c6b37wPa
  30:     0x7f745da94190 - driver::compile_input::ha0cf61571709f853Eba
  31:     0x7f745db652b0 - run_compiler::h331ebfa0d68f5ee75bc
  32:     0x7f745db63910 - thunk::F.Invoke<A, R>::invoke::h10527493330292840218
  33:     0x7f745db62800 - rt::unwind::try::try_fn::h13426443671359034537
  34:     0x7f745d517a70 - rust_try_inner
  35:     0x7f745d517a60 - rust_try
  36:     0x7f745db62ab0 - thunk::F.Invoke<A, R>::invoke::h16144672682574185653
  37:     0x7f745d497050 - sys::thread::thread_start::h76415ad3898ce7eaaOE
  38:     0x7f745749a250 - start_thread
  39:     0x7f745d08b219 - clone
  40:                0x0 - <unknown>
