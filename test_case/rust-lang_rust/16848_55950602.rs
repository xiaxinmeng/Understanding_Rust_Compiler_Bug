
src/main.rs:85:25: 85:51 warning: use of deprecated item: use into_iter, #[warn(deprecated)] on by default
src/main.rs:85     for serverconfig in config.servers.move_iter() {
                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~

error: internal compiler error: trying to take the sizing type of str, an unsized type
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:694

stack backtrace:
   1:     0x7f2e84b7eb80 - rt::backtrace::imp::write::hadb95304cd6b59a2iHq
   2:     0x7f2e84b81d40 - failure::on_fail::h88a79ad90b452a8dK2q
   3:     0x7f2e85331f60 - unwind::begin_unwind_inner::hd810389f34a8dc8bMTd
   4:     0x7f2e80f92570 - unwind::begin_unwind::h883036773483107899
   5:     0x7f2e80f92d10 - diagnostic::Handler::bug::ha03a127ac108cb809ND
   6:     0x7f2e85712040 - driver::session::Session::bug::hd8091c59ae3b07effyx
   7:     0x7f2e85af8300 - middle::trans::type_of::sizing_type_of::h12d44c3ad0a509b9NP9
   8:     0x7f2e85b27670 - middle::trans::meth::get_vtable::h71ff864a2270cfc0kOk
   9:     0x7f2e85b27000 - middle::trans::expr::apply_adjustments::unsized_info::h8041c4d8f81d7431Gl3
  10:     0x7f2e85b2a0e0 - middle::trans::expr::apply_adjustments::unsize_expr::closure.124122
  11:     0x7f2e85b2a220 - middle::trans::expr::apply_adjustments::into_fat_ptr::h012bc185249de733Zs3
  12:     0x7f2e85b25db0 - middle::trans::expr::apply_adjustments::apply_autoref::hff612d8b5d90a70cD82
  13:     0x7f2e85b25db0 - middle::trans::expr::apply_adjustments::apply_autoref::hff612d8b5d90a70cD82
  14:     0x7f2e85ae5960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  15:     0x7f2e85ae4420 - middle::trans::expr::trans_into::hffcd0924abd4fea27S2
  16:     0x7f2e85bc9be0 - middle::trans::tvec::write_content::hd73973faa21b5330vPj
  17:     0x7f2e85b2d780 - middle::trans::tvec::trans_slice_vec::h65d0df732c3725d4PEj
  18:     0x7f2e85b21f10 - middle::trans::expr::trans_unadjusted::hb5708bd9b995e003LD3
  19:     0x7f2e85ae5960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  20:     0x7f2e85b1ab20 - middle::trans::callee::trans_args::h40ba956d7101922f4r2
  21:     0x7f2e85aec7c0 - middle::trans::callee::trans_call_inner::hea87a946e0cffa34961
  22:     0x7f2e85b155b0 - middle::trans::callee::trans_method_call::h5a0bf5976bce0583C21
  23:     0x7f2e85b22da0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h6f9547e6400812f9Sc4
  24:     0x7f2e85b21f10 - middle::trans::expr::trans_unadjusted::hb5708bd9b995e003LD3
  25:     0x7f2e85ae5960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  26:     0x7f2e85b1ab20 - middle::trans::callee::trans_args::h40ba956d7101922f4r2
  27:     0x7f2e85aec7c0 - middle::trans::callee::trans_call_inner::hea87a946e0cffa34961
  28:     0x7f2e85b155b0 - middle::trans::callee::trans_method_call::h5a0bf5976bce0583C21
  29:     0x7f2e85b22da0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h6f9547e6400812f9Sc4
  30:     0x7f2e85b21f10 - middle::trans::expr::trans_unadjusted::hb5708bd9b995e003LD3
  31:     0x7f2e85ae5960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  32:     0x7f2e85b1ab20 - middle::trans::callee::trans_args::h40ba956d7101922f4r2
  33:     0x7f2e85aec7c0 - middle::trans::callee::trans_call_inner::hea87a946e0cffa34961
  34:     0x7f2e85b155b0 - middle::trans::callee::trans_method_call::h5a0bf5976bce0583C21
  35:     0x7f2e85b22da0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h6f9547e6400812f9Sc4
  36:     0x7f2e85b21f10 - middle::trans::expr::trans_unadjusted::hb5708bd9b995e003LD3
  37:     0x7f2e85ae5960 - middle::trans::expr::trans::h139fd41d89b98122RW2
  38:     0x7f2e85b2c9c0 - middle::trans::expr::trans_binary::h5983843557b2865dyf5
  39:     0x7f2e85b21f10 - middle::trans::expr::trans_unadjusted::hb5708bd9b995e003LD3
  40:     0x7f2e85ae4420 - middle::trans::expr::trans_into::hffcd0924abd4fea27S2
  41:     0x7f2e85ae4900 - middle::trans::controlflow::trans_block::h697fa29e838af1025ZY
  42:     0x7f2e85b92810 - middle::trans::base::trans_closure::hf5f648fb424f9c93r1e
  43:     0x7f2e85ad6b60 - middle::trans::base::trans_fn::h17dae15f71272d8eEcf
  44:     0x7f2e85b96490 - middle::trans::meth::trans_impl::h5b1ebd2bcbfeba03Xck
  45:     0x7f2e85ad3780 - middle::trans::base::trans_item::h7f0dc64cc8095e95Nvf
  46:     0x7f2e85ad3780 - middle::trans::base::trans_item::h7f0dc64cc8095e95Nvf
  47:     0x7f2e85ad3780 - middle::trans::base::trans_item::h7f0dc64cc8095e95Nvf
  48:     0x7f2e85b9e780 - middle::trans::base::trans_crate::h40ab1d2d17a4f286Gtg
  49:     0x7f2e85f98330 - driver::driver::phase_4_translate_to_llvm::h9eab234cedcf7a6b7Yw
  50:     0x7f2e85f8f590 - driver::driver::compile_input::h06caa00e975ed0209ww
  51:     0x7f2e860126b0 - driver::run_compiler::h30663c3fdd061b12FnA
  52:     0x7f2e86012590 - driver::main_args::closure.146245
  53:     0x7f2e85740740 - task::TaskBuilder<S>::try_future::closure.101946
  54:     0x7f2e85740530 - task::TaskBuilder<S>::spawn_internal::closure.101917
  55:     0x7f2e8691e590 - task::spawn_opts::closure.8444
  56:     0x7f2e85389480 - rust_try_inner
  57:     0x7f2e85389470 - rust_try
  58:     0x7f2e8532f560 - unwind::try::h985905ef4937434euId
  59:     0x7f2e8532f3c0 - task::Task::run::h3a7021021ab3297dfYc
  60:     0x7f2e8691e300 - task::spawn_opts::closure.8384
  61:     0x7f2e85330fb0 - thread::thread_start::h67b84fc708de7e36rid
  62:     0x7f2e84645100 - start_thread
  63:     0x7f2e8500c929 - clone
  64:                0x0 - <unknown>
