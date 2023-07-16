
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: DistinctSources(DistinctSources { begin: (Real("operations\\test-ops-client\\src\\main.rs"), BytePos(0)), end: (Macros("::async_stream::stream"), BytePos(9087160)) })', src\librustc_errors\emitter.rs:2111:17
stack backtrace:
   0:     0x7ffac93a7659 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h625fdcbc2a9694d5
   1:     0x7ffac93d20bb - core::fmt::write::h235756b81a8b440f
   2:     0x7ffac9399294 - <std::io::IoSliceMut as core::fmt::Debug>::fmt::h6e5f44856ecf3f04
   3:     0x7ffac93abcfc - std::panicking::take_hook::he2dd45ee22ea221f
   4:     0x7ffac93ab94c - std::panicking::take_hook::he2dd45ee22ea221f
   5:     0x7ffac33180ca - rustc_driver::report_ice::hcbf9bbe620d9cecc
   6:     0x7ffac93ac504 - std::panicking::rust_panic_with_hook::hd8b8e2d75c42801f
   7:     0x7ffac93ac065 - rust_begin_unwind
   8:     0x7ffac93cec70 - core::panicking::panic_fmt::h886688db175bdb2e
   9:     0x7ffac93ce903 - core::result::unwrap_failed::h121585868e30f490
  10:     0x7ffac6d4e45d - rustc_errors::emitter::is_case_difference::h1c1bd965681311fc
  11:     0x7ffac6d40591 - rustc_errors::emitter::HumanReadableErrorType::new_emitter::h3912f470f18b8c9e
  12:     0x7ffac6d42312 - <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit_diagnostic::h2684bc7a0b02c2a7
  13:     0x7ffac6d59bb4 - <rustc_errors::json::JsonEmitter as rustc_errors::emitter::Emitter>::emit_diagnostic::hda2d893071c06bb6
  14:     0x7ffac6d3e8e6 - rustc_errors::HandlerInner::emit_diagnostic::h42ed1343a44b0cc5
  15:     0x7ffac6d4fab5 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h47f2baf3bc188133
  16:     0x7ffac56399f8 - rustc_typeck::check::demand::<impl rustc_typeck::check::FnCtxt>::demand_coerce::hb78b4f38c95bbf5d
  17:     0x7ffac564456f - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  18:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  19:     0x7ffac5685eba - rustc_typeck::check::FnCtxt::self_type_matches_expected_vid::h6928db0ede289609
  20:     0x7ffac5630352 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  21:     0x7ffac562de5c - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  22:     0x7ffac563f571 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  23:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  24:     0x7ffac5644558 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  25:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  26:     0x7ffac5689243 - rustc_typeck::check::FnCtxt::check_decl_initializer::h7e8fd8a039af1b50
  27:     0x7ffac56892ef - rustc_typeck::check::FnCtxt::check_decl_local::h145f96877dd64543
  28:     0x7ffac56897bf - rustc_typeck::check::FnCtxt::check_stmt::he4a62204237b1b76
  29:     0x7ffac568a166 - rustc_typeck::check::FnCtxt::check_block_no_value::he5cd3b4c3f8672a2
  30:     0x7ffac563f588 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  31:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  32:     0x7ffac568a1a4 - rustc_typeck::check::FnCtxt::check_block_no_value::he5cd3b4c3f8672a2
  33:     0x7ffac563f588 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  34:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  35:     0x7ffac564d60e - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  36:     0x7ffac56710d0 - <rustc_typeck::check::GatherLocalsVisitor as rustc_hir::intravisit::Visitor>::visit_pat::h6cc40edee6bc0f9e
  37:     0x7ffac5633011 - rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure::h658039c54cd63e23
  38:     0x7ffac563f16c - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  39:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  40:     0x7ffac5685eba - rustc_typeck::check::FnCtxt::self_type_matches_expected_vid::h6928db0ede289609
  41:     0x7ffac5630352 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  42:     0x7ffac562de5c - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  43:     0x7ffac563f571 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  44:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  45:     0x7ffac5685eba - rustc_typeck::check::FnCtxt::self_type_matches_expected_vid::h6928db0ede289609
  46:     0x7ffac5630352 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  47:     0x7ffac562de5c - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  48:     0x7ffac563f571 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  49:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  50:     0x7ffac568a1a4 - rustc_typeck::check::FnCtxt::check_block_no_value::he5cd3b4c3f8672a2
  51:     0x7ffac563f588 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  52:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  53:     0x7ffac5689243 - rustc_typeck::check::FnCtxt::check_decl_initializer::h7e8fd8a039af1b50
  54:     0x7ffac56892ef - rustc_typeck::check::FnCtxt::check_decl_local::h145f96877dd64543
  55:     0x7ffac56897bf - rustc_typeck::check::FnCtxt::check_stmt::he4a62204237b1b76
  56:     0x7ffac568a166 - rustc_typeck::check::FnCtxt::check_block_no_value::he5cd3b4c3f8672a2
  57:     0x7ffac563f588 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  58:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  59:     0x7ffac563f188 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  60:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  61:     0x7ffac568a1a4 - rustc_typeck::check::FnCtxt::check_block_no_value::he5cd3b4c3f8672a2
  62:     0x7ffac563f588 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  63:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  64:     0x7ffac564d60e - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  65:     0x7ffac56710d0 - <rustc_typeck::check::GatherLocalsVisitor as rustc_hir::intravisit::Visitor>::visit_pat::h6cc40edee6bc0f9e
  66:     0x7ffac5633011 - rustc_typeck::check::closure::<impl rustc_typeck::check::FnCtxt>::check_expr_closure::h658039c54cd63e23
  67:     0x7ffac563f16c - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  68:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  69:     0x7ffac5685eba - rustc_typeck::check::FnCtxt::self_type_matches_expected_vid::h6928db0ede289609
  70:     0x7ffac5630352 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  71:     0x7ffac562de5c - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt>::check_call::h2c4d5a631e8148f7
  72:     0x7ffac563f571 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  73:     0x7ffac563eb87 - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  74:     0x7ffac564d60e - rustc_typeck::check::expr::<impl rustc_typeck::check::FnCtxt>::check_expr_with_expectation::h4645e25389d427d7
  75:     0x7ffac56710d0 - <rustc_typeck::check::GatherLocalsVisitor as rustc_hir::intravisit::Visitor>::visit_pat::h6cc40edee6bc0f9e
  76:     0x7ffac55b68ab - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  77:     0x7ffac566f836 - <rustc_typeck::check::fixup_opaque_types::FixupFolder as rustc::ty::fold::TypeFolder>::fold_ty::h592aee909a4c47ac
  78:     0x7ffac56a084f - <rustc_typeck::check::check_opaque_for_inheriting_lifetimes::ProhibitOpaqueVisitor as core::fmt::Debug>::fmt::h03b9ea941dd82de1
  79:     0x7ffac55098fc - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  80:     0x7ffac570eb86 - <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr::h2bdabc9927600410
  81:     0x7ffac55172c7 - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  82:     0x7ffac566f848 - <rustc_typeck::check::fixup_opaque_types::FixupFolder as rustc::ty::fold::TypeFolder>::fold_ty::h592aee909a4c47ac
  83:     0x7ffac56a084f - <rustc_typeck::check::check_opaque_for_inheriting_lifetimes::ProhibitOpaqueVisitor as core::fmt::Debug>::fmt::h03b9ea941dd82de1
  84:     0x7ffac55098fc - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  85:     0x7ffac570eb86 - <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr::h2bdabc9927600410
  86:     0x7ffac55172c7 - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  87:     0x7ffac54cd5bb - <rustc_typeck::collect::has_late_bound_regions::LateBoundRegionsDetector as rustc_hir::intravisit::Visitor>::visit_lifetime::hc3b965261ecca9b1
  88:     0x7ffac56a252c - <rustc_typeck::check::check_opaque_for_inheriting_lifetimes::ProhibitOpaqueVisitor as core::fmt::Debug>::fmt::h03b9ea941dd82de1
  89:     0x7ffac550950c - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  90:     0x7ffac5710256 - <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr::h2bdabc9927600410
  91:     0x7ffac558992c - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  92:     0x7ffac56f5402 - <rustc_typeck::namespace::Namespace as core::fmt::Debug>::fmt::hbfbf22bb8d996fcc
  93:     0x7ffac54c6d57 - <rustc_typeck::collect::CollectItemTypesVisitor as rustc_hir::intravisit::Visitor>::visit_item::h16809ad5598ca4cb
  94:     0x7ffac55d0399 - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  95:     0x7ffac54c5ed4 - <rustc_typeck::check::upvar::InferBorrowKind as rustc_typeck::expr_use_visitor::Delegate>::mutate::hb458d05bec4d68dd
  96:     0x7ffac56a17dd - <rustc_typeck::check::check_opaque_for_inheriting_lifetimes::ProhibitOpaqueVisitor as core::fmt::Debug>::fmt::h03b9ea941dd82de1
  97:     0x7ffac5509bbc - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
  98:     0x7ffac570e3f0 - <rustc_typeck::check::regionck::RegionCtxt as rustc_hir::intravisit::Visitor>::visit_expr::h2bdabc9927600410
  99:     0x7ffac55723e9 - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
 100:     0x7ffac5509f74 - <rustc_typeck::outlives::explicit::ExplicitPredicatesMap as core::fmt::Debug>::fmt::hc2e8371c3078742d
 101:     0x7ffac54b97cb - rustc_typeck::check_crate::h3487626fb3d3b3cf
 102:     0x7ffac34348c1 - rustc_interface::passes::QueryContext::print_stats::h25b51332ab3c20f8
 103:     0x7ffac32b4293 - <rustc_span::symbol::SymbolStr as core::fmt::Display>::fmt::h6ddbe188959d8aa3
 104:     0x7ffac33078a5 - rustc_driver::pretty::print_after_hir_lowering::h40bb6c52d19325cc
 105:     0x7ffac331a91c - <rustc_driver::DEFAULT_HOOK as core::ops::deref::Deref>::deref::h11dcec2ec1184e59
 106:     0x7ffac330522d - rustc_driver::pretty::print_after_hir_lowering::h40bb6c52d19325cc
 107:     0x7ffac32c03c4 - rustc_driver::pretty::print_after_hir_lowering::h40bb6c52d19325cc
 108:     0x7ffac32b0cee - <env_logger::filter::inner::Filter as core::fmt::Display>::fmt::h43fd01c0e47a0ed1
 109:     0x7ffac32ae6b9 - <env_logger::filter::inner::Filter as core::fmt::Display>::fmt::h43fd01c0e47a0ed1
 110:     0x7ffac32b260c - <env_logger::filter::inner::Filter as core::fmt::Display>::fmt::h43fd01c0e47a0ed1
 111:     0x7ffac93bdd22 - _rust_maybe_catch_panic
 112:     0x7ffac32c3ba2 - rustc_driver::pretty::print_after_hir_lowering::h40bb6c52d19325cc
 113:     0x7ffac9387a07 - ZN244_$LT$std..error..$LT$impl$u20$core..convert..From$LT$alloc..string..String$GT$$u20$for$u20$alloc..boxed..Box$LT$dyn$u20$std..error..Error$u2b$core..marker..Send$u2b$core..marker..Sync$GT$$GT$..from..StringError$u20$as$u20$core..fmt..Display$GT$3fmt17
 114:     0x7ffac93bb5d7 - std::sys::windows::thread::Thread::new::h641608bd6db7dfca
 115:     0x7ffb3aba7bd4 - BaseThreadInitThunk
 116:     0x7ffb3b3eced1 - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.42.0-beta.3 (86f329b41 2020-02-07) running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [typeck_tables_of] processing `create_wall`
#1 [typeck_tables_of] processing `create_wall::{{closure}}#0`
#2 [type_of] processing `create_wall::{{closure}}#0`
#3 [collect_mod_item_types] collecting item types in top-level module
#4 [analysis] running analysis passes on this crate
end of query stack
