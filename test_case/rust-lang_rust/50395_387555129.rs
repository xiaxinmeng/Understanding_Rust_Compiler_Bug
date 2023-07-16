plain
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]    --> tools/clippy/clippy_lints/src/utils/mod.rs:677:9
[00:56:32]     |
[00:56:32] 677 |         ty::TyRef(_, ref tm) => walk_ptrs_ty(tm.ty),
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]    --> tools/clippy/clippy_lints/src/consts.rs:429:13
[00:56:32]     |
[00:56:32]     |
[00:56:32] 429 |             ty::TyRef(_, tam) => match tam.ty.sty {
[00:56:32]     |             ^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]    --> tools/clippy/clippy_lints/src/utils/mod.rs:687:13
[00:56:32]     |
[00:56:32] 687 |             ty::TyRef(_, ref tm) => inner(tm.ty, depth + 1),
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]    --> tools/clippy/clippy_lints/src/utils/mod.rs:677:9
[00:56:32]     |
[00:56:32]     |
[00:56:32] 677 |         ty::TyRef(_, ref tm) => walk_ptrs_ty(tm.ty),
[00:56:32] 
[00:56:32] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:32]    --> tools/clippy/clippy_lints/src/utils/mod.rs:687:13
[00:56:32]     |
[00:56:32]     |
[00:56:32] 687 |             ty::TyRef(_, ref tm) => inner(tm.ty, depth + 1),
[00:56:32] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]    --> tools/clippy/clippy_lints/src/loops.rs:746:9
[00:56:33]     |
[00:56:33]     |
[00:56:33] 746 |         ty::TyRef(_, ref subty) => is_slice_like(cx, subty.ty),
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]     --> tools/clippy/clippy_lints/src/loops.rs:1368:17
[00:56:33]      |
[00:56:33]      |
[00:56:33] 1368 |                 ty::TyRef(_, ref tam) => match (&pat[0].node, &pat[1].node) {
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]     --> tools/clippy/clippy_lints/src/loops.rs:1708:28
[00:56:33]      |
---
[00:56:33] 
[00:56:33] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:33]    --> tools/clippy/clippy_lints/src/loops.rs:746:9
[00:56:33]     |
[00:56:33] 746 |         ty::TyRef(_, ref subty) => is_slice_like(cx, subty.ty),
[00:56:33] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     --> tools/clippy/clippy_lints/src/loops.rs:1368:17
[00:56:34]      |
[00:56:34]      |
[00:56:34] 1368 |                 ty::TyRef(_, ref tam) => match (&pat[0].node, &pat[1].node) {
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     --> tools/clippy/clippy_lints/src/loops.rs:1708:28
[00:56:34]      |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:752:21
[00:56:34]     |
[00:56:34] 752 |                     ty::TyRef(_, ty) if ty.ty.sty == ty::TyStr => for &(method, pos) in &PATTERN_METHODS {
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/map_clone.rs:54:44
[00:56:34]    |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:971:16
[00:56:34]     |
[00:56:34] 971 |         if let ty::TyRef(_, ty::TypeAndMut { ty: innermost, .. }) = inner.sty {
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:981:31
[00:56:34]     |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:752:21
[00:56:34]     |
[00:56:34] 752 |                     ty::TyRef(_, ty) if ty.ty.sty == ty::TyStr => for &(method, pos) in &PATTERN_METHODS {
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:970:12
[00:56:34]     |
[00:56:34]     |
[00:56:34] 970 |     if let ty::TyRef(_, ty::TypeAndMut { ty: inner, .. }) = arg_ty.sty {
[00:56:34]     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:971:16
[00:56:34]     |
[00:56:34] 971 |         if let ty::TyRef(_, ty::TypeAndMut { ty: innermost, .. }) = inner.sty {
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/methods.rs:981:31
[00:56:34]     |
---
[00:56:34] 75 | |                 ty::TypeAndMut {
[00:56:34] 76 | |                     mutbl: hir::MutMutable,
[00:56:34] 77 | |                     ..
[00:56:34] 78 | |                 },
[00:56:34] 79 | |             ) = self.cx.tables.expr_ty(e).sty
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]     --> tools/clippy/clippy_lints/src/methods.rs:1318:13
[00:56:34]      |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/needless_borrow.rs:80:20
[00:56:34]    |
[00:56:34] 80 |             if let ty::TyRef(_, ref tam) = cx.tables.pat_ty(pat).sty;
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/needless_borrow.rs:82:20
[00:56:34]    |
---
[00:56:34] 75 | |                 ty::TypeAndMut {
[00:56:34] 76 | |                     mutbl: hir::MutMutable,
[00:56:34] 77 | |                     ..
[00:56:34] 78 | |                 },
[00:56:34] 79 | |             ) = self.cx.tables.expr_ty(e).sty
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/mut_reference.rs:64:21
[00:56:34]    |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/needless_borrow.rs:80:20
[00:56:34]    |
[00:56:34] 80 |             if let ty::TyRef(_, ref tam) = cx.tables.pat_ty(pat).sty;
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/needless_borrow.rs:82:20
[00:56:34]    |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/transmute.rs:232:31
[00:56:34]     |
[00:56:34] 232 |                             (&ty::TyRef(_, rty), &ty::TyRawPtr(ptr_ty)) => span_lint_and_then(
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/transmute.rs:287:56
[00:56:34]     |
[00:56:34]     |
[00:56:34] 287 |                             (&ty::TyRawPtr(from_pty), &ty::TyRef(_, to_ref_ty)) => span_lint_and_then(
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/transmute.rs:334:31
[00:56:34]     |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/transmute.rs:232:31
[00:56:34]     |
[00:56:34] 232 |                             (&ty::TyRef(_, rty), &ty::TyRawPtr(ptr_ty)) => span_lint_and_then(
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/transmute.rs:287:56
[00:56:34]     |
[00:56:34]     |
[00:56:34] 287 |                             (&ty::TyRawPtr(from_pty), &ty::TyRef(_, to_ref_ty)) => span_lint_and_then(
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]    --> tools/clippy/clippy_lints/src/transmute.rs:334:31
[00:56:34]     |
---
[00:56:34] 
[00:56:34] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:34]   --> tools/clippy/clippy_lints/src/vec.rs:38:20
[00:56:34]    |
[00:56:34] 38 |             if let ty::TyRef(_, ref ty) = cx.tables.expr_ty_adjusted(expr).sty;
[00:56:34] 
[00:56:35] error: aborting due to 24 previous errors
[00:56:35] 
[00:56:35] For more information about this error, try `rustc --explain E0023`.
---
[00:56:35] warning: build failed, waiting for other jobs to finish...
[00:56:35] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[00:56:35]   --> tools/clippy/clippy_lints/src/vec.rs:38:20
[00:56:35]    |
[00:56:35] 38 |             if let ty::TyRef(_, ref ty) = cx.tables.expr_ty_adjusted(expr).sty;
[00:56:35] 
[00:56:35] error: aborting due to 24 previous errors
[00:56:35] 
[00:56:35] For more information about this error, try `rustc --explain E0023`.
---
[01:17:27] [RUSTC-TIMING] byteorder test:false 0.827
[01:17:30] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[01:17:30]    --> tools/miri/miri/validation.rs:404:13
[01:17:30]     |
[01:17:30] 404 |             ty::TyRef(_, ty::TypeAndMut { ty: pointee, .. }) |
[01:17:30] 
[01:17:30] error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
[01:17:30]    --> tools/miri/miri/validation.rs:661:17
[01:17:30]     |
[01:17:30]     |
[01:17:30] 661 | /                 TyRef(region,
[01:17:30] 662 | |                     ty::TypeAndMut {
[01:17:30] 663 | |                         ty: pointee_ty,
[01:17:30] 664 | |                         mutbl,
[01:17:30] 665 | |                     }) => {
[01:17:30] 
[01:17:30] error: aborting due to 2 previous errors
[01:17:30] 
[01:17:30] For more information about this error, try `rustc --explain E0023`.
---
[01:17:30] Cloning into 'rust-toolstate'...
[01:17:30] [master 7eff5f2] (linux CI update)
[01:17:30]  1 file changed, 1 insertion(+)
[01:17:34] To https://github.com/rust-lang-nursery/rust-toolstate.git
[01:17:34]    eec401a..7eff5f2  master -> master
[01:17:34] Error: The state of "clippy-driver" has regressed from "test-pass" to "build-fail"
[01:17:34] Error: The state of "miri" has regressed from "test-pass" to "build-fail"

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0534c600
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
