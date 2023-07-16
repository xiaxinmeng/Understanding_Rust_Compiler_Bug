plain
     |                   ^^^^^^^^^^^^^
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2016:5
     |
2016 |     Err(rustc_span::ErrorGuaranteed),
     |     --- `ExprKind::Err` defined here
help: use the tuple variant pattern syntax instead
     |
     |
196  |                 | ExprKind::Err(/* fields */) => {
help: consider importing one of these items instead
     |
12   | use crate::Constant::Err;
     |
---
     |             ^^^^^^^^^^^^^
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2016:5
     |
2016 |     Err(rustc_span::ErrorGuaranteed),
     |     --- `ExprKind::Err` defined here
help: use the tuple variant pattern syntax instead
     |
     |
717  |             ExprKind::Err(/* fields */) => {},
help: consider importing one of these items instead
     |
1    | use crate::Constant::Err;
     |
---

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `TyKind::Err`
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:989:13
     |
989  |             TyKind::Err | TyKind::Infer | TyKind::Never => {},
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2679:5
     |
     |
2679 |     Err(rustc_span::ErrorGuaranteed),
     |     --- `TyKind::Err` defined here
help: use the tuple variant pattern syntax instead
     |
     |
989  |             TyKind::Err(/* fields */) | TyKind::Infer | TyKind::Never => {},
help: consider importing one of these items instead
     |
1    | use crate::Constant::Err;
     |
---
     |
       and 12 other candidates
help: if you import `Err`, refer to it directly
     |
989  -             TyKind::Err | TyKind::Infer | TyKind::Never => {},
989  +             Err | TyKind::Infer | TyKind::Never => {},

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `hir::ExprKind::Err`
    --> src/tools/clippy/clippy_utils/src/sugg.rs:160:15
     |
     |
160  |             | hir::ExprKind::Err => Sugg::NonParen(get_snippet(expr.span)),
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2016:5
     |
     |
2016 |     Err(rustc_span::ErrorGuaranteed),
     |     --- `hir::ExprKind::Err` defined here
help: use the tuple variant pattern syntax instead
     |
     |
160  |             | hir::ExprKind::Err(/* fields */) => Sugg::NonParen(get_snippet(expr.span)),
help: consider importing one of these items instead
     |
4    | use crate::Constant::Err;
     |
---
     |
       and 13 other candidates
help: if you import `Err`, refer to it directly
     |
160  -             | hir::ExprKind::Err => Sugg::NonParen(get_snippet(expr.span)),
160  +             | Err => Sugg::NonParen(get_snippet(expr.span)),

error[E0532]: expected unit struct, unit variant or constant, found tuple variant `ExprKind::Err`
    --> src/tools/clippy/clippy_utils/src/visitors.rs:668:15
     |
     |
668  |             | ExprKind::Err => (),
     |               ^^^^^^^^^^^^^
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2016:5
     |
2016 |     Err(rustc_span::ErrorGuaranteed),
     |     --- `ExprKind::Err` defined here
help: use the tuple variant pattern syntax instead
     |
     |
668  |             | ExprKind::Err(/* fields */) => (),
help: consider importing one of these items instead
     |
1    | use crate::Constant::Err;
     |
