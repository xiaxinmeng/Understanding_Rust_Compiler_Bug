 text
thread 'rustc' panicked at 'Box<Any>', ../src/libsyntax/diagnostic.rs:253
stack backtrace:
   1:      0x3d806b1d2e0 - sys::backtrace::tracing::imp::write::h7f8bffe1677463dfM0t
   2:      0x3d806b23e25 - panicking::log_panic::_<closure>::closure.41442
   3:      0x3d806b238e7 - panicking::log_panic::h1e3f4d9fc8696c4017x
   4:      0x3d806aecec3 - sys_common::unwind::begin_unwind_inner::hf2f35ad4f77e2e7dlTs
   5:      0x3d802ec7587 - sys_common::unwind::begin_unwind::begin_unwind::h4516416896677524385
   6:      0x3d802ec8436 - diagnostic::_<impl>::bug::h682320cf3b2d4b9cHqC
   7:      0x3d8042c5363 - session::_<impl>::bug::h2dc754e0eedce01brlr
   8:      0x3d80580ae0d - trans::consts::const_expr::h89f3dab057d316c9rGt
   9:      0x3d80580cf57 - trans::consts::get_const_expr_as_global::h57432be0004f7cc1oAt
  10:      0x3d805778666 - trans::expr::trans::hfda119ccec83bbfdKWB
  11:      0x3d8058299c9 - trans::expr::trans_to_lvalue::hdbb3cbfb9c2982e5EqC
  12:      0x3d80578eeb0 - trans::base::init_local::h8b4532f4a564f33eP2h
  13:      0x3d8057a9382 - trans::controlflow::trans_block::h21c1e30fbaaa7ae7HBw
  14:      0x3d8057a5a67 - trans::base::trans_closure::hd865582de7ec744fFQi
  15:      0x3d8057a9cfc - trans::base::trans_fn::haa8ae223ea96d86dr0i
  16:      0x3d8057ae21f - trans::base::trans_item::hab01204c7966b68broj
  17:      0x3d8057ba154 - trans::base::trans_crate::h7493f3a97149ae95e8j
  18:      0x3d80707fce1 - driver::phase_4_translate_to_llvm::hc1a37d86c018987fdTa
  19:      0x3d807079b86 - driver::phase_3_run_analysis_passes::_<closure>::closure.25661
  20:      0x3d807058993 - middle::ty::context::_<impl>::create_and_enter::create_and_enter::h15469618450242186396
  21:      0x3d8070543d1 - driver::phase_3_run_analysis_passes::h15237454356639212767
  22:      0x3d8070296e9 - driver::compile_input::hf8753496fe664ae5hca
  23:      0x3d80701b90b - run_compiler::h2b9f78b23d87ce7cOwc
  24:      0x3d8070185c6 - sys_common::unwind::try::try_fn::try_fn::h2643310051023045133
  25:      0x3d806b1af78 - __rust_try
  26:      0x3d806b1276b - sys_common::unwind::try::inner_try::h7275435b1036ba69TPs
  27:      0x3d807018920 - boxed::_<impl>::call_box::call_box::h13129466644103381480
  28:      0x3d806b22333 - sys::thread::_<impl>::new::thread_start::ha7ee5c311da0b8cdeax
  29:      0x3d7ffae94a3 - start_thread
  30:      0x3d8067b413c - clone
  31:                0x0 - <unknown>
