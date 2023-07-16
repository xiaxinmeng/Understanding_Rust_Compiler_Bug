plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0004]: non-exhaustive patterns: `Variant(_, _)` not covered
    --> src/librustdoc/clean/mod.rs:1396:15
     |
1396 |         match *ty.kind() {
     |               ^^^^^^^^^^ pattern `Variant(_, _)` not covered
    ::: /checkout/compiler/rustc_middle/src/ty/sty.rs:199:5
     |
     |
199  |     Variant(Ty<'tcx>, VariantIdx),
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `rustc_middle::ty::TyKind`


error[E0004]: non-exhaustive patterns: `Variant(_, _)` not covered
    |
    |
539 |         Some(match *self.cx.tcx.type_of(ty_id).kind() {
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `Variant(_, _)` not covered
   ::: /checkout/compiler/rustc_middle/src/ty/sty.rs:199:5
    |
    |
199 |     Variant(Ty<'tcx>, VariantIdx),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_middle::ty::TyKind`

