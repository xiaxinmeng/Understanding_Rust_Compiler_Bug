
[00:59:37] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::LitKind`
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:110:51
[00:59:37]     |
[00:59:37] 110 |             (&ExprLit(ref l), &ExprLit(ref r)) => l.node == r.node,
[00:59:37]     |                                                   ^^^^^^^^^^^^^^^^
[00:59:37]     |
[00:59:37]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::LitKind`
[00:59:37]
[00:59:37] error[E0369]: binary operation `==` cannot be applied to type `&rustc::hir::PathSegment`
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:121:36
[00:59:37]     |
[00:59:37] 121 |                 !self.ignore_fn && l_path == r_path && self.eq_exprs(l_args, r_args)
[00:59:37]     |                                    ^^^^^^^^^^^^^^^^
[00:59:37]     |
[00:59:37]     = note: an implementation of `std::cmp::PartialEq` might be missing for `&rustc::hir::PathSegment`
[00:59:37]
[00:59:37] error[E0599]: no method named `hash` found for type `rustc::hir::BlockCheckMode` in the current scope
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:339:17
[00:59:37]     |
[00:59:37] 339 |         b.rules.hash(&mut self.s);
[00:59:37]     |                 ^^^^
[00:59:37]
[00:59:37] error[E0599]: no method named `hash` found for type `&syntax::codemap::Spanned<rustc::hir::BinOp_>` in the current scope
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:376:19
[00:59:37]     |
[00:59:37] 376 |                 o.hash(&mut self.s);
[00:59:37]     |                   ^^^^
[00:59:37]     |
[00:59:37]     = note: the method `hash` exists but the following trait bounds were not satisfied:
[00:59:37]             `syntax::codemap::Spanned<rustc::hir::BinOp_> : std::hash::Hash`
[00:59:37]             `&syntax::codemap::Spanned<rustc::hir::BinOp_> : std::hash::Hash`
[00:59:37]             `syntax::codemap::Spanned<rustc::hir::BinOp_> : std::hash::Hash`
[00:59:37]
[00:59:37] error[E0599]: no method named `hash` found for type `rustc::hir::BinOp_` in the current scope
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:388:25
[00:59:37]     |
[00:59:37] 388 |                 op.node.hash(&mut self.s);
[00:59:37]     |                         ^^^^
[00:59:37]
[00:59:37] error[E0599]: no method named `hash` found for type `rustc::hir::CaptureClause` in the current scope
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:422:21
[00:59:37]     |
[00:59:37] 422 |                 cap.hash(&mut self.s);
[00:59:37]     |                     ^^^^
[00:59:37]
[00:59:37] error[E0599]: no method named `hash` found for type `&syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>>` in the current scope
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:453:19
[00:59:37]     |
[00:59:37] 453 |                 l.hash(&mut self.s);
[00:59:37]     |                   ^^^^
[00:59:37]     |
[00:59:37]     = note: the method `hash` exists but the following trait bounds were not satisfied:
[00:59:37]             `syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>> : std::hash::Hash`
[00:59:37]             `&syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>> : std::hash::Hash`
[00:59:37]             `syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>> : std::hash::Hash`
[00:59:37]             `syntax::codemap::Spanned<syntax::ast::LitKind> : std::hash::Hash`
[00:59:37]
[00:59:37] error[E0599]: no method named `hash` found for type `rustc::hir::UnOp` in the current scope
[00:59:37]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:535:21
[00:59:37]     |
[00:59:37] 535 |                 lop.hash(&mut self.s);
[00:59:37]     |                     ^^^^
[00:59:37]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::LitKind`
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:110:51
[00:59:38]     |
[00:59:38] 110 |             (&ExprLit(ref l), &ExprLit(ref r)) => l.node == r.node,
[00:59:38]     |                                                   ^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::LitKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `&rustc::hir::PathSegment`
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:121:36
[00:59:38]     |
[00:59:38] 121 |                 !self.ignore_fn && l_path == r_path && self.eq_exprs(l_args, r_args)
[00:59:38]     |                                    ^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `&rustc::hir::PathSegment`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::VisibilityKind`
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/internal_lints.rs:123:49
[00:59:38]     |
[00:59:38] 123 |             } else if is_lint_array_type(ty) && item.vis.node == VisibilityKind::Inherited && item.name == "ARRAY" {
[00:59:38]     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::VisibilityKind`
[00:59:38]
[00:59:38] error[E0599]: no method named `hash` found for type `rustc::hir::BlockCheckMode` in the current scope
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:339:17
[00:59:38]     |
[00:59:38] 339 |         b.rules.hash(&mut self.s);
[00:59:38]     |                 ^^^^
[00:59:38]
[00:59:38] error[E0599]: no method named `hash` found for type `&syntax::codemap::Spanned<rustc::hir::BinOp_>` in the current scope
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:376:19
[00:59:38]     |
[00:59:38] 376 |                 o.hash(&mut self.s);
[00:59:38]     |                   ^^^^
[00:59:38]     |
[00:59:38]     = note: the method `hash` exists but the following trait bounds were not satisfied:
[00:59:38]             `syntax::codemap::Spanned<rustc::hir::BinOp_> : std::hash::Hash`
[00:59:38]             `&syntax::codemap::Spanned<rustc::hir::BinOp_> : std::hash::Hash`
[00:59:38]             `syntax::codemap::Spanned<rustc::hir::BinOp_> : std::hash::Hash`
[00:59:38]
[00:59:38] error[E0599]: no method named `hash` found for type `rustc::hir::BinOp_` in the current scope
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:388:25
[00:59:38]     |
[00:59:38] 388 |                 op.node.hash(&mut self.s);
[00:59:38]     |                         ^^^^
[00:59:38]
[00:59:38] error[E0599]: no method named `hash` found for type `rustc::hir::CaptureClause` in the current scope
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:422:21
[00:59:38]     |
[00:59:38] 422 |                 cap.hash(&mut self.s);
[00:59:38]     |                     ^^^^
[00:59:38]
[00:59:38] error[E0599]: no method named `hash` found for type `&syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>>` in the current scope
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:453:19
[00:59:38]     |
[00:59:38] 453 |                 l.hash(&mut self.s);
[00:59:38]     |                   ^^^^
[00:59:38]     |
[00:59:38]     = note: the method `hash` exists but the following trait bounds were not satisfied:
[00:59:38]             `syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>> : std::hash::Hash`
[00:59:38]             `&syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>> : std::hash::Hash`
[00:59:38]             `syntax::ptr::P<syntax::codemap::Spanned<syntax::ast::LitKind>> : std::hash::Hash`
[00:59:38]             `syntax::codemap::Spanned<syntax::ast::LitKind> : std::hash::Hash`
[00:59:38]
[00:59:38] error[E0599]: no method named `hash` found for type `rustc::hir::UnOp` in the current scope
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/hir_utils.rs:535:21
[00:59:38]     |
[00:59:38] 535 |                 lop.hash(&mut self.s);
[00:59:38]     |                     ^^^^
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::VisibilityKind`
[00:59:38]    --> tools/clippy/clippy_lints/src/utils/internal_lints.rs:123:49
[00:59:38]     |
[00:59:38] 123 |             } else if is_lint_array_type(ty) && item.vis.node == VisibilityKind::Inherited && item.name == "ARRAY" {
[00:59:38]     |                                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::VisibilityKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::VisibilityKind`
[00:59:38]   --> tools/clippy/clippy_lints/src/enum_glob_use.rs:47:12
[00:59:38]    |
[00:59:38] 47 |         if item.vis.node == VisibilityKind::Public {
[00:59:38]    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]    |
[00:59:38]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::VisibilityKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::VisibilityKind`
[00:59:38]    --> tools/clippy/clippy_lints/src/enum_variants.rs:265:24
[00:59:38]     |
[00:59:38] 265 |                     if item.vis.node == VisibilityKind::Public {
[00:59:38]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::VisibilityKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:38]   --> tools/clippy/clippy_lints/src/if_let_redundant_pattern_matching.rs:51:87
[00:59:38]    |
[00:59:38] 51 |                     PatKind::TupleStruct(ref path, ref pats, _) if pats.len() == 1 && pats[0].node == PatKind::Wild => {
[00:59:38]    |                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]    |
[00:59:38]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::VisibilityKind`
[00:59:38]   --> tools/clippy/clippy_lints/src/enum_glob_use.rs:47:12
[00:59:38]    |
[00:59:38] 47 |         if item.vis.node == VisibilityKind::Public {
[00:59:38]    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]    |
[00:59:38]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::VisibilityKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::VisibilityKind`
[00:59:38]    --> tools/clippy/clippy_lints/src/enum_variants.rs:265:24
[00:59:38]     |
[00:59:38] 265 |                     if item.vis.node == VisibilityKind::Public {
[00:59:38]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::VisibilityKind`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `&rustc::hir::Expr`
[00:59:38]     --> tools/clippy/clippy_lints/src/loops.rs:1958:12
[00:59:38]      |
[00:59:38] 1958 |         if expr == self.end_expr {
[00:59:38]      |            ^^^^^^^^^^^^^^^^^^^^^
[00:59:38]      |
[00:59:38]      = note: an implementation of `std::cmp::PartialEq` might be missing for `&rustc::hir::Expr`
[00:59:38]
[00:59:38] error[E0369]: binary operation `==` cannot be applied to type `[rustc::hir::PathSegment]`
[00:59:38]    --> tools/clippy/clippy_lints/src/map_clone.rs:111:34
[00:59:38]     |
[00:59:38] 111 |             !path.is_global() && path.segments[..] == arg_segment
[00:59:38]     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:38]     |
[00:59:38]     = note: an implementation of `std::cmp::PartialEq` might be missing for `[rustc::hir::PathSegment]`
[00:59:38]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]   --> tools/clippy/clippy_lints/src/if_let_redundant_pattern_matching.rs:51:87
[00:59:39]    |
[00:59:39] 51 |                     PatKind::TupleStruct(ref path, ref pats, _) if pats.len() == 1 && pats[0].node == PatKind::Wild => {
[00:59:39]    |                                                                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/matches.rs:224:8
[00:59:39]     |
[00:59:39] 224 |     if arms[1].pats[0].node == PatKind::Wild {
[00:59:39]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `!=` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/matches.rs:268:39
[00:59:39]     |
[00:59:39] 268 |             if inner.iter().any(|pat| pat.node != PatKind::Wild) {
[00:59:39]     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/matches.rs:367:47
[00:59:39]     |
[00:59:39] 367 |                     if inner.iter().any(|pat| pat.node == PatKind::Wild);
[00:59:39]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::VisibilityKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/methods.rs:843:43
[00:59:39]     |
[00:59:39] 843 |                             let lint = if item.vis.node == hir::VisibilityKind::Public {
[00:59:39]     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::VisibilityKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `&rustc::hir::Ty`
[00:59:39]     --> tools/clippy/clippy_lints/src/methods.rs:2050:55
[00:59:39]      |
[00:59:39] 2050 |         let is_actually_self = |ty| is_self_ty(ty) || ty == self_ty;
[00:59:39]      |                                                       ^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `&rustc::hir::Ty`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::Ty_`
[00:59:39]     --> tools/clippy/clippy_lints/src/methods.rs:2179:54
[00:59:39]      |
[00:59:39] 2179 |             (OutType::Unit, &hir::Return(ref ty)) if ty.node == hir::TyTup(vec![].into()) => true,
[00:59:39]      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::Ty_`
[00:59:39]
[00:59:39] error[E0369]: binary operation `!=` cannot be applied to type `rustc::hir::Ty_`
[00:59:39]     --> tools/clippy/clippy_lints/src/methods.rs:2181:53
[00:59:39]      |
[00:59:39] 2181 |             (OutType::Any, &hir::Return(ref ty)) if ty.node != hir::TyTup(vec![].into()) => true,
[00:59:39]      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::Ty_`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc.rs:421:16
[00:59:39]     |
[00:59:39] 421 |             if right.node == PatKind::Wild {
[00:59:39]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::Expr`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc.rs:545:69
[00:59:39]     |
[00:59:39] 545 |             ExprAssign(_, ref rhs) | ExprAssignOp(_, _, ref rhs) => **rhs == *expr,
[00:59:39]     |                                                                     ^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::Expr`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc_early.rs:216:20
[00:59:39]     |
[00:59:39] 216 |                 if field.node.pat.node == PatKind::Wild {
[00:59:39]     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `!=` cannot be applied to type `syntax::ast::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc_early.rs:234:24
[00:59:39]     |
[00:59:39] 234 |                     if field.node.pat.node != PatKind::Wild {
[00:59:39]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc_early.rs:241:24
[00:59:39]     |
[00:59:39] 241 |                     if field.node.pat.node == PatKind::Wild {
[00:59:39]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `&rustc::hir::Expr`
[00:59:39]     --> tools/clippy/clippy_lints/src/loops.rs:1958:12
[00:59:39]      |
[00:59:39] 1958 |         if expr == self.end_expr {
[00:59:39]      |            ^^^^^^^^^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `&rustc::hir::Expr`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `[rustc::hir::PathSegment]`
[00:59:39]    --> tools/clippy/clippy_lints/src/map_clone.rs:111:34
[00:59:39]     |
[00:59:39] 111 |             !path.is_global() && path.segments[..] == arg_segment
[00:59:39]     |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `[rustc::hir::PathSegment]`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/matches.rs:224:8
[00:59:39]     |
[00:59:39] 224 |     if arms[1].pats[0].node == PatKind::Wild {
[00:59:39]     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `!=` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/matches.rs:268:39
[00:59:39]     |
[00:59:39] 268 |             if inner.iter().any(|pat| pat.node != PatKind::Wild) {
[00:59:39]     |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/matches.rs:367:47
[00:59:39]     |
[00:59:39] 367 |                     if inner.iter().any(|pat| pat.node == PatKind::Wild);
[00:59:39]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:40:16
[00:59:39]    |
[00:59:39] 40 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:40:58
[00:59:39]    |
[00:59:39] 40 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:65:16
[00:59:39]    |
[00:59:39] 65 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:65:58
[00:59:39]    |
[00:59:39] 65 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::VisibilityKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/methods.rs:843:43
[00:59:39]     |
[00:59:39] 843 |                             let lint = if item.vis.node == hir::VisibilityKind::Public {
[00:59:39]     |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::VisibilityKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `syntax::ptr::P<[rustc::hir::PathSegment]>`
[00:59:39]    --> tools/clippy/clippy_lints/src/ranges.rs:121:24
[00:59:39]     |
[00:59:39] 121 |                     if iter_path.segments == len_path.segments;
[00:59:39]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ptr::P<[rustc::hir::PathSegment]>`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `&rustc::hir::Ty`
[00:59:39]     --> tools/clippy/clippy_lints/src/methods.rs:2050:55
[00:59:39]      |
[00:59:39] 2050 |         let is_actually_self = |ty| is_self_ty(ty) || ty == self_ty;
[00:59:39]      |                                                       ^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `&rustc::hir::Ty`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::Ty_`
[00:59:39]     --> tools/clippy/clippy_lints/src/methods.rs:2179:54
[00:59:39]      |
[00:59:39] 2179 |             (OutType::Unit, &hir::Return(ref ty)) if ty.node == hir::TyTup(vec![].into()) => true,
[00:59:39]      |                                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::Ty_`
[00:59:39]
[00:59:39] error[E0369]: binary operation `!=` cannot be applied to type `rustc::hir::Ty_`
[00:59:39]     --> tools/clippy/clippy_lints/src/methods.rs:2181:53
[00:59:39]      |
[00:59:39] 2181 |             (OutType::Any, &hir::Return(ref ty)) if ty.node != hir::TyTup(vec![].into()) => true,
[00:59:39]      |                                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]      |
[00:59:39]      = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::Ty_`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc.rs:421:16
[00:59:39]     |
[00:59:39] 421 |             if right.node == PatKind::Wild {
[00:59:39]     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::Expr`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc.rs:545:69
[00:59:39]     |
[00:59:39] 545 |             ExprAssign(_, ref rhs) | ExprAssignOp(_, _, ref rhs) => **rhs == *expr,
[00:59:39]     |                                                                     ^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::Expr`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc_early.rs:216:20
[00:59:39]     |
[00:59:39] 216 |                 if field.node.pat.node == PatKind::Wild {
[00:59:39]     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `!=` cannot be applied to type `syntax::ast::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc_early.rs:234:24
[00:59:39]     |
[00:59:39] 234 |                     if field.node.pat.node != PatKind::Wild {
[00:59:39]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::PatKind`
[00:59:39]    --> tools/clippy/clippy_lints/src/misc_early.rs:241:24
[00:59:39]     |
[00:59:39] 241 |                     if field.node.pat.node == PatKind::Wild {
[00:59:39]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::PatKind`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:40:16
[00:59:39]    |
[00:59:39] 40 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:40:58
[00:59:39]    |
[00:59:39] 40 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:65:16
[00:59:39]    |
[00:59:39] 65 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `rustc::hir::PathSegment`
[00:59:39]   --> tools/clippy/clippy_lints/src/overflow_check_conditional.rs:65:58
[00:59:39]    |
[00:59:39] 65 |             if path1.segments[0] == path3.segments[0] || path2.segments[0] == path3.segments[0];
[00:59:39]    |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]    |
[00:59:39]    = note: an implementation of `std::cmp::PartialEq` might be missing for `rustc::hir::PathSegment`
[00:59:39]
[00:59:39] error[E0369]: binary operation `==` cannot be applied to type `syntax::ptr::P<[rustc::hir::PathSegment]>`
[00:59:39]    --> tools/clippy/clippy_lints/src/ranges.rs:121:24
[00:59:39]     |
[00:59:39] 121 |                     if iter_path.segments == len_path.segments;
[00:59:39]     |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:59:39]     |
[00:59:39]     = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ptr::P<[rustc::hir::PathSegment]>`
[00:59:39]
[00:59:40] error: aborting due to 31 previous errors
