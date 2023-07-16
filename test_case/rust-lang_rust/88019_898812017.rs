plain
    Checking clippy_utils v0.1.56 (/checkout/src/tools/clippy/clippy_utils)
    Checking url v2.2.2
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.56 (/checkout/src/tools/clippy/clippy_lints)
error[E0004]: non-exhaustive patterns: `Macro { .. }` not covered
     |
388  |     match item.kind {
388  |     match item.kind {
     |           ^^^^^^^^^ pattern `Macro { .. }` not covered
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2784:5
     |
     |
2784 |     Macro { is_exported: bool, macro_def: MacroDef<'hir> },
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `rustc_hir::ItemKind`


error[E0004]: non-exhaustive patterns: `Macro { .. }` not covered
     |
113  |         match it.kind {
113  |         match it.kind {
     |               ^^^^^^^ pattern `Macro { .. }` not covered
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2784:5
     |
     |
2784 |     Macro { is_exported: bool, macro_def: MacroDef<'hir> },
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `rustc_hir::ItemKind`


error[E0004]: non-exhaustive patterns: `Macro { .. }` not covered
     |
93   |         match it.kind {
93   |         match it.kind {
     |               ^^^^^^^ pattern `Macro { .. }` not covered
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2784:5
     |
     |
2784 |     Macro { is_exported: bool, macro_def: MacroDef<'hir> },
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `rustc_hir::ItemKind`

