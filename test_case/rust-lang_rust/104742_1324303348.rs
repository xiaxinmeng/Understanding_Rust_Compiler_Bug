plain
    Checking unicode-width v0.1.10
error[E0433]: failed to resolve: use of undeclared type `Ident`
   --> src/tools/clippy/clippy_utils/src/ty.rs:976:41
    |
976 |             .find_by_name_and_kind(tcx, Ident::with_dummy_span(assoc_ty), AssocKind::Type, container_id)
    |
help: consider importing one of these items
    |
5   | use crate::Ident;
