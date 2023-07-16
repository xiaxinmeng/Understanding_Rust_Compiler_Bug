
thread 'rustc' panicked at 'Somehow loan paths are equal though their tys are not.', /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/mod.rs:340

stack backtrace:
   1:     0x7f81300fcfef - sys::backtrace::write::h7d93eae88f8f3133eas
                        at /home/ariel/Rust/source.arielb1/src/libstd/sys/unix/backtrace.rs:158
   2:     0x7f81301265c2 - panicking::on_panic::hd6d2040cb042b360dQw
                        at /home/ariel/Rust/source.arielb1/src/libstd/panicking.rs:47
   3:     0x7f81300662ae - rt::unwind::begin_unwind_inner::hdfc00ef77747fd67nvw
                        at /home/ariel/Rust/source.arielb1/src/libstd/rt/unwind.rs:569
   4:     0x7f812deb427e - rt::unwind::begin_unwind::h4527150991352814625
                        at /home/ariel/Rust/source.arielb1/src/libstd/rt/unwind.rs:522
   5:     0x7f812defd180 - borrowck::LoanPath<'tcx>.PartialEq::eq::hb5ac793488acbd65kYe
                        at /home/ariel/Rust/source.arielb1/build/<std macros>:3
   6:     0x7f812defd531 - cmp::PartialEq::ne::h7212816885052916627
                        at /home/ariel/Rust/source.arielb1/src/libcore/cmp.rs:54
   7:     0x7f812df04743 - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>::report_error_if_loan_conflicts_with_restriction::h330a007d0c931f15gGa
                        at /home/ariel/Rust/source.arielb1/src/liballoc/rc.rs:503
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:421
   8:     0x7f812df03cff - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>::report_error_if_loans_conflict::hc67d5057cc2422b0QDa
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:395
   9:     0x7f812df03692 - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>::check_for_conflicting_loans::closure.14184
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:367
  10:     0x7f812df0359b - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>::each_issued_loan::closure.14180
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:243
  11:     0x7f812df03500 - middle::dataflow::DataFlowContext<'a, 'tcx, O>::each_bit_on_entry::closure.14177
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/dataflow.rs:340
  12:     0x7f812df0346c - middle::dataflow::DataFlowContext<'a, 'tcx, O>::each_bit::h8172701194449475680
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/dataflow.rs:428
  13:     0x7f812df00cb7 - middle::dataflow::DataFlowContext<'a, 'tcx, O>::each_bit_for_node::h9784804647185004587
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/dataflow.rs:373
  14:     0x7f812df006aa - middle::dataflow::DataFlowContext<'a, 'tcx, O>::each_bit_on_entry::h12642047615205846027
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/dataflow.rs:340
  15:     0x7f812df00583 - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>::each_issued_loan::h7186547586807499617
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:241
  16:     0x7f812deb02d2 - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>::check_for_conflicting_loans::ha1ede3abe0edf94e2ya
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:364
  17:     0x7f812deaf729 - borrowck::check_loans::CheckLoanCtxt<'a, 'tcx>.euv..Delegate<'tcx>::borrow::h77a09b542b878142Mia
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:147
  18:     0x7f812dee157a - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_pat::closure.13659
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:1086
  19:     0x7f812dedf2c1 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern_::h17772808113108892374
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/mem_categorization.rs:1240
  20:     0x7f812dedfd29 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern_::h17772808113108892374
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/mem_categorization.rs:1276
  21:     0x7f812dee06f0 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern_::h17772808113108892374
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/mem_categorization.rs:1340
  22:     0x7f812dedefa5 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern::h9421848541532533988
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/mem_categorization.rs:1183
  23:     0x7f812dedee1b - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_pat::h10577114017912470439
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:1057
  24:     0x7f812def8f75 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_arm::h9221684664539810873
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:992
  25:     0x7f812deea783 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_expr::h13346995264182234936
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:494
  26:     0x7f812def483c - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::consume_expr::h11339377743097584266
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:383
  27:     0x7f812dee743b - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_block::h15484141585504048712
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:720
  28:     0x7f812dee9bb3 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_expr::h13346995264182234936
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:527
  29:     0x7f812def483c - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::consume_expr::h11339377743097584266
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:383
  30:     0x7f812dee743b - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_block::h15484141585504048712
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:720
  31:     0x7f812debb3e6 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_fn::h878218103832928268
                        at /home/ariel/Rust/source.arielb1/src/librustc/middle/expr_use_visitor.rs:337
  32:     0x7f812deb29cd - borrowck::check_loans::check_loans::h24184a3fff1ff048koa
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/check_loans.rs:213
  33:     0x7f812df7df89 - borrowck::borrowck_fn::h8659eb5213fc9d0bNOe
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/mod.rs:170
  34:     0x7f812df7bfa3 - borrowck::BorrowckCtxt<'a, 'tcx>.Visitor<'v>::visit_fn::he736900d0ab9b54fbGe
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/mod.rs:67
  35:     0x7f812df83bc6 - visit::walk_item::h7799702806911098296
                        at /home/ariel/Rust/source.arielb1/src/libsyntax/visit.rs:250
  36:     0x7f812df7e13a - borrowck::borrowck_item::hc37bd1e8ac3e538bRNe
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/mod.rs:142
  37:     0x7f812df7e071 - borrowck::BorrowckCtxt<'a, 'tcx>.Visitor<'v>::visit_item::h788cd39771815092nHe
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/mod.rs:78
  38:     0x7f812df836d7 - visit::walk_mod::h12981370025445949276
                        at /home/ariel/Rust/source.arielb1/src/libsyntax/visit.rs:160
  39:     0x7f812df83621 - visit::Visitor::visit_mod::h3759204270374681163
                        at /home/ariel/Rust/source.arielb1/src/libsyntax/visit.rs:64
  40:     0x7f812df8355b - visit::walk_crate::h16242287306266332251
                        at /home/ariel/Rust/source.arielb1/src/libsyntax/visit.rs:152
  41:     0x7f812df83144 - borrowck::check_crate::h65fa1876eda66f3dQIe
                        at /home/ariel/Rust/source.arielb1/src/librustc_borrowck/borrowck/mod.rs:108
  42:     0x7f8130953401 - driver::phase_3_run_analysis_passes::closure.22840
                        at /home/ariel/Rust/source.arielb1/src/librustc_driver/driver.rs:668
  43:     0x7f8130953666 - util::common::time::closure.22845
                        at /home/ariel/Rust/source.arielb1/src/librustc/util/common.rs:53
  44:     0x7f81309535c8 - time::duration::Duration::span::h942799780947541246
                        at /home/ariel/Rust/source.arielb1/src/libstd/time/duration.rs:152
  45:     0x7f8130953103 - util::common::time::h3255575620523811692
                        at /home/ariel/Rust/source.arielb1/src/librustc/util/common.rs:52
  46:     0x7f8130795094 - driver::phase_3_run_analysis_passes::hc567144ab7e30ff2tGa
                        at /home/ariel/Rust/source.arielb1/src/librustc_driver/driver.rs:667
  47:     0x7f813075199a - driver::compile_input::hf013ac36951f6218Qba
                        at /home/ariel/Rust/source.arielb1/src/librustc_driver/driver.rs:122
  48:     0x7f81309c965f - run_compiler::h373f46bdd73e6f1865b
                        at /home/ariel/Rust/source.arielb1/src/librustc_driver/lib.rs:156
  49:     0x7f81309c613b - run::closure.25696
                        at /home/ariel/Rust/source.arielb1/src/librustc_driver/lib.rs:99
  50:     0x7f81309c5328 - monitor::closure.25668
                        at /home/ariel/Rust/source.arielb1/src/librustc_driver/lib.rs:813
  51:     0x7f81309c5205 - boxed::F.FnBox<A>::call_box::h17893208957306821533
                        at /home/ariel/Rust/source.arielb1/src/liballoc/boxed.rs:374
  52:     0x7f81309c498e - boxed::Box<FnBox<A, Output $u3d$$u20$R$GT$$u2b$$u20$Send$u20$$u2b$$u20$$u27$a$GT$.FnOnce$LT$A$GT$::call_once::h12657657273517801178
                        at /home/ariel/Rust/source.arielb1/src/liballoc/boxed.rs:390
  53:     0x7f81309c3f5b - thread::Builder::spawn_inner::closure.25623
                        at /home/ariel/Rust/source.arielb1/src/libstd/thread/mod.rs:346
  54:     0x7f81309c3ede - rt::unwind::try::try_fn::__rust_abi::h5966669131881951758
                        at /home/ariel/Rust/source.arielb1/src/libstd/rt/unwind.rs:139
  55:     0x7f81309c3e79 - rt::unwind::try::try_fn::h5966669131881951758
  56:     0x7f8130257498 - rust_try_inner
  57:     0x7f8130257485 - rust_try
  58:     0x7f81309c3429 - rt::unwind::try::h15572344574618122417
                        at /home/ariel/Rust/source.arielb1/src/libstd/rt/unwind.rs:125
  59:     0x7f81309c320c - thread::Builder::spawn_inner::closure.25545
                        at /home/ariel/Rust/source.arielb1/src/libstd/thread/mod.rs:346
  60:     0x7f81309c4bc0 - boxed::F.FnBox<A>::call_box::h16471855808048377496
                        at /home/ariel/Rust/source.arielb1/src/liballoc/boxed.rs:374
  61:     0x7f81300f4c2e - boxed::Box<FnBox<A, Output $u3d$$u20$R$GT$$u2b$$u20$$u27$a$GT$.FnOnce$LT$A$GT$::call_once::h13741088014957751873
                        at /home/ariel/Rust/source.arielb1/src/liballoc/boxed.rs:382
  62:     0x7f81300f4b66 - sys_common::thread::start_thread::h8510a6abcec09c73bcr
                        at /home/ariel/Rust/source.arielb1/src/libstd/sys/common/thread.rs:30
  63:     0x7f8130122544 - sys::thread::Thread::new::thread_start::__rust_abi
                        at /home/ariel/Rust/source.arielb1/src/libstd/sys/unix/thread.rs:77
  64:     0x7f8130122524 - sys::thread::Thread::new::thread_start::h2ded727c690cef52JBv
  65:     0x7f81294420a3 - start_thread
  66:     0x7f812fcd004c - clone
  67:                0x0 - <unknown>
