
[00:55:15] error[E0053]: method `visit_path` has an incompatible type for trait
[00:55:15]    --> tools\clippy\clippy_lints\src\utils\internal_lints.rs:201:5
[00:55:15]     |
[00:55:15] 201 |     fn visit_path(&mut self, path: &'tcx Path, _: NodeId) {
[00:55:15]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[00:55:15]     |
[00:55:15]     = note: expected type `fn(&mut utils::internal_lints::LintCollector<'a, 'tcx>, &'tcx rustc::hir::Path, rustc::hir::HirId)`
[00:55:15]                found type `fn(&mut utils::internal_lints::LintCollector<'a, 'tcx>, &'tcx rustc::hir::Path, syntax::ast::NodeId)`
[00:55:15] 
[00:55:15] [RUSTC-TIMING] failure test:false 1.718
[00:55:15]    Compiling rls-analysis v0.14.0
[00:55:15] error[E0053]: method `visit_path` has an incompatible type for trait
[00:55:15]    --> tools\clippy\clippy_lints\src\use_self.rs:211:5
[00:55:15]     |
[00:55:15] 211 |     fn visit_path(&mut self, path: &'tcx Path, _id: NodeId) {
[00:55:15]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[00:55:15]     |
[00:55:15]     = note: expected type `fn(&mut use_self::UseSelfVisitor<'a, 'tcx>, &'tcx rustc::hir::Path, rustc::hir::HirId)`
[00:55:15]                found type `fn(&mut use_self::UseSelfVisitor<'a, 'tcx>, &'tcx rustc::hir::Path, syntax::ast::NodeId)`
[00:55:15] 
[00:55:15] error: aborting due to 2 previous errors
