plain
[01:15:06]    |
[01:15:06] 68 | /     fn check_fn(
[01:15:06] 69 | |         &mut self,
[01:15:06] 70 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 71 | |         _: intravisit::FnKind<'tcx>,
[01:15:06] ...  |
[01:15:06] 77 | |         NonminimalBoolVisitor { cx }.visit_body(body)
[01:15:06]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut booleans::NonminimalBool, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut booleans::NonminimalBool, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
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
[01:15:06] error[E0053]: method `check_mod` has an incompatible type for trait
[01:15:06]   --> src/tools/clippy/clippy_lints/src/enum_glob_use.rs:42:5
[01:15:06]    |
[01:15:06]    |
[01:15:06] 42 |     fn check_mod(&mut self, cx: &LateContext<'a, 'tcx>, m: &'tcx Mod, _: Span, _: NodeId) {
[01:15:06]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut enum_glob_use::EnumGlobUse, &rustc::lint::LateContext<'a, 'tcx>, &'tcx rustc::hir::Mod, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut enum_glob_use::EnumGlobUse, &rustc::lint::LateContext<'a, 'tcx>, &'tcx rustc::hir::Mod, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]   --> src/tools/clippy/clippy_lints/src/escape.rs:62:5
[01:15:06]    |
[01:15:06] 62 | /     fn check_fn(
[01:15:06] 62 | /     fn check_fn(
[01:15:06] 63 | |         &mut self,
[01:15:06] 64 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 65 | |         _: visit::FnKind<'tcx>,
[01:15:06] 98 | |         }
[01:15:06] 99 | |     }
[01:15:06]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut escape::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut escape::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/functions.rs:108:5
[01:15:06]     |
[01:15:06] 108 | /     fn check_fn(
---
[01:15:06] 150 | |         self.check_line_number(cx, span);
[01:15:06] 151 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut functions::Functions, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut functions::Functions, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/implicit_return.rs:124:5
[01:15:06]     |
[01:15:06] 124 | /     fn check_fn(
---
[01:15:06] 140 | |         }
[01:15:06] 141 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut implicit_return::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut implicit_return::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/misc.rs:252:5
[01:15:06]     |
[01:15:06] 252 | /     fn check_fn(
[01:15:06] 252 | /     fn check_fn(
[01:15:06] 253 | |         &mut self,
[01:15:06] 254 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 255 | |         k: FnKind<'tcx>,
[01:15:06] 278 | |         }
[01:15:06] 279 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut misc::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut misc::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/missing_const_for_fn.rs:75:5
[01:15:06]     |
[01:15:06] 75  | /     fn check_fn(
---
[01:15:06] 114 | |         }
[01:15:06] 115 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut missing_const_for_fn::MissingConstForFn, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut missing_const_for_fn::MissingConstForFn, &rustc::lint::LateContext<'_, '_>, rustc::hir::intravisit::FnKind<'_>, &rustc::hir::FnDecl, &rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:77:5
[01:15:06]     |
[01:15:06] 77  | /     fn check_fn(
---
[01:15:06] 319 | |         }
[01:15:06] 320 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut needless_pass_by_value::NeedlessPassByValue, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut needless_pass_by_value::NeedlessPassByValue, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:84:5
[01:15:06]     |
[01:15:06] 84  | /     fn check_fn(
---
[01:15:06] 222 | |         }
[01:15:06] 223 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut redundant_clone::RedundantClone, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut redundant_clone::RedundantClone, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/shadow.rs:92:5
[01:15:06]     |
[01:15:06] 92  | /     fn check_fn(
---
[01:15:06] 104 | |         check_fn(cx, decl, body);
[01:15:06] 105 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut shadow::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut shadow::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/trivially_copy_pass_by_ref.rs:161:5
[01:15:06]     |
[01:15:06] 161 | /     fn check_fn(
[01:15:06] 161 | /     fn check_fn(
[01:15:06] 162 | |         &mut self,
[01:15:06] 163 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 164 | |         kind: FnKind<'tcx>,
[01:15:06] ...   |
[01:15:06] 203 | |         self.check_poly_fn(cx, decl, &fn_sig, Some(span));
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut trivially_copy_pass_by_ref::TriviallyCopyPassByRef, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut trivially_copy_pass_by_ref::TriviallyCopyPassByRef, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/types.rs:178:5
[01:15:06]     |
[01:15:06] 178 |     fn check_fn(&mut self, cx: &LateContext<'_, '_>, _: FnKind<'_>, decl: &FnDecl, _: &Body, _: Span, id: NodeId) {
[01:15:06] 178 |     fn check_fn(&mut self, cx: &LateContext<'_, '_>, _: FnKind<'_>, decl: &FnDecl, _: &Body, _: Span, id: NodeId) {
[01:15:06]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut types::TypePass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut types::TypePass, &rustc::lint::LateContext<'_, '_>, rustc::hir::intravisit::FnKind<'_>, &rustc::hir::FnDecl, &rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]     --> src/tools/clippy/clippy_lints/src/types.rs:1332:5
[01:15:06]      |
[01:15:06] 1332 | /     fn check_fn(
---
[01:15:06] 1341 | |         self.check_fndecl(cx, decl);
[01:15:06] 1342 | |     }
[01:15:06]      | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]      |
[01:15:06]      = note: expected type `fn(&mut types::TypeComplexityPass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                 found type `fn(&mut types::TypeComplexityPass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]   --> src/tools/clippy/clippy_lints/src/unused_label.rs:49:5
[01:15:06]    |
[01:15:06] 49 | /     fn check_fn(
---
[01:15:06] 70 | |         }
[01:15:06] 71 | |     }
[01:15:06]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut unused_label::UnusedLabel, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut unused_label::UnusedLabel, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/unwrap.rs:194:5
[01:15:06]     |
[01:15:06] 194 | /     fn check_fn(
[01:15:06] 194 | /     fn check_fn(
[01:15:06] 195 | |         &mut self,
[01:15:06] 196 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 197 | |         kind: FnKind<'tcx>,
[01:15:06] ...   |
[01:15:06] 212 | |         walk_fn(&mut v, kind, decl, body.id(), span, fn_id);
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut unwrap::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut unwrap::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]   --> src/tools/clippy/clippy_lints/src/booleans.rs:68:5
[01:15:06]    |
[01:15:06] 68 | /     fn check_fn(
[01:15:06] 68 | /     fn check_fn(
[01:15:06] 69 | |         &mut self,
[01:15:06] 70 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 71 | |         _: intravisit::FnKind<'tcx>,
[01:15:06] ...  |
[01:15:06] 77 | |         NonminimalBoolVisitor { cx }.visit_body(body)
[01:15:06]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut booleans::NonminimalBool, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut booleans::NonminimalBool, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
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
[01:15:06] error[E0053]: method `check_mod` has an incompatible type for trait
[01:15:06]   --> src/tools/clippy/clippy_lints/src/enum_glob_use.rs:42:5
[01:15:06]    |
[01:15:06]    |
[01:15:06] 42 |     fn check_mod(&mut self, cx: &LateContext<'a, 'tcx>, m: &'tcx Mod, _: Span, _: NodeId) {
[01:15:06]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut enum_glob_use::EnumGlobUse, &rustc::lint::LateContext<'a, 'tcx>, &'tcx rustc::hir::Mod, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut enum_glob_use::EnumGlobUse, &rustc::lint::LateContext<'a, 'tcx>, &'tcx rustc::hir::Mod, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]   --> src/tools/clippy/clippy_lints/src/escape.rs:62:5
[01:15:06]    |
[01:15:06] 62 | /     fn check_fn(
[01:15:06] 62 | /     fn check_fn(
[01:15:06] 63 | |         &mut self,
[01:15:06] 64 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 65 | |         _: visit::FnKind<'tcx>,
[01:15:06] 98 | |         }
[01:15:06] 99 | |     }
[01:15:06]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]    |
[01:15:06]    |
[01:15:06]    = note: expected type `fn(&mut escape::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]               found type `fn(&mut escape::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/functions.rs:108:5
[01:15:06]     |
[01:15:06] 108 | /     fn check_fn(
---
[01:15:06] 150 | |         self.check_line_number(cx, span);
[01:15:06] 151 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut functions::Functions, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut functions::Functions, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/implicit_return.rs:124:5
[01:15:06]     |
[01:15:06] 124 | /     fn check_fn(
---
[01:15:06] 140 | |         }
[01:15:06] 141 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut implicit_return::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut implicit_return::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error: aborting due to 16 previous errors
[01:15:06] 
[01:15:06] For more information about this error, try `rustc --explain E0053`.
[01:15:06] [RUSTC-TIMING] clippy_lints test:false 4.270
---
[01:15:06]     |
[01:15:06] 252 | /     fn check_fn(
[01:15:06] 253 | |         &mut self,
[01:15:06] 254 | |         cx: &LateContext<'a, 'tcx>,
[01:15:06] 255 | |         k: FnKind<'tcx>,
[01:15:06] 278 | |         }
[01:15:06] 279 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut misc::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut misc::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:06] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:06]    --> src/tools/clippy/clippy_lints/src/missing_const_for_fn.rs:75:5
[01:15:06]     |
[01:15:06] 75  | /     fn check_fn(
---
[01:15:06] 114 | |         }
[01:15:06] 115 | |     }
[01:15:06]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:06]     |
[01:15:06]     = note: expected type `fn(&mut missing_const_for_fn::MissingConstForFn, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:06]                found type `fn(&mut missing_const_for_fn::MissingConstForFn, &rustc::lint::LateContext<'_, '_>, rustc::hir::intravisit::FnKind<'_>, &rustc::hir::FnDecl, &rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]    --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:77:5
[01:15:07]     |
[01:15:07] 77  | /     fn check_fn(
---
[01:15:07] 319 | |         }
[01:15:07] 320 | |     }
[01:15:07]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]     |
[01:15:07]     = note: expected type `fn(&mut needless_pass_by_value::NeedlessPassByValue, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                found type `fn(&mut needless_pass_by_value::NeedlessPassByValue, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:84:5
[01:15:07]     |
[01:15:07] 84  | /     fn check_fn(
---
[01:15:07] 222 | |         }
[01:15:07] 223 | |     }
[01:15:07]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]     |
[01:15:07]     = note: expected type `fn(&mut redundant_clone::RedundantClone, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                found type `fn(&mut redundant_clone::RedundantClone, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]    --> src/tools/clippy/clippy_lints/src/shadow.rs:92:5
[01:15:07]     |
[01:15:07] 92  | /     fn check_fn(
---
[01:15:07] 104 | |         check_fn(cx, decl, body);
[01:15:07] 105 | |     }
[01:15:07]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]     |
[01:15:07]     = note: expected type `fn(&mut shadow::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                found type `fn(&mut shadow::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]    --> src/tools/clippy/clippy_lints/src/trivially_copy_pass_by_ref.rs:161:5
[01:15:07]     |
[01:15:07] 161 | /     fn check_fn(
[01:15:07] 161 | /     fn check_fn(
[01:15:07] 162 | |         &mut self,
[01:15:07] 163 | |         cx: &LateContext<'a, 'tcx>,
[01:15:07] 164 | |         kind: FnKind<'tcx>,
[01:15:07] ...   |
[01:15:07] 203 | |         self.check_poly_fn(cx, decl, &fn_sig, Some(span));
[01:15:07]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]     |
[01:15:07]     |
[01:15:07]     = note: expected type `fn(&mut trivially_copy_pass_by_ref::TriviallyCopyPassByRef, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                found type `fn(&mut trivially_copy_pass_by_ref::TriviallyCopyPassByRef, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]    --> src/tools/clippy/clippy_lints/src/types.rs:178:5
[01:15:07]     |
[01:15:07] 178 |     fn check_fn(&mut self, cx: &LateContext<'_, '_>, _: FnKind<'_>, decl: &FnDecl, _: &Body, _: Span, id: NodeId) {
[01:15:07] 178 |     fn check_fn(&mut self, cx: &LateContext<'_, '_>, _: FnKind<'_>, decl: &FnDecl, _: &Body, _: Span, id: NodeId) {
[01:15:07]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]     |
[01:15:07]     = note: expected type `fn(&mut types::TypePass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                found type `fn(&mut types::TypePass, &rustc::lint::LateContext<'_, '_>, rustc::hir::intravisit::FnKind<'_>, &rustc::hir::FnDecl, &rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]     --> src/tools/clippy/clippy_lints/src/types.rs:1332:5
[01:15:07]      |
[01:15:07] 1332 | /     fn check_fn(
---
[01:15:07] 1341 | |         self.check_fndecl(cx, decl);
[01:15:07] 1342 | |     }
[01:15:07]      | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]      |
[01:15:07]      = note: expected type `fn(&mut types::TypeComplexityPass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                 found type `fn(&mut types::TypeComplexityPass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]   --> src/tools/clippy/clippy_lints/src/unused_label.rs:49:5
[01:15:07]    |
[01:15:07] 49 | /     fn check_fn(
---
[01:15:07] 70 | |         }
[01:15:07] 71 | |     }
[01:15:07]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]    |
[01:15:07]    = note: expected type `fn(&mut unused_label::UnusedLabel, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]               found type `fn(&mut unused_label::UnusedLabel, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error[E0053]: method `check_fn` has an incompatible type for trait
[01:15:07]    --> src/tools/clippy/clippy_lints/src/unwrap.rs:194:5
[01:15:07]     |
[01:15:07] 194 | /     fn check_fn(
[01:15:07] 194 | /     fn check_fn(
[01:15:07] 195 | |         &mut self,
[01:15:07] 196 | |         cx: &LateContext<'a, 'tcx>,
[01:15:07] 197 | |         kind: FnKind<'tcx>,
[01:15:07] ...   |
[01:15:07] 212 | |         walk_fn(&mut v, kind, decl, body.id(), span, fn_id);
[01:15:07]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:15:07]     |
[01:15:07]     |
[01:15:07]     = note: expected type `fn(&mut unwrap::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:15:07]                found type `fn(&mut unwrap::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:15:07] error: aborting due to 16 previous errors
[01:15:07] 
[01:15:07] For more information about this error, try `rustc --explain E0053`.
[01:15:07] error: Could not compile `clippy_lints`.
---
[01:19:21]    |
[01:19:21] 68 | /     fn check_fn(
[01:19:21] 69 | |         &mut self,
[01:19:21] 70 | |         cx: &LateContext<'a, 'tcx>,
[01:19:21] 71 | |         _: intravisit::FnKind<'tcx>,
[01:19:21] ...  |
[01:19:21] 77 | |         NonminimalBoolVisitor { cx }.visit_body(body)
[01:19:21]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]    |
[01:19:21]    |
[01:19:21]    = note: expected type `fn(&mut booleans::NonminimalBool, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]               found type `fn(&mut booleans::NonminimalBool, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/cyclomatic_complexity.rs:119:5
[01:19:21]     |
[01:19:21] 119 | /     fn check_fn(
[01:19:21] 119 | /     fn check_fn(
[01:19:21] 120 | |         &mut self,
[01:19:21] 121 | |         cx: &LateContext<'a, 'tcx>,
[01:19:21] 122 | |         _: intravisit::FnKind<'tcx>,
[01:19:21] 131 | |         }
[01:19:21] 132 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut cyclomatic_complexity::CyclomaticComplexity, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut cyclomatic_complexity::CyclomaticComplexity, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_mod` has an incompatible type for trait
[01:19:21]   --> src/tools/clippy/clippy_lints/src/enum_glob_use.rs:42:5
[01:19:21]    |
[01:19:21]    |
[01:19:21] 42 |     fn check_mod(&mut self, cx: &LateContext<'a, 'tcx>, m: &'tcx Mod, _: Span, _: NodeId) {
[01:19:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]    |
[01:19:21]    = note: expected type `fn(&mut enum_glob_use::EnumGlobUse, &rustc::lint::LateContext<'a, 'tcx>, &'tcx rustc::hir::Mod, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]               found type `fn(&mut enum_glob_use::EnumGlobUse, &rustc::lint::LateContext<'a, 'tcx>, &'tcx rustc::hir::Mod, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]   --> src/tools/clippy/clippy_lints/src/escape.rs:62:5
[01:19:21]    |
[01:19:21] 62 | /     fn check_fn(
[01:19:21] 62 | /     fn check_fn(
[01:19:21] 63 | |         &mut self,
[01:19:21] 64 | |         cx: &LateContext<'a, 'tcx>,
[01:19:21] 65 | |         _: visit::FnKind<'tcx>,
[01:19:21] 98 | |         }
[01:19:21] 99 | |     }
[01:19:21]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]    |
[01:19:21]    |
[01:19:21]    = note: expected type `fn(&mut escape::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]               found type `fn(&mut escape::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/functions.rs:108:5
[01:19:21]     |
[01:19:21] 108 | /     fn check_fn(
---
[01:19:21] 150 | |         self.check_line_number(cx, span);
[01:19:21] 151 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut functions::Functions, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut functions::Functions, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/implicit_return.rs:124:5
[01:19:21]     |
[01:19:21] 124 | /     fn check_fn(
---
[01:19:21] 140 | |         }
[01:19:21] 141 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut implicit_return::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut implicit_return::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/misc.rs:252:5
[01:19:21]     |
[01:19:21] 252 | /     fn check_fn(
[01:19:21] 252 | /     fn check_fn(
[01:19:21] 253 | |         &mut self,
[01:19:21] 254 | |         cx: &LateContext<'a, 'tcx>,
[01:19:21] 255 | |         k: FnKind<'tcx>,
[01:19:21] 278 | |         }
[01:19:21] 279 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut misc::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut misc::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/missing_const_for_fn.rs:75:5
[01:19:21]     |
[01:19:21] 75  | /     fn check_fn(
---
[01:19:21] 114 | |         }
[01:19:21] 115 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut missing_const_for_fn::MissingConstForFn, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut missing_const_for_fn::MissingConstForFn, &rustc::lint::LateContext<'_, '_>, rustc::hir::intravisit::FnKind<'_>, &rustc::hir::FnDecl, &rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:77:5
[01:19:21]     |
[01:19:21] 77  | /     fn check_fn(
---
[01:19:21] 319 | |         }
[01:19:21] 320 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut needless_pass_by_value::NeedlessPassByValue, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut needless_pass_by_value::NeedlessPassByValue, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/redundant_clone.rs:84:5
[01:19:21]     |
[01:19:21] 84  | /     fn check_fn(
---
[01:19:21] 222 | |         }
[01:19:21] 223 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut redundant_clone::RedundantClone, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut redundant_clone::RedundantClone, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/shadow.rs:92:5
[01:19:21]     |
[01:19:21] 92  | /     fn check_fn(
---
[01:19:21] 104 | |         check_fn(cx, decl, body);
[01:19:21] 105 | |     }
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut shadow::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut shadow::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/trivially_copy_pass_by_ref.rs:161:5
[01:19:21]     |
[01:19:21] 161 | /     fn check_fn(
[01:19:21] 161 | /     fn check_fn(
[01:19:21] 162 | |         &mut self,
[01:19:21] 163 | |         cx: &LateContext<'a, 'tcx>,
[01:19:21] 164 | |         kind: FnKind<'tcx>,
[01:19:21] ...   |
[01:19:21] 203 | |         self.check_poly_fn(cx, decl, &fn_sig, Some(span));
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut trivially_copy_pass_by_ref::TriviallyCopyPassByRef, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut trivially_copy_pass_by_ref::TriviallyCopyPassByRef, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/types.rs:178:5
[01:19:21]     |
[01:19:21] 178 |     fn check_fn(&mut self, cx: &LateContext<'_, '_>, _: FnKind<'_>, decl: &FnDecl, _: &Body, _: Span, id: NodeId) {
[01:19:21] 178 |     fn check_fn(&mut self, cx: &LateContext<'_, '_>, _: FnKind<'_>, decl: &FnDecl, _: &Body, _: Span, id: NodeId) {
[01:19:21]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut types::TypePass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut types::TypePass, &rustc::lint::LateContext<'_, '_>, rustc::hir::intravisit::FnKind<'_>, &rustc::hir::FnDecl, &rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]     --> src/tools/clippy/clippy_lints/src/types.rs:1332:5
[01:19:21]      |
[01:19:21] 1332 | /     fn check_fn(
---
[01:19:21] 1341 | |         self.check_fndecl(cx, decl);
[01:19:21] 1342 | |     }
[01:19:21]      | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]      |
[01:19:21]      = note: expected type `fn(&mut types::TypeComplexityPass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                 found type `fn(&mut types::TypeComplexityPass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]   --> src/tools/clippy/clippy_lints/src/unused_label.rs:49:5
[01:19:21]    |
[01:19:21] 49 | /     fn check_fn(
---
[01:19:21] 70 | |         }
[01:19:21] 71 | |     }
[01:19:21]    | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]    |
[01:19:21]    = note: expected type `fn(&mut unused_label::UnusedLabel, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]               found type `fn(&mut unused_label::UnusedLabel, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error[E0053]: method `check_fn` has an incompatible type for trait
[01:19:21]    --> src/tools/clippy/clippy_lints/src/unwrap.rs:194:5
[01:19:21]     |
[01:19:21] 194 | /     fn check_fn(
[01:19:21] 194 | /     fn check_fn(
[01:19:21] 195 | |         &mut self,
[01:19:21] 196 | |         cx: &LateContext<'a, 'tcx>,
[01:19:21] 197 | |         kind: FnKind<'tcx>,
[01:19:21] ...   |
[01:19:21] 212 | |         walk_fn(&mut v, kind, decl, body.id(), span, fn_id);
[01:19:21]     | |_____^ expected struct `rustc::hir::HirId`, found struct `syntax::ast::NodeId`
[01:19:21]     |
[01:19:21]     |
[01:19:21]     = note: expected type `fn(&mut unwrap::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, rustc::hir::HirId)`
[01:19:21]                found type `fn(&mut unwrap::Pass, &rustc::lint::LateContext<'a, 'tcx>, rustc::hir::intravisit::FnKind<'tcx>, &'tcx rustc::hir::FnDecl, &'tcx rustc::hir::Body, syntax_pos::Span, syntax::ast::NodeId)`
[01:19:21] error: aborting due to 16 previous errors
[01:19:21] 
[01:19:21] For more information about this error, try `rustc --explain E0053`.
[01:19:21] [RUSTC-TIMING] clippy_lints test:false 6.296
---
travis_time:end:092b99e0:start=1550630981944909453,finish=1550630981951768299,duration=6858846
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12f97058
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:017f746c
travis_time:start:017f746c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2841a0e0
$ dmesg | grep -i kill
