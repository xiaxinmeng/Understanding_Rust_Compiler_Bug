
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: closure, ty_unboxed_closure(syntax::ast::DefId{krate: 0u32, node: 17u32}, ReScope(7u32))
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', rust/src/libsyntax/diagnostic.rs:169

stack backtrace:
   1:     0x7f117b3d7ef0 - rt::backtrace::imp::write::h565d36b3d31f4204EAq
   2:     0x7f117b3db080 - failure::on_fail::hdf6d221f6a9a6b896Vq
   3:     0x7f117f962f40 - unwind::begin_unwind_inner::h091583a8cc9e94acIxd
   4:     0x7f117ca074f0 - unwind::begin_unwind::h6201309080514735908
   5:     0x7f117ca07bb0 - diagnostic::Handler::bug::h3becd3518f2c97d9ISF
   6:     0x7f117ffb6010 - driver::session::Session::bug::ha5f6c5d18117a203Mrx
   7:     0x7f11804522a0 - middle::trans::debuginfo::TypeMap::get_unique_type_id_of_type::h3e7f30471c698b62Pip
   8:     0x7f118045b170 - middle::trans::debuginfo::type_metadata::h6d519db29655d0f07ur
   9:     0x7f1180402990 - middle::trans::debuginfo::create_function_debug_context::h45ff9bca3ca8b243L1p
  10:     0x7f118037c540 - middle::trans::base::new_fn_ctxt::hd570c7c13b27ff15Qpe
  11:     0x7f1180406c00 - middle::trans::base::trans_closure::hb146f0d5d34464919Qe
  12:     0x7f11803adf90 - middle::trans::closure::trans_unboxed_closure::h5e34114f89399a1df6i
  13:     0x7f11803983a0 - middle::trans::expr::trans_rvalue_dps_unadjusted::h1a6f65d9f1a5aab5I73
  14:     0x7f118035bec0 - middle::trans::expr::trans_into::h813b1c1f5fd2fb7cLJ2
  15:     0x7f1180434650 - middle::trans::_match::store_local::closure.127381
  16:     0x7f1180434430 - middle::trans::_match::mk_binding_alloca::h16065912786747425502
  17:     0x7f11803ff9b0 - middle::trans::_match::store_local::hc2ec5eb01922eeb5pfi
  18:     0x7f118035b480 - middle::trans::base::init_local::h4138afe3203e44aeJZd
  19:     0x7f118035a840 - middle::trans::controlflow::trans_stmt::hcfdd82d0f99118a4cSY
  20:     0x7f118035c330 - middle::trans::controlflow::trans_block::h5993a8a634369706iXY
  21:     0x7f1180406c00 - middle::trans::base::trans_closure::hb146f0d5d34464919Qe
  22:     0x7f118034e3a0 - middle::trans::base::trans_fn::hafff09b5964c0b32x2e
  23:     0x7f118034b8b0 - middle::trans::base::trans_item::h25b7af64bcdaa99cQlf
  24:     0x7f1180411d30 - middle::trans::base::trans_crate::h38bf11f2c58839936lg
  25:     0x7f118084faa0 - driver::driver::phase_4_translate_to_llvm::hac5e756b10d7155eqSw
  26:     0x7f1180847250 - driver::driver::compile_input::hffca3f088d7d5b21lpw
  27:     0x7f11808c9440 - driver::run_compiler::hd66340705fdb422fCfA
  28:     0x7f11808c9320 - driver::run::closure.146486
  29:     0x7f117ffe3f40 - task::TaskBuilder<S>::try_future::closure.101300
  30:     0x7f117ffe3d30 - task::TaskBuilder<S>::spawn_internal::closure.101271
  31:     0x7f117fcb5550 - task::spawn_opts::closure.8455
  32:     0x7f117f9c3070 - rust_try_inner
  33:     0x7f117f9c3060 - rust_try
  34:     0x7f117f960a00 - unwind::try::h1a920b60f039eb8cqmd
  35:     0x7f117f960890 - task::Task::run::hb8390ba45e7ec26a4Bc
  36:     0x7f117fcb52c0 - task::spawn_opts::closure.8395
  37:     0x7f117f961fe0 - thread::thread_start::h00e0085f8f5c32d4fWc
  38:     0x7f117a788250 - start_thread
  39:     0x7f117f63b3b9 - clone
  40:                0x0 - <unknown>
