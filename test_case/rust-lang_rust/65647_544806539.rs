`
error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::LitKind`
   --> clippy_lints/src/utils/hir_utils.rs:117:70
    |
117 |             (&ExprKind::Lit(ref l), &ExprKind::Lit(ref r)) => l.node == r.node,
    |                                                               ------ ^^ ------ syntax::ast::LitKind
    |                                                               |
    |                                                               syntax::ast::LitKind
    |
    = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::LitKind`

error[E0599]: no method named `hash` found for type `&syntax::source_map::Spanned<rustc::hir::BinOpKind>` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:414:19
    |
414 |                 o.hash(&mut self.s);
    |                   ^^^^ method not found in `&syntax::source_map::Spanned<rustc::hir::BinOpKind>`
    |
    = note: the method `hash` exists but the following trait bounds were not satisfied:
            `&syntax::source_map::Spanned<rustc::hir::BinOpKind> : std::hash::Hash`

error[E0599]: no method named `hash` found for type `rustc::hir::BinOpKind` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:422:25
    |
422 |                 op.node.hash(&mut self.s);
    |                         ^^^^ method not found in `rustc::hir::BinOpKind`

error[E0599]: no method named `hash` found for type `&syntax::source_map::Spanned<syntax::ast::LitKind>` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:463:19
    |
463 |                 l.hash(&mut self.s);
    |                   ^^^^ method not found in `&syntax::source_map::Spanned<syntax::ast::LitKind>`
    |
    = note: the method `hash` exists but the following trait bounds were not satisfied:
            `&syntax::source_map::Spanned<syntax::ast::LitKind> : std::hash::Hash`

error[E0599]: no method named `hash` found for type `rustc::hir::UnOp` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:522:21
    |
522 |                 lop.hash(&mut self.s);
    |                     ^^^^ method not found in `rustc::hir::UnOp`

error: aborting due to 5 previous errors

Some errors have detailed explanations: E0369, E0599.
For more information about an error, try `rustc --explain E0369`.
error: could not compile `clippy_lints`.
warning: build failed, waiting for other jobs to finish...
error[E0369]: binary operation `==` cannot be applied to type `syntax::ast::LitKind`
   --> clippy_lints/src/utils/hir_utils.rs:117:70
    |
117 |             (&ExprKind::Lit(ref l), &ExprKind::Lit(ref r)) => l.node == r.node,
    |                                                               ------ ^^ ------ syntax::ast::LitKind
    |                                                               |
    |                                                               syntax::ast::LitKind
    |
    = note: an implementation of `std::cmp::PartialEq` might be missing for `syntax::ast::LitKind`

error[E0599]: no method named `hash` found for type `&syntax::source_map::Spanned<rustc::hir::BinOpKind>` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:414:19
    |
414 |                 o.hash(&mut self.s);
    |                   ^^^^ method not found in `&syntax::source_map::Spanned<rustc::hir::BinOpKind>`
    |
    = note: the method `hash` exists but the following trait bounds were not satisfied:
            `&syntax::source_map::Spanned<rustc::hir::BinOpKind> : std::hash::Hash`

error[E0599]: no method named `hash` found for type `rustc::hir::BinOpKind` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:422:25
    |
422 |                 op.node.hash(&mut self.s);
    |                         ^^^^ method not found in `rustc::hir::BinOpKind`

error[E0599]: no method named `hash` found for type `&syntax::source_map::Spanned<syntax::ast::LitKind>` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:463:19
    |
463 |                 l.hash(&mut self.s);
    |                   ^^^^ method not found in `&syntax::source_map::Spanned<syntax::ast::LitKind>`
    |
    = note: the method `hash` exists but the following trait bounds were not satisfied:
            `&syntax::source_map::Spanned<syntax::ast::LitKind> : std::hash::Hash`

error[E0599]: no method named `hash` found for type `rustc::hir::UnOp` in the current scope
   --> clippy_lints/src/utils/hir_utils.rs:522:21
    |
522 |                 lop.hash(&mut self.s);
    |                     ^^^^ method not found in `rustc::hir::UnOp`

error: aborting due to 5 previous errors

