
thread 'rustc' panicked at 'explicit panic', compiler\rustc_middle\src\lint.rs:287:37
stack backtrace:
   0:     0x7ff863329e2e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h34f33193f2c6e691
   1:     0x7ff863357e7b - core::fmt::write::h435be716304d26a1
   2:     0x7ff86331b788 - <std::io::IoSlice as core::fmt::Debug>::fmt::h5c209409e89b6b83
   3:     0x7ff86332f36d - std::panicking::take_hook::h78aad67ccc0974f2
   4:     0x7ff86332ef48 - std::panicking::take_hook::h78aad67ccc0974f2
   5:     0x7ff8480a19b7 - rustc_driver::report_ice::hef358031e18df712
   6:     0x7ff86332fe80 - std::panicking::rust_panic_with_hook::h4d71ff73fb7c451d
   7:     0x7ff84c5ac074 - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::context::Lift for rustc_ast::ast::InlineAsmOptions>::lift_to_tcx::h042fb894ab77ec23
   8:     0x7ff84c5abf6f - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::context::Lift for rustc_ast::ast::InlineAsmOptions>::lift_to_tcx::h042fb894ab77ec23
   9:     0x7ff84c5abfad - rustc_middle::ty::structural_impls::<impl rustc_middle::ty::context::Lift for rustc_ast::ast::InlineAsmOptions>::lift_to_tcx::h042fb894ab77ec23
  10:     0x7ff84c5c6d01 - rustc_middle::lint::struct_lint_level::struct_lint_level_impl::h8ed4092b7d535841
  11:     0x7ff84bcec813 - <rustc_lint::array_into_iter::ArrayIntoIter as rustc_lint::passes::LateLintPass>::check_expr::h876461b2d5357325
  12:     0x7ff84bcacc05 - <rustc_lint::BuiltinCombinedLateLintPass as rustc_lint::passes::LateLintPass>::check_expr::hdeaf5fd3da5c9014
  13:     0x7ff84827d272 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  14:     0x7ff84821224f - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  15:     0x7ff84827d27d - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  16:     0x7ff84821240e - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  17:     0x7ff84827d27d - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  18:     0x7ff84827d27d - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  19:     0x7ff84827d372 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  20:     0x7ff84821252c - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  21:     0x7ff84827d27d - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  22:     0x7ff84827d6e3 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  23:     0x7ff84827db47 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  24:     0x7ff84820efa8 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  25:     0x7ff8482809a3 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  26:     0x7ff84821354b - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  27:     0x7ff8482803c3 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  28:     0x7ff84820e36e - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  29:     0x7ff84827368d - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  30:     0x7ff848296921 - <rustc_interface::proc_macro_decls::Finder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::h95eaafd33dc1f3b3
  31:     0x7ff8482bd8ae - rustc_interface::passes::QueryContext::print_stats::h960d22dc06b880d2
  32:     0x7ff8480e2d00 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h3e367fa31fb5b7e3
  33:     0x7ff8480a52c7 - <rustc_driver::Compilation as core::fmt::Debug>::fmt::ha64461c03fe53d77
  34:     0x7ff8480bd5b6 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h3e367fa31fb5b7e3
  35:     0x7ff848059e73 - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::hee7667dfc0b493b5
  36:     0x7ff8480e80db - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h3e367fa31fb5b7e3
  37:     0x7ff8480c0d16 - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h3e367fa31fb5b7e3
  38:     0x7ff8480eb9dd - rustc_ast::util::parser::prec_let_scrutinee_needs_par::h3e367fa31fb5b7e3
  39:     0x7ff86333ffb7 - std::sys::windows::thread::Thread::new::h4a8ee57959003089
  40:     0x7ff8c6267c24 - BaseThreadInitThunk
  41:     0x7ff8c69acea1 - RtlUserThreadStart

error: internal compiler error: unexpected panic
