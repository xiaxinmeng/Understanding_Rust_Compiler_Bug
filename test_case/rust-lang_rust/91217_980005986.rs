plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.58 (/checkout/src/tools/clippy/clippy_lints)
error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
   --> src/tools/clippy/clippy_lints/src/cognitive_complexity.rs:84:33
    |
84  |             let fn_span = match kind {
    |                                 ^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`


error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
   --> src/tools/clippy/clippy_lints/src/functions/not_unsafe_ptr_arg_deref.rs:19:26
19  |     let unsafety = match kind {
19  |     let unsafety = match kind {
    |                          ^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`


error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
   --> src/tools/clippy/clippy_lints/src/missing_const_for_fn.rs:111:15
111 |         match kind {
111 |         match kind {
    |               ^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`


error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
   --> src/tools/clippy/clippy_lints/src/needless_pass_by_value.rs:86:15
86  |         match kind {
86  |         match kind {
    |               ^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`


error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
   --> src/tools/clippy/clippy_lints/src/pass_by_ref_or_value.rs:251:15
251 |         match kind {
251 |         match kind {
    |               ^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`


error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
    |
134 |         match kind {
134 |         match kind {
    |               ^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`


error[E0004]: non-exhaustive patterns: `ForeignFn(_)` not covered
    |
82  |         match fn_kind {
82  |         match fn_kind {
    |               ^^^^^^^ pattern `ForeignFn(_)` not covered
   ::: /checkout/compiler/rustc_hir/src/intravisit.rs:112:5
    |
    |
112 |     ForeignFn(Ident),
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `rustc_hir::intravisit::FnKind`

