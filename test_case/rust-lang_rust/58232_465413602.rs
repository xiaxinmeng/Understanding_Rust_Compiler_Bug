
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/cyclomatic_complexity.rs:119:5
[01:15:06]     |
[01:15:06] 119 | /     fn check_fn(
[01:15:06] 119 | /     fn check_fn(
[01:15:06] 120 | |         &mut self,
[01:15:06] 121 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 122 | |         _: intravisit::FnKind<'tcx>,
[01:15:06] 131 | |         }
[01:15:06] 132 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut cyclomatic_complexity::CyclomaticComplexity, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut cyclomatic_complexity::CyclomaticComplexity, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
