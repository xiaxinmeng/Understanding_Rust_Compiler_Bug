
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'unsized_part_of_type failed even though ty is unsized', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_trans/trans/common.rs:166

stack backtrace:
   1:     0x7f197e3b3750 - sys::backtrace::write::h49b06c4e5fa1765dZTA
   2:     0x7f197e3d83e0 - failure::on_fail::h128a61ca90111ec2JFJ
   3:     0x7f197e330350 - rt::unwind::begin_unwind_inner::h142cc6242e7988934jJ
   4:     0x7f197d002e90 - rt::unwind::begin_unwind::h13556005847138426690
   5:     0x7f197d0dc3e0 - trans::common::unsized_part_of_type::h4971bfd82c960f177Sk
   6:     0x7f197d10c5e0 - trans::type_of::type_of::type_of_unsize_info::hc9a1771493b5585eB9o
   7:     0x7f197d05fd20 - trans::type_of::type_of::h45cc5a0ffa907939h9o
   8:     0x7f197d08ef50 - trans::base::alloc_ty::h23dd60deb6e81842ODt
   9:     0x7f197d1563c0 - trans::_match::mk_binding_alloca::h10681521704565058413
  10:     0x7f197d053f30 - trans::base::init_local::hca973ce50444bab8Cpt
  11:     0x7f197d054ea0 - trans::controlflow::trans_block::hfb3d01e31adc3da5Qee
  12:     0x7f197d120030 - trans::base::trans_closure::hd99a1fe118eadd85ofu
  13:     0x7f197d040670 - trans::base::trans_fn::hb1bc39659fd54f79Equ
  14:     0x7f197d03b6f0 - trans::base::trans_item::hf71bcbd6c9a9283ftPu
  15:     0x7f197d1275f0 - trans::base::trans_crate::h3465dd947f56ed22NMv
  16:     0x7f197e9e80e0 - driver::phase_4_translate_to_llvm::h130c1ed11c0c6b37wPa
  17:     0x7f197e9c1190 - driver::compile_input::ha0cf61571709f853Eba
  18:     0x7f197ea922b0 - run_compiler::h331ebfa0d68f5ee75bc
  19:     0x7f197ea90910 - thunk::F.Invoke<A, R>::invoke::h10527493330292840218
  20:     0x7f197ea8f800 - rt::unwind::try::try_fn::h13426443671359034537
  21:     0x7f197e444a70 - rust_try_inner
  22:     0x7f197e444a60 - rust_try
  23:     0x7f197ea8fab0 - thunk::F.Invoke<A, R>::invoke::h16144672682574185653
  24:     0x7f197e3c4050 - sys::thread::thread_start::h76415ad3898ce7eaaOE
  25:     0x7f19783c7250 - start_thread
  26:     0x7f197dfb8219 - clone
  27:                0x0 - <unknown>
