plain
    Checking tester v0.9.0
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/check_proc_macro.rs:121:54
     |
121  |         ExprKind::Call(e, []) | ExprKind::MethodCall(_, [e], _) => (expr_search_pat(tcx, e).0, Pat::Str("(")),
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
121  |         ExprKind::Call(e, []) | ExprKind::MethodCall(_, [e], _, _) => (expr_search_pat(tcx, e).0, Pat::Str("(")),

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/check_proc_macro.rs:123:32
     |
     |
123  |         | ExprKind::MethodCall(_, [first, .., last], _)
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
123  |         | ExprKind::MethodCall(_, [first, .., last], _, _)

    Checking pulldown-cmark v0.9.2
    Checking aho-corasick v0.7.18
    Checking bstr v0.2.17
---
     |                                      ^^^^  ^^^^  ^ expected 4 fields, found 3
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
144  |                 ExprKind::MethodCall(name, args, _, _) => {
144  |                 ExprKind::MethodCall(name, args, _, _) => {
     |                                                   +++

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:285:36
     |
285  |             (&ExprKind::MethodCall(l_path, l_args, _), &ExprKind::MethodCall(r_path, r_args, _)) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
285  |             (&ExprKind::MethodCall(l_path, l_args, _, _), &ExprKind::MethodCall(r_path, r_args, _)) => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:285:78
     |
     |
285  |             (&ExprKind::MethodCall(l_path, l_args, _), &ExprKind::MethodCall(r_path, r_args, _)) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
285  |             (&ExprKind::MethodCall(l_path, l_args, _), &ExprKind::MethodCall(r_path, r_args, _, _)) => {

    Checking idna v0.2.0
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:746:34
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:746:34
     |
746  |             ExprKind::MethodCall(path, args, ref _fn_span) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
746  |             ExprKind::MethodCall(path, args, ref _fn_span, _) => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/ptr.rs:39:37
     |
     |
39   |         if let ExprKind::MethodCall(seg, [recv], _) = expr.kind {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
39   |         if let ExprKind::MethodCall(seg, [recv], _, _) = expr.kind {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/sugg.rs:871:34
     |
     |
871  |             ExprKind::MethodCall(_, call_args, _) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
871  |             ExprKind::MethodCall(_, call_args, _, _) => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/sugg.rs:936:46
     |
     |
936  |                         ExprKind::MethodCall(_, [self_expr, ..], _) if self_expr.hir_id == cmt.hir_id => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
936  |                         ExprKind::MethodCall(_, [self_expr, ..], _, _) if self_expr.hir_id == cmt.hir_id => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/sugg.rs:943:84
     |
     |
943  |                         ExprKind::Call(_, [call_args @ ..]) | ExprKind::MethodCall(_, [_, call_args @ ..], _) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
943  |                         ExprKind::Call(_, [call_args @ ..]) | ExprKind::MethodCall(_, [_, call_args @ ..], _, _) => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/visitors.rs:623:34
     |
     |
623  |             ExprKind::MethodCall(_, args, _) | ExprKind::Tup(args) | ExprKind::Array(args) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
623  |             ExprKind::MethodCall(_, args, _, _) | ExprKind::Tup(args) | ExprKind::Array(args) => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/lib.rs:1046:37
     |
     |
1046 |         if let ExprKind::MethodCall(path, args, _) = &current.kind {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
1046 |         if let ExprKind::MethodCall(path, args, _, _) = &current.kind {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/lib.rs:1073:37
     |
     |
1073 |         if let ExprKind::MethodCall(path, args, _) = current.kind {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
1073 |         if let ExprKind::MethodCall(path, args, _, _) = current.kind {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_utils/src/lib.rs:1242:50
     |
     |
1242 | ...                   ExprKind::MethodCall(_, args, _) => {
     |
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1896:16
     |
     |
1896 |     MethodCall(&'hir PathSegment<'hir>, &'hir Expr<'hir>, &'hir [Expr<'hir>], Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
1242 |                             ExprKind::MethodCall(_, args, _, _) => {

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_utils` due to 14 previous errors
warning: build failed, waiting for other jobs to finish...
