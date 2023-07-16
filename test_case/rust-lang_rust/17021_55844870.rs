
RUST_BACKTRACE=1 cargo test
   Compiling rust-sqlite v0.0.1 (file:///home/connolly/projects/rust-sqlite)
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: closure, ty_unboxed_closure(syntax::ast::DefId{krate: 0u32, node: 837u32}, ReScope(836u32))
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: closure, ty_unboxed_closure(syntax::ast::DefId{krate: 0u32, node: 838u32}, ReScope(837u32))
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ast_util.rs:694

stack backtrace:
   1:     0x7f556e08be10 - rt::backtrace::imp::write::h58c507e3d62ba470EGq
   2:     0x7f556e08efd0 - failure::on_fail::h4e8b8c41435d737861q
   3:     0x7f556e875170 - unwind::begin_unwind_inner::h1fb7b215753db772MTd
   4:     0x7f556a793ba0 - unwind::begin_unwind::h13599161861070157624
   5:     0x7f556a794340 - diagnostic::Handler::bug::h5acd6ba1b54e525eXND
   6:     0x7f556ec5cf20 - driver::session::Session::bug::he4cfdccd626f2833Biv
   7:     0x7f556f1370c0 - middle::trans::debuginfo::TypeMap::get_unique_type_id_of_type::h5512f66d43f3b795VTm
   8:     0x7f556f142a70 - middle::trans::debuginfo::type_metadata::h7d666eda32ab8213gdp
   9:     0x7f556f0de540 - middle::trans::debuginfo::create_function_debug_context::hd88c85dea33273c8oEn
  10:     0x7f556f055b30 - middle::trans::base::new_fn_ctxt::hd01b77407a4d5fbeU5b
  11:     0x7f556f0e2bf0 - middle::trans::base::trans_closure::hed18cfef32647cfchxc
  12:     0x7f556f08e650 - middle::trans::closure::trans_unboxed_closure::h8b81ff097b4bcc84rKg
  13:     0x7f556f077240 - middle::trans::expr::trans_rvalue_dps_unadjusted::h30466267ac61ee80ZL1
  14:     0x7f556f032670 - middle::trans::expr::trans_into::h7f0f7222768a8208Ss0
  15:     0x7f556f080ee0 - middle::trans::expr::trans_uniq_expr::h8a7464811676935ePv2
  16:     0x7f556f081a60 - middle::trans::expr::trans_unary::hf5e9e68b6559378fJr2
  17:     0x7f556f0763b0 - middle::trans::expr::trans_unadjusted::h47e1e28f94cd2d83Sc1
  18:     0x7f556f033bb0 - middle::trans::expr::trans::h37c06598b4b77312Cw0
  19:     0x7f556f06f0a0 - middle::trans::callee::trans_args::h80ca0184ed615ab5R1Z
  20:     0x7f556f03b3b0 - middle::trans::callee::trans_call_inner::hf3b9652a55858086WGZ
  21:     0x7f556f068b90 - middle::trans::callee::trans_call::h859116d764b84d33fBZ
  22:     0x7f556f077240 - middle::trans::expr::trans_rvalue_dps_unadjusted::h30466267ac61ee80ZL1
  23:     0x7f556f032670 - middle::trans::expr::trans_into::h7f0f7222768a8208Ss0
  24:     0x7f556f032b50 - middle::trans::controlflow::trans_block::h42a66b0844254317VvW
  25:     0x7f556f0e2bf0 - middle::trans::base::trans_closure::hed18cfef32647cfchxc
  26:     0x7f556f020640 - middle::trans::base::trans_fn::h09596f78ce96e9aauIc
  27:     0x7f556f0e6860 - middle::trans::meth::trans_impl::hcbb399ecf3aaea18LIh
  28:     0x7f556f01c280 - middle::trans::base::trans_item::hc56287d0be1f09e3D1c
  29:     0x7f556f01c280 - middle::trans::base::trans_item::hc56287d0be1f09e3D1c
  30:     0x7f556f0ef150 - middle::trans::base::trans_crate::hf87901a2dda586fbwZd
  31:     0x7f556f5097c0 - driver::driver::phase_4_translate_to_llvm::hecec95a14eb6141atJu
  32:     0x7f556f500a10 - driver::driver::compile_input::hf2ae38f6706178a6vhu
  33:     0x7f556f5b1150 - driver::run_compiler::h8454bca9b6a0718317x
  34:     0x7f556f5b1030 - driver::main_args::closure.147620
  35:     0x7f556ec8b680 - task::TaskBuilder<S>::try_future::closure.100301
  36:     0x7f556ec8b470 - task::TaskBuilder<S>::spawn_internal::closure.100272
  37:     0x7f556feda6d0 - task::spawn_opts::closure.8426
  38:     0x7f556e8cc7e0 - rust_try_inner
  39:     0x7f556e8cc7d0 - rust_try
  40:     0x7f556e872750 - unwind::try::hc32837413dc79d50uId
  41:     0x7f556e8725b0 - task::Task::run::he2343ec4d9f3287dfYc
  42:     0x7f556feda440 - task::spawn_opts::closure.8366
  43:     0x7f556e8741a0 - thread::thread_start::h71dd1e21d4d03408rid
  44:     0x7f556db480c0 - start_thread
  45:     0x7f556e53af89 - __clone
  46:                0x0 - <unknown>

Build failed, waiting for other jobs to finish...

