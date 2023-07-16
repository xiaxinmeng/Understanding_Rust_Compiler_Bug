
<anon>:4:5: 4:21 warning: path statement with no effect, #[warn(path_statements)] on by default
<anon>:4     DUFF_EXPRESSION;
             ^~~~~~~~~~~~~~~~
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'assertion failed: type_is_zero_size(bcx.ccx(), bcx.tcx().node_id_to_type(expr.id))', ../src/librustc_trans/trans/expr.rs:161
stack backtrace:
   1:     0x7f5f5913a330 - sys::backtrace::tracing::imp::write::h3675b4f0ca767761Xcv
   2:     0x7f5f5914368b - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44519
   3:     0x7f5f591431e3 - panicking::default_handler::h18faf4fbd296d909lSz
   4:     0x7f5f591079ac - sys_common::unwind::begin_unwind_inner::hfb5d07d6e405c6bbg1t
   5:     0x7f5f575ca5df - sys_common::unwind::begin_unwind::h7362127277952153626
   6:     0x7f5f57645e4c - trans::expr::trans_into::h7652c0f3c8b5cd0cZND
   7:     0x7f5f576b48fd - trans::controlflow::trans_stmt_semi::h63e0f5036212e5988qy
   8:     0x7f5f57642620 - trans::controlflow::trans_block::hde5006322ab6cf144ry
   9:     0x7f5f57640fde - trans::base::trans_closure::hba0c0872f549ec52KOj
  10:     0x7f5f5764332c - trans::base::trans_fn::hed9c6073728147b7TZj
  11:     0x7f5f57647bd2 - trans::base::trans_item::h6acb22e4cadb1a91Xnk
  12:     0x7f5f5766576b - trans::base::TransItemsWithinModVisitor<'a, 'tcx>.Visitor<'v>::visit_item::h17401197a5140e35bol
  13:     0x7f5f57652e4b - trans::base::trans_crate::he0efa1d5417c3e0bB7k
  14:     0x7f5f5966fd84 - driver::phase_4_translate_to_llvm::h6b8ee7bea6f9aad8O1a
  15:     0x7f5f5966e58c - driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::closure.29535
  16:     0x7f5f5966ae57 - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.28550
  17:     0x7f5f59664c04 - middle::ty::context::ctxt<'tcx>::create_and_enter::h16192426302205697189
  18:     0x7f5f596616ef - driver::phase_3_run_analysis_passes::h10920926650053012691
  19:     0x7f5f59634a15 - driver::compile_input::h7ae6a86e23de0774Hca
  20:     0x7f5f59623b47 - run_compiler::hb0408bcf47642fe6mPc
  21:     0x7f5f596212c1 - sys_common::unwind::try::try_fn::h7614151098073783683
  22:     0x7f5f59137d1b - __rust_try
  23:     0x7f5f591301fd - sys_common::unwind::inner_try::hadd81c754a64f07ciYt
  24:     0x7f5f59621b10 - boxed::F.FnBox<A>::call_box::h18261575856772421581
  25:     0x7f5f59141c59 - sys::thread::Thread::new::thread_start::h9bc812305b5e01feFPy
  26:     0x7f5f5240b423 - start_thread
  27:     0x7f5f58db9cbc - clone
  28:                0x0 - <unknown>
