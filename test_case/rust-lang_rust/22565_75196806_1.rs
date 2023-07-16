
error: internal compiler error: argument to function with "rust-call" ABI is neither a tuple nor unit
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:189

stack backtrace:
   1:     0x7fa130b68a60 - sys::backtrace::write::h17046f691c944fd6wlB
   2:     0x7fa130b91650 - failure::on_fail::hc6163ed550dbd15faOK
   3:     0x7fa130ad7e10 - rt::unwind::begin_unwind_inner::hf466231ca0fbfbbdvsK
   4:     0x7fa12de449c0 - rt::unwind::begin_unwind::h15994696547183619422
   5:     0x7fa12de45180 - diagnostic::Handler::bug::h1bc3ded4b9cf6157abF
   6:     0x7fa12eb47030 - session::Session::bug::h0d2283befd4ab8993ds
   7:     0x7fa12f892bd0 - trans::type_of::type_of_rust_fn::h653f330b805e918apWo
   8:     0x7fa12f89aac0 - trans::base::decl_rust_fn::h55f15a19d1270dfe9gs
   9:     0x7fa12f7d3a20 - trans::monomorphize::monomorphic_fn::closure.41094
  10:     0x7fa12f7c67d0 - trans::monomorphize::monomorphic_fn::hf0c9202f660b2747RAd
  11:     0x7fa12f80a020 - trans::callee::trans_fn_ref_with_substs::hf1f1aa6751244623DDg
  12:     0x7fa12f808d00 - trans::meth::trans_static_method_callee::h4e4e9f0617eb5a10shz
  13:     0x7fa12f805b30 - trans::callee::trans::h2ae2f86c822ea322rgg
  14:     0x7fa12f81c590 - trans::callee::trans_call_inner::h12505348671615469836
  15:     0x7fa12f824dd0 - trans::expr::trans_rvalue_dps_unadjusted::h2a5828c13afa2678YZi
  16:     0x7fa12f7dad00 - trans::expr::trans_into::h8fd4f8a9bb5dccd8YFh
  17:     0x7fa12f7da1a0 - trans::controlflow::trans_stmt_semi::hccf4a17010754a60Zde
  18:     0x7fa12f7db310 - trans::controlflow::trans_block::hc124f866353eafbdQee
  19:     0x7fa12f8a8a20 - trans::base::trans_closure::h6c5fc36e7a33e12eofu
  20:     0x7fa12f7c6270 - trans::base::trans_fn::h76390f46f62f07cdEqu
  21:     0x7fa12f7c1410 - trans::base::trans_item::ha4ea07b91896bde9tPu
  22:     0x7fa12f8b00f0 - trans::base::trans_crate::hc435c9fdb6f30ff4NMv
  23:     0x7fa1311b3250 - driver::phase_4_translate_to_llvm::hb02bd998f41b29a0wPa
  24:     0x7fa13118c170 - driver::compile_input::h91ff1ed16e22d05cEba
  25:     0x7fa13125fea0 - run_compiler::h9c581f85e29a6b265bc
  26:     0x7fa13125e500 - thunk::F.Invoke<A, R>::invoke::h5155567596601403298
  27:     0x7fa13125d3f0 - rt::unwind::try::try_fn::h5629523784837248188
  28:     0x7fa130bfdf20 - rust_try_inner
  29:     0x7fa130bfdf10 - rust_try
  30:     0x7fa13125d6a0 - thunk::F.Invoke<A, R>::invoke::h7807062886684583533
  31:     0x7fa130b7d190 - sys::thread::thread_start::h57c9c75c73c301cbfVF
  32:     0x7fa12ab1f2b0 - start_thread
  33:     0x7fa13075e249 - __clone
  34:                0x0 - <unknown>
