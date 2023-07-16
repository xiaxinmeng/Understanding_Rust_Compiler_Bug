 rust
error: internal compiler error: trying to take the sizing type of str, an unsized type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:694

stack backtrace:
   1:     0x7f9a5d8f0b80 - rt::backtrace::imp::write::hadb95304cd6b59a2iHq
   2:     0x7f9a5d8f3d40 - failure::on_fail::h88a79ad90b452a8dK2q
   3:     0x7f9a5e0a0f60 - unwind::begin_unwind_inner::hd810389f34a8dc8bMTd
   4:     0x7f9a59d06570 - unwind::begin_unwind::h883036773483107899
   5:     0x7f9a59d06d10 - diagnostic::Handler::bug::ha03a127ac108cb809ND
   6:     0x7f9a5e481040 - driver::session::Session::bug::hd8091c59ae3b07effyx
   7:     0x7f9a5e867300 - middle::trans::type_of::sizing_type_of::h12d44c3ad0a509b9NP9
   8:     0x7f9a5e896670 - middle::trans::meth::get_vtable::h71ff864a2270cfc0kOk
   9:     0x7f9a5e896000 - middle::trans::expr::apply_adjustments::unsized_info::h8041c4d8f81d7431Gl3
  10:     0x7f9a5e8990e0 - middle::trans::expr::apply_adjustments::unsize_expr::closure.124122
  11:     0x7f9a5e899220 - middle::trans::expr::apply_adjustments::into_fat_ptr::h012bc185249de733Zs3
  12:     0x7f9a5e894db0 - middle::trans::expr::apply_adjustments::apply_autoref::hff612d8b5d90a70cD82
  13:     0x7f9a5e894db0 - middle::trans::expr::apply_adjustments::apply_autoref::hff612d8b5d90a70cD82
  14:     0x7f9a5e854960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  15:     0x7f9a5e853420 - middle::trans::expr::trans_into::hffcd0924abd4fea27S2
  16:     0x7f9a5e938be0 - middle::trans::tvec::write_content::hd73973faa21b5330vPj
  17:     0x7f9a5e89c780 - middle::trans::tvec::trans_slice_vec::h65d0df732c3725d4PEj
  18:     0x7f9a5e890f10 - middle::trans::expr::trans_unadjusted::hb5708bd9b995e003LD3
  19:     0x7f9a5e854960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  20:     0x7f9a5e8f9ce0 - middle::trans::_match::store_local::hbfa7bf297960a9f5jni
  21:     0x7f9a5e852a20 - middle::trans::base::init_local::h122a5d8fb0284157dae
  22:     0x7f9a5e851f80 - middle::trans::controlflow::trans_stmt::hfd63a5c43efa1d1f9UY
  23:     0x7f9a5e853900 - middle::trans::controlflow::trans_block::h697fa29e838af1025ZY
  24:     0x7f9a5e901810 - middle::trans::base::trans_closure::hf5f648fb424f9c93r1e
  25:     0x7f9a5e845b60 - middle::trans::base::trans_fn::h17dae15f71272d8eEcf
  26:     0x7f9a5e842780 - middle::trans::base::trans_item::h7f0dc64cc8095e95Nvf
  27:     0x7f9a5e90d780 - middle::trans::base::trans_crate::h40ab1d2d17a4f286Gtg
  28:     0x7f9a5ed07330 - driver::driver::phase_4_translate_to_llvm::h9eab234cedcf7a6b7Yw
  29:     0x7f9a5ecfe590 - driver::driver::compile_input::h06caa00e975ed0209ww
  30:     0x7f9a5ed816b0 - driver::run_compiler::h30663c3fdd061b12FnA
  31:     0x7f9a5ed81590 - driver::main_args::closure.146245
  32:     0x7f9a5e4af740 - task::TaskBuilder<S>::try_future::closure.101946
  33:     0x7f9a5e4af530 - task::TaskBuilder<S>::spawn_internal::closure.101917
  34:     0x7f9a5f68d590 - task::spawn_opts::closure.8444
  35:     0x7f9a5e0f8480 - rust_try_inner
  36:     0x7f9a5e0f8470 - rust_try
  37:     0x7f9a5e09e560 - unwind::try::h985905ef4937434euId
  38:     0x7f9a5e09e3c0 - task::Task::run::h3a7021021ab3297dfYc
  39:     0x7f9a5f68d300 - task::spawn_opts::closure.8384
  40:     0x7f9a5e09ffb0 - thread::thread_start::h67b84fc708de7e36rid
  41:     0x7f9a5d3b7250 - start_thread
  42:     0x7f9a5dd793b9 - clone
  43:                0x0 - <unknown>
