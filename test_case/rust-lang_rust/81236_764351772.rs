plain
Successfully built 888cffd31b1d
Successfully tagged rust-ci:latest
Built container sha256:888cffd31b1da503e58b2cbb988fb0cf330d447ca8dbca51eeeb0c891900677f
Uploading finished image to https://ci-caches.rust-lang.org/docker/aa85b52f727783ce661c5275c3edac7e8b4fbba025fc0a8e6392d6566595f3d157bb7da4c70e2eda4e5e6e7455d8337e7dfb6de6b288fcd6e68cf37f838a32b6
upload failed: - to s3://rust-lang-ci-sccache2/docker/aa85b52f727783ce661c5275c3edac7e8b4fbba025fc0a8e6392d6566595f3d157bb7da4c70e2eda4e5e6e7455d8337e7dfb6de6b288fcd6e68cf37f838a32b6 Unable to locate credentials
[CI_JOB_NAME=mingw-check]
---
    Checking toml v0.5.7
    Checking url v2.1.1
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.51 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
     |
     |
320  |             ExprKind::Loop(ref body, _, desugaring) => {
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 3
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1622:5
     |
1622 |     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
     |     -------------------------------------------------------- tuple variant defined here

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/utils/higher.rs:145:16
     |
145  |         if let hir::ExprKind::Loop(ref block, _, _) = arms[0].body.kind;
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 3
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1622:5
     |
1622 |     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
     |     -------------------------------------------------------- tuple variant defined here

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/utils/higher.rs:161:16
     |
161  |         if let hir::ExprKind::Loop(block, _, hir::LoopSource::While) = &expr.kind;
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 3
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1622:5
     |
1622 |     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
     |     -------------------------------------------------------- tuple variant defined here

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:126:15
     |
126  |             (&ExprKind::Loop(ref lb, ref ll, ref lls), &ExprKind::Loop(ref rb, ref rl, ref rls)) => {
     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 3
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1622:5
     |
1622 |     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
     |     -------------------------------------------------------- tuple variant defined here

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:126:57
     |
126  |             (&ExprKind::Loop(ref lb, ref ll, ref lls), &ExprKind::Loop(ref rb, ref rl, ref rls)) => {
     |                                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 3
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1622:5
     |
1622 |     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
     |     -------------------------------------------------------- tuple variant defined here

error[E0609]: no field `ident` on type `&_`
   --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:127:78
    |
127 |                 lls == rls && self.eq_block(lb, rb) && both(ll, rl, |l, r| l.ident.name == r.ident.name)


error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:563:13
     |
563  |             ExprKind::Loop(ref b, ref i, _) => {
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 4 fields, found 3
     | 
    ::: /checkout/compiler/rustc_hir/src/hir.rs:1622:5
     |
1622 |     Loop(&'hir Block<'hir>, Option<Label>, LoopSource, Span),
     |     -------------------------------------------------------- tuple variant defined here

error[E0609]: no field `ident` on type `&_`
   --> src/tools/clippy/clippy_lints/src/utils/hir_utils.rs:566:38
    |
566 |                     self.hash_name(i.ident.name);


error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/needless_continue.rs:224:7
     |
224  |     | ast::ExprKind::Loop(loop_block, label, _) = &expr.kind
     |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 3
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1308:5
     |
     |
1308 |     Loop(P<Block>, Option<Label>),
     |     ----------------------------- tuple variant defined here
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0023, E0609.
For more information about an error, try `rustc --explain E0023`.
