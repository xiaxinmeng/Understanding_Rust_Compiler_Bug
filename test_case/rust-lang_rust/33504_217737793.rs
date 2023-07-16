
thread 'rustc' panicked at 'Def::Err in memory categorization', ../src/librustc/middle/mem_categorization.rs:613
stack backtrace:
   1:     0x7fb3f2270330 - sys::backtrace::tracing::imp::write::h3675b4f0ca767761Xcv
   2:     0x7fb3f227968b - panicking::default_handler::_$u7b$$u7b$closure$u7d$$u7d$::closure.44519
   3:     0x7fb3f22791e3 - panicking::default_handler::h18faf4fbd296d909lSz
   4:     0x7fb3f223d9ac - sys_common::unwind::begin_unwind_inner::hfb5d07d6e405c6bbg1t
   5:     0x7fb3ef4c120f - sys_common::unwind::begin_unwind::h17349651284183375827
   6:     0x7fb3ef4ffc45 - middle::mem_categorization::MemCategorizationContext<'t, 'a, 'tcx>::cat_def::hdeb344a5491bba80aeK
   7:     0x7fb3ef4fdbc2 - middle::mem_categorization::MemCategorizationContext<'t, 'a, 'tcx>::cat_pattern_::h2171199047159068738
   8:     0x7fb3ef4fa833 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_pat::hf4d5b3d8cb6f67ecUgh
   9:     0x7fb3ef4f5353 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_irrefutable_pat::h5668c6e2ce3263877bh
  10:     0x7fb3ef4f8ad6 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_stmt::h3e8eaabe6a16d9c1lKg
  11:     0x7fb3ef4f7c3e - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_expr::he2c7af9ebce7c88agsg
  12:     0x7fb3ef4f54de - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::consume_expr::h1393a4fb15fc5089Xjg
  13:     0x7fb3ef4f5155 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'a, 'tcx>::walk_fn::h8572ed4d964016a4Xeg
  14:     0x7fb3f18af490 - check::upvar::AdjustBorrowKind<'a, 'tcx>.Visitor<'v>::visit_fn::h029e8800c7c40314P4j
  15:     0x7fb3f18af213 - intravisit::walk_expr::h6226208990425569121
  16:     0x7fb3f17f7b0a - check::check_bare_fn::h846d602fffbab51bLDo
  17:     0x7fb3f17f2876 - check::check_item_body::h1bc79a321f78eb96h3o
  18:     0x7fb3f17e932b - check::check_item_bodies::h5671f35c802ef7c1yBo
  19:     0x7fb3f17e0bb3 - check_crate::h7bb847abc9f256eeeCC
  20:     0x7fb3f279c667 - driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::closure.28550
  21:     0x7fb3f279ac04 - middle::ty::context::ctxt<'tcx>::create_and_enter::h16192426302205697189
  22:     0x7fb3f27976ef - driver::phase_3_run_analysis_passes::h10920926650053012691
  23:     0x7fb3f276aa15 - driver::compile_input::h7ae6a86e23de0774Hca
  24:     0x7fb3f2759b47 - run_compiler::hb0408bcf47642fe6mPc
  25:     0x7fb3f27572c1 - sys_common::unwind::try::try_fn::h7614151098073783683
  26:     0x7fb3f226dd1b - __rust_try
  27:     0x7fb3f22661fd - sys_common::unwind::inner_try::hadd81c754a64f07ciYt
  28:     0x7fb3f2757b10 - boxed::F.FnBox<A>::call_box::h18261575856772421581
  29:     0x7fb3f2277c59 - sys::thread::Thread::new::thread_start::h9bc812305b5e01feFPy
  30:     0x7fb3eb541473 - start_thread
  31:     0x7fb3f1eefacc - clone
  32:                0x0 - <unknown>
