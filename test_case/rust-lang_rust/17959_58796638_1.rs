
x.rs:1:1: 1:1 error: internal compiler error: Encountered errors `[FulfillmentError(Obligation(trait_ref=<<generic #0> as core::kinds::Sized>,depth=1),Unimplemented)]` fulfilling `<*mut GBox<Bar+'static> as core::ptr::RawPtr<GBox<Bar+'static>>>` during trans
x.rs:1 #![feature(unsafe_destructor)]
       ^
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'Box<Any>', /private/tmp/rust-jaR1Yf/src/libsyntax/diagnostic.rs:113

stack backtrace:
   1:        0x10b9bdd99 - rt::backtrace::imp::write::h273044213f8a2894OLq
   2:        0x10b9c10b1 - failure::on_fail::h0c4f1dedf6e42b45n2q
   3:        0x10bc30a55 - unwind::begin_unwind_inner::hf294fceb7364c35dJud
   4:        0x10b0f06a7 - unwind::begin_unwind::h15760749405991757620
   5:        0x10b0f0640 - unwind::begin_unwind::h15760749405991757620
   6:        0x108afebbd - driver::session::Session::span_bug::h77f162e9927192568sy
   7:        0x108d5c953 - middle::trans::common::fulfill_obligation::hc2c86e36ef61e3720p7
   8:        0x108d2313c - middle::trans::meth::trans_static_method_callee::hd78422dcc619c285H1k
   9:        0x108d2b56a - middle::trans::callee::trans_call::closure.124198
  10:        0x108d02b64 - middle::trans::callee::trans_call_inner::h7083ee476552c80bJM2
  11:        0x108d2adc7 - middle::trans::callee::trans_call::hbcd3295eac45cd284G2
  12:        0x108d3a621 - middle::trans::expr::trans_rvalue_dps_unadjusted::h091faa7fa9ae748diV4
  13:        0x108d38846 - middle::trans::expr::trans_unadjusted::h4687b5440f9f001cOh4
  14:        0x108cfc466 - middle::trans::expr::trans::h37ac91f8a5ab7eceKA3
  15:        0x108d4261a - middle::trans::expr::trans_binary::hc03bfcc1c44bed1euW5
  16:        0x108d38a66 - middle::trans::expr::trans_unadjusted::h4687b5440f9f001cOh4
  17:        0x108cfafcd - middle::trans::expr::trans_into::h49532942979d7e36Qw3
  18:        0x108cfb30d - middle::trans::controlflow::trans_block::hb62bfc3bd29145ddlKZ
  19:        0x108dabf3e - middle::trans::base::trans_closure::h6516d41fe619fb89MFf
  20:        0x108ced49a - middle::trans::base::trans_fn::h03123addb07e0c84aRf
  21:        0x108cef36e - middle::trans::monomorphize::monomorphic_fn::hae161eb11b7b9b092bZ
  22:        0x108d25fef - middle::trans::callee::trans_fn_ref_with_substs::h1c89324ccde73d421q2
  23:        0x108d227cb - middle::trans::callee::trans_fn_ref::h08bb22de61cc16afzf2
  24:        0x108d0465d - middle::trans::meth::trans_method_callee::h929a999be88dbc4bhXk
  25:        0x108d2bddb - middle::trans::callee::trans_method_call::closure.124210
  26:        0x108d02b64 - middle::trans::callee::trans_call_inner::h7083ee476552c80bJM2
  27:        0x108d2bc34 - middle::trans::callee::trans_method_call::h56051e89a394e3d5cI2
  28:        0x108d39a66 - middle::trans::expr::trans_rvalue_dps_unadjusted::h091faa7fa9ae748diV4
  29:        0x108d38846 - middle::trans::expr::trans_unadjusted::h4687b5440f9f001cOh4
  30:        0x108cfc466 - middle::trans::expr::trans::h37ac91f8a5ab7eceKA3
  31:        0x108d42c5f - middle::trans::expr::trans_unary::h06cacce96bad061fdC5
  32:        0x108d38a88 - middle::trans::expr::trans_unadjusted::h4687b5440f9f001cOh4
  33:        0x108cfc466 - middle::trans::expr::trans::h37ac91f8a5ab7eceKA3
  34:        0x108cfb7ce - middle::trans::controlflow::trans_if::h6aaf3e0e1595058b2MZ
  35:        0x108d39f00 - middle::trans::expr::trans_rvalue_dps_unadjusted::h091faa7fa9ae748diV4
  36:        0x108cfafad - middle::trans::expr::trans_into::h49532942979d7e36Qw3
  37:        0x108cfb30d - middle::trans::controlflow::trans_block::hb62bfc3bd29145ddlKZ
  38:        0x108dabf3e - middle::trans::base::trans_closure::h6516d41fe619fb89MFf
  39:        0x108ced49a - middle::trans::base::trans_fn::h03123addb07e0c84aRf
  40:        0x108cef36e - middle::trans::monomorphize::monomorphic_fn::hae161eb11b7b9b092bZ
  41:        0x108d15716 - middle::trans::base::get_res_dtor::h86daf3abd2d4c636VZd
  42:        0x108d148c2 - middle::trans::glue::trans_struct_drop::h2f0ef3a8eaf46f60NG0
  43:        0x108d1480a - middle::trans::glue::trans_struct_drop_flag::closure.123855
  44:        0x108d1471e - middle::trans::base::with_cond::hd938d2ce54028d64eRe
  45:        0x108d12731 - middle::trans::glue::make_drop_glue::as_closure.123819
  46:        0x108d114a6 - middle::trans::glue::make_generic_glue::h829e06c9ca5f1e5dji1
  47:        0x108d0f881 - middle::trans::glue::get_drop_glue::hf9d8897b2193e2915q0
  48:        0x108d0ee68 - middle::trans::glue::drop_ty::h4fc02bdea2206c15Bm0
  49:        0x108e26e34 - middle::trans::cleanup::DropValue.Cleanup::trans::h29d99c7be18fce65Q9x
  50:        0x108cfa8ec - middle::trans::cleanup::FunctionContext<'blk, 'tcx>.CleanupMethods<'blk, 'tcx>::pop_and_trans_ast_cleanup_scope::hf5d8b4a67ca0650deWw
  51:        0x108cfb321 - middle::trans::controlflow::trans_block::hb62bfc3bd29145ddlKZ
  52:        0x108dabf3e - middle::trans::base::trans_closure::h6516d41fe619fb89MFf
  53:        0x108ced49a - middle::trans::base::trans_fn::h03123addb07e0c84aRf
  54:        0x108ceac15 - middle::trans::base::trans_item::h3a59f9835edac94etag
  55:        0x108db5a78 - middle::trans::base::trans_crate::h82fc9db67d8fc02dZ8g
  56:        0x1091f31b5 - driver::driver::phase_4_translate_to_llvm::he6e636df9afeed0b5Tx
  57:        0x1091eb2b8 - driver::driver::compile_input::h368552c9534b42730qx
  58:        0x109273e34 - driver::run_compiler::h06d188804fcb3e77jfB
  59:        0x109271f66 - driver::run::closure.147679
  60:        0x1089573cb - task::TaskBuilder<S>::try_future::closure.101752
  61:        0x1089572b3 - task::TaskBuilder<S>::spawn_internal::closure.101723
  62:        0x1088ce8ad - task::spawn_opts::closure.8528
  63:        0x10bc961fc - rust_try_inner
  64:        0x10bc961e6 - rust_try
  65:        0x10bc2e0c7 - unwind::try::h84e3668876f753a7rjd
  66:        0x10bc2df4c - task::Task::run::h6ffe7aca8c2f2a9aCzc
  67:        0x1088ce6d2 - task::spawn_opts::closure.8467
  68:        0x10bc2f90a - thread::thread_start::h0f961398128da0abNTc
  69:     0x7fff8df4f899 - _pthread_body
  70:     0x7fff8df4f72a - _pthread_struct_init
