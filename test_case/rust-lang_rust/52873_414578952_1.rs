
 Documenting typenum v1.10.0
     Running `rustdoc --crate-name typenum C:\Users\Victor\.cargo\registry\src\github.com-1ecc6299db9ec823\typenum-1.10.0\src\lib.rs --cap-lints warn -o D:\Development\rust\orm\target\doc -L dependency=D:\Development\rust\orm\target\debug\deps`

error: internal compiler error: librustc\traits\structural_impls.rs:178: impossible case reached

thread '<unnamed>' panicked at 'Box<Any>', librustc_errors\lib.rs:579:9

stack backtrace:
   0:      0x7fec2d40b63 - <std::sys::windows::args::Args as core::ops::drop::Drop>::drop::h07841acef7f9fb3b
   1:      0x7fec2d2a07f - <std::sys::windows::dynamic_lib::DynamicLibrary as core::ops::drop::Drop>::drop::h226bbb1fa45d584f
   2:      0x7fec2d28478 - std::panicking::take_hook::h99aaaf280ee62516
   3:      0x7fec2d28131 - std::panicking::take_hook::h99aaaf280ee62516
   4:      0x7fec2d28c7c - std::panicking::rust_panic_with_hook::ha70ac236a1ec44dd
   5:      0x7fee74590e2 - <rustc_errors::diagnostic::SubDiagnostic as core::fmt::Debug>::fmt::hf147e4b7188a1285
   6:      0x7fee7454619 - rustc_errors::Handler::bug::h5ba7d2a37e6d06a3
   7:      0x7fec34169cb - rustc::util::bug::bug_fmt::hd0f455d506468210
   8:      0x7fec3415f4c - rustc::ty::context::tls::track_diagnostic::h5e461387e66ce2d9
   9:      0x7fec3393d15 - rustc::ty::context::tls::track_diagnostic::h5e461387e66ce2d9
  10:      0x7fec34156f9 - rustc::ty::context::tls::track_diagnostic::h5e461387e66ce2d9
  11:      0x7fec34168dc - rustc::util::bug::bug_fmt::hd0f455d506468210
  12:      0x7fec3416842 - rustc::util::bug::bug_fmt::hd0f455d506468210
  13:      0x7fec315299d - rustc::traits::structural_impls::<impl rustc::ty::context::Lift<'tcx> for rustc::traits::SelectionError<'a>>::lift_to_tcx::h82aa210d085894b4
  14:      0x7fec35d2dec - rustc::ty::context::TyCtxt::intern_layout::ha57401a3edfd8794
  15:      0x7fec314469b - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  16:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  17:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  18:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  19:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  20:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  21:      0x7fec30c3ad0 - <unknown>
  22:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  23:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  24:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  25:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  26:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  27:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  28:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  29:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  30:      0x7fec30c3ad0 - <unknown>
  31:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  32:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  33:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  34:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  35:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  36:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  37:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  38:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  39:      0x7fec30c3ad0 - <unknown>
  40:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  41:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  42:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  43:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  44:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  45:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  46:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  47:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  48:      0x7fec30c3ad0 - <unknown>
  49:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  50:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  51:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  52:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  53:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  54:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  55:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  56:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  57:      0x7fec30c3ad0 - <unknown>
  58:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  59:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  60:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  61:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  62:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  63:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  64:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  65:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  66:      0x7fec30c3ad0 - <unknown>
  67:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  68:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  69:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  70:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  71:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  72:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  73:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  74:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  75:      0x7fec30c3ad0 - <unknown>
  76:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  77:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  78:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  79:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  80:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  81:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  82:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  83:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  84:      0x7fec30c3ad0 - <unknown>
  85:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  86:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  87:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  88:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  89:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  90:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  91:      0x7fec374480d - rustc::infer::InferCtxt::commit_from::h29f023f92d762265
  92:      0x7fec361c3d7 - rustc::ty::context::TypeckTables::expr_ty::h07311fd154b7e722
  93:      0x7fec30c3ad0 - <unknown>
  94:      0x7fec3145260 - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  95:      0x7fec31ffa60 - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  96:      0x7fec31444dd - rustc::traits::select::SelectionContext::coinductive_predicate::he82cb534328ed0b2
  97:      0x7fec3143707 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d
  98:      0x7fec31ff26b - rustc::dep_graph::graph::DepGraph::assert_ignored::hbdde8c850282765c
  99:      0x7fec3142295 - rustc::traits::select::SelectionContext::evaluate_obligation_recursively::habed88d9c383763d

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.30.0-nightly (33b923fd4 2018-08-18) running on x86_64-pc-windows-msvc

error: Could not document `typenum`.
