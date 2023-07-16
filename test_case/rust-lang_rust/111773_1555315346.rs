plain
    Checking clippy_lints v0.1.71 (/checkout/src/tools/clippy/clippy_lints)
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:293:30
    |
293 |             if matches!(res, Res::PrimTy(PrimTy::Uint(_))) || matches!(res, Res::PrimTy(PrimTy::Int(_))) {
    |                         ---  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    |                         this expression has type `&PathSegmentRes`
    |
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`
               found enum `rustc_hir::def::Res<_>`

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:293:77
    |
293 |             if matches!(res, Res::PrimTy(PrimTy::Uint(_))) || matches!(res, Res::PrimTy(PrimTy::Int(_))) {
    |                                                                        ---  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    |                                                                        this expression has type `&PathSegmentRes`
    |
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`
               found enum `rustc_hir::def::Res<_>`

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:306:26
    |
306 |         if matches!(res, Res::PrimTy(PrimTy::Uint(_))) || matches!(res, Res::PrimTy(PrimTy::Int(_))) {
    |                     ---  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    |                     this expression has type `PathSegmentRes`
    |
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`
               found enum `rustc_hir::def::Res<_>`
help: try wrapping the pattern in `rustc_hir::PathSegmentRes::Res`
    |
306 |         if matches!(res, rustc_hir::PathSegmentRes::Res(Res::PrimTy(PrimTy::Uint(_)))) || matches!(res, Res::PrimTy(PrimTy::Int(_))) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:306:73
    |
    |
306 |         if matches!(res, Res::PrimTy(PrimTy::Uint(_))) || matches!(res, Res::PrimTy(PrimTy::Int(_))) {
    |                                                                    ---  ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    |                                                                    this expression has type `PathSegmentRes`
    |
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`
               found enum `rustc_hir::def::Res<_>`
help: try wrapping the pattern in `rustc_hir::PathSegmentRes::Res`
    |
306 |         if matches!(res, Res::PrimTy(PrimTy::Uint(_))) || matches!(res, rustc_hir::PathSegmentRes::Res(Res::PrimTy(PrimTy::Int(_)))) {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:310:16
    |
    |
310 |         if let Res::Def(_, def_id) = res {
    |                ^^^^^^^^^^^^^^^^^^^   --- this expression has type `PathSegmentRes`
    |                |
    |                expected `PathSegmentRes`, found `Res<_>`
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`
help: try wrapping the pattern in `rustc_hir::PathSegmentRes::Res`
    |
    |
310 |         if let rustc_hir::PathSegmentRes::Res(Res::Def(_, def_id)) = res {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:337:21
    |
    |
336 |             return match (self, segment.res) {
    |                          ------------------- this expression has type `(LenOutput, PathSegmentRes)`
337 |                 (_, Res::PrimTy(PrimTy::Bool)) => true,
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:338:35
    |
336 |             return match (self, segment.res) {
    |                          ------------------- this expression has type `(LenOutput, PathSegmentRes)`
337 |                 (_, Res::PrimTy(PrimTy::Bool)) => true,
338 |                 (Self::Option(_), Res::Def(_, def_id)) if cx.tcx.is_diagnostic_item(sym::Option, def_id) => true,
    |                                   ^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/len_zero.rs:339:35
    |
336 |             return match (self, segment.res) {
    |                          ------------------- this expression has type `(LenOutput, PathSegmentRes)`
...
339 |                 (Self::Result(_), Res::Def(_, def_id)) if cx.tcx.is_diagnostic_item(sym::Result, def_id) => true,
    |                                   ^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`

error[E0308]: mismatched types
error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/operators/op_ref.rs:188:16
    |
188 |         if let Res::Def(_, trait_id) = seg.res;
    |                ^^^^^^^^^^^^^^^^^^^^^   ------- this expression has type `PathSegmentRes`
    |                |
    |                expected `PathSegmentRes`, found `Res<_>`
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`
help: try wrapping the pattern in `rustc_hir::PathSegmentRes::Res`
    |
    |
188 |         if let rustc_hir::PathSegmentRes::Res(Res::Def(_, trait_id)) = seg.res;


error[E0599]: no method named `opt_def_id` found for enum `PathSegmentRes` in the current scope
    |
46  |             if let Some(def_id) = last.res.opt_def_id();
    |                                            ^^^^^^^^^^ method not found in `PathSegmentRes`
    |
    |
note: the method `opt_def_id` exists on the type `rustc_hir::def::Res`
   --> /checkout/compiler/rustc_hir/src/def.rs:624:5
    |
624 |     pub fn opt_def_id(&self) -> Option<DefId> {

error[E0308]: mismatched types
   --> src/tools/clippy/clippy_lints/src/trait_bounds.rs:131:26
    |
    |
131 |                     res: Res::SelfTyParam { trait_: def_id }, ..
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `PathSegmentRes`, found `Res<_>`
132 |                 }) = segments.first();
    |                      ---------------- this expression has type `std::option::Option<&rustc_hir::PathSegment<'_>>`
    = note: expected enum `PathSegmentRes`
               found enum `rustc_hir::def::Res<_>`

Some errors have detailed explanations: E0308, E0599.
