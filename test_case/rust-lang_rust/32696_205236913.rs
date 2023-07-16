
thread 'rustc' panicked at 'explicit panic', ../src/librbml/lib.rs:437
stack backtrace:
   1:     0x7f479a0fd3e0 - std::sys::backtrace::tracing::imp::write::hc787ac7b725fc252
   2:     0x7f479a10aaeb - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hff309ab1d83ffd90
   3:     0x7f479a10a68b - std::panicking::default_hook::h08ad3bb09872855b
   4:     0x7f479a0ce3af - std::sys_common::unwind::begin_unwind_inner::hfa66df7b3e69707e
   5:     0x7f479123abff - std::sys_common::unwind::begin_unwind::h422885d31fe250dc
   6:     0x7f47912388c6 - rbml::reader::get_doc::hf96062258c695953
   7:     0x7f47991daa3b - rustc_metadata::decoder::item_name::h937d5af7cabdc0db
   8:     0x7f479922228b - rustc_metadata::csearch::_<impl rustc..middle..cstore..CrateStore<'tcx> for cstore..CStore>::item_name::h6797a808760c2731
   9:     0x7f4794084dd7 - rustc::ty::item_path::_<impl ty..context..TyCtxt<'tcx>>::push_item_path::hac1b9fe02a3c37ed
  10:     0x7f4794085510 - rustc::ty::item_path::_<impl ty..context..TyCtxt<'tcx>>::push_item_path::hac1b9fe02a3c37ed
  11:     0x7f4793f8785d - rustc::ty::item_path::_<impl ty..context..TyCtxt<'tcx>>::item_path_str::h94bec330b9844a26
  12:     0x7f4798da4208 - rustc_trans::type_of::llvm_type_name::h8232cc541b66c2fa
  13:     0x7f4798c7b301 - rustc_trans::type_of::in_memory_type_of::h39ed35af1c9bd3c0
  14:     0x7f4798d17c4f - rustc_trans::consts::get_static::ha556e7f1f5a5c12b
  15:     0x7f4798d04ffc - rustc_trans::expr::trans_var::h9e834a8c15e86835
  16:     0x7f4798d4324b - rustc_trans::expr::trans_unadjusted::h1c3be1e5aa61621c
  17:     0x7f4798cfbe69 - rustc_trans::expr::trans::h542fffcc984b03b8
  18:     0x7f4798d4c680 - rustc_trans::expr::trans_addr_of::h14be6b3556c791ca
  19:     0x7f4798d433a9 - rustc_trans::expr::trans_unadjusted::h1c3be1e5aa61621c
  20:     0x7f4798cfbe69 - rustc_trans::expr::trans::h542fffcc984b03b8
  21:     0x7f4798c9ec1a - rustc_trans::base::init_local::h5b725441afc1979c
  22:     0x7f4798cb40a4 - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  23:     0x7f4798d44a10 - rustc_trans::expr::trans_rvalue_dps_unadjusted::h2767d79242f1a8da
  24:     0x7f4798cbcb04 - rustc_trans::expr::trans_into::hccfa250f984a4de0
  25:     0x7f4798d2211b - rustc_trans::controlflow::trans_stmt_semi::h924ebf14e2481e96
  26:     0x7f4798cb4080 - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  27:     0x7f4798d44a10 - rustc_trans::expr::trans_rvalue_dps_unadjusted::h2767d79242f1a8da
  28:     0x7f4798cbcb04 - rustc_trans::expr::trans_into::hccfa250f984a4de0
  29:     0x7f4798cb43ad - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  30:     0x7f4798cb2921 - rustc_trans::base::trans_closure::h2b7da2bc6c23f7b2
  31:     0x7f4798d04583 - rustc_trans::closure::trans_closure_expr::hfd53eb0252594d76
  32:     0x7f4798d11889 - rustc_trans::consts::const_expr_unadjusted::h1d9baab0ebf2616d
  33:     0x7f4798d0ebf5 - rustc_trans::consts::const_expr::hf3d8253a95ec460b
  34:     0x7f4798d100ed - rustc_trans::consts::get_const_expr_as_global::h1f3cb11932140c8c
  35:     0x7f4798cfb462 - rustc_trans::expr::trans::h542fffcc984b03b8
  36:     0x7f4798c9ec1a - rustc_trans::base::init_local::h5b725441afc1979c
  37:     0x7f4798cb40a4 - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  38:     0x7f4798cb2921 - rustc_trans::base::trans_closure::h2b7da2bc6c23f7b2
  39:     0x7f4798cb4c70 - rustc_trans::base::trans_fn::h9d408db44d2e9f24
  40:     0x7f4798ce84c1 - rustc_trans::callee::get_fn::h80509230b00b2d45
  41:     0x7f4798c940ce - rustc_trans::callee::Callee::def::ha59f7b3a44ed318c
  42:     0x7f4798ce303f - rustc_trans::callee::Callee::method_call::h91c54111eae6060e
  43:     0x7f4798d448df - rustc_trans::expr::trans_rvalue_dps_unadjusted::h2767d79242f1a8da
  44:     0x7f4798cbcb04 - rustc_trans::expr::trans_into::hccfa250f984a4de0
  45:     0x7f4798d2211b - rustc_trans::controlflow::trans_stmt_semi::h924ebf14e2481e96
  46:     0x7f4798cb4080 - rustc_trans::controlflow::trans_block::h85160ce006cd8aa8
  47:     0x7f4798cb2921 - rustc_trans::base::trans_closure::h2b7da2bc6c23f7b2
  48:     0x7f4798cb4c70 - rustc_trans::base::trans_fn::h9d408db44d2e9f24
  49:     0x7f4798cbf1f1 - rustc_trans::base::trans_item::h6ce564dd6c26e0a3
  50:     0x7f4798cd8a3b - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc_front..intravisit..Visitor<'v>>::visit_item::h17575fb313345f09
  51:     0x7f4798cc60bb - rustc_trans::base::trans_crate::h1aff7a8470cdce2a
  52:     0x7f479a65ca28 - rustc_driver::driver::phase_4_translate_to_llvm::h7b8775f4874fc83b
  53:     0x7f479a65b2b5 - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h31c654547881d0b4
  54:     0x7f479a657e98 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::h25b1c02cc580d282
  55:     0x7f479a651e4b - rustc::ty::context::TyCtxt::create_and_enter::h567cfc6893966048
  56:     0x7f479a64e8c9 - rustc_driver::driver::phase_3_run_analysis_passes::h925d8f426a8a3ebf
  57:     0x7f479a6212e7 - rustc_driver::driver::compile_input::h4de0250909a780b6
  58:     0x7f479a60f044 - rustc_driver::run_compiler::hb9ea120b81672dbd
  59:     0x7f479a60c4a1 - std::sys_common::unwind::try::try_fn::h10e40fd87dec6cc0
  60:     0x7f479a0faa3b - __rust_try
  61:     0x7f479a0fa9cd - std::sys_common::unwind::inner_try::h010cc5014fd950a2
  62:     0x7f479a60ccea - _<F as std..boxed..FnBox<A>>::call_box::hbe6d6868b645ad75
  63:     0x7f479a108c74 - std::sys::thread::Thread::new::thread_start::h003cd23941cb846b
  64:     0x7f4791fb0181 - start_thread
  65:     0x7f4799d5247c - __clone
  66:                0x0 - <unknown>
