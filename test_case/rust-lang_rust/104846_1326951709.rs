plain
   Compiling compiletest_rs v0.9.0
    Checking unicode-normalization v0.1.22
    Checking rand_chacha v0.3.0
    Checking strsim v0.10.0
error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
  --> src/tools/clippy/clippy_utils/src/eager_or_lazy.rs:76:32
   |
76 |                 PredicateKind::Trait(pred) => cx.tcx.trait_def(pred.trait_ref.def_id).is_marker,
   |                                ^^^^^ variant or associated item not found in `PredicateKind<'_>`
    Checking tester v0.9.0
    Checking filetime v0.2.14
    Checking same-file v1.0.6
    Checking same-file v1.0.6
error[E0599]: no variant or associated item named `RegionOutlives` found for enum `PredicateKind` in the current scope
  --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:28:36
   |
28 |                 ty::PredicateKind::RegionOutlives(_)
   |                                    ^^^^^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `TypeOutlives` found for enum `PredicateKind` in the current scope
  --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:29:38
   |
29 |                 | ty::PredicateKind::TypeOutlives(_)
   |                                      ^^^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
  --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:31:38
31 |                 | ty::PredicateKind::Projection(_)
   |                                      ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
  --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:34:38
   |
34 |                 | ty::PredicateKind::Trait(..)
   |                                      ^^^^^ variant or associated item not found in `PredicateKind<'_>`
    Checking diff v0.1.13
    Checking bstr v0.2.17
    Checking rand v0.8.5
    Checking bytes v1.0.1
    Checking bytes v1.0.1
    Checking parking_lot v0.12.1
error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
   |
84 |                         ty::PredicateKind::Trait(trait_predicate) => {
   |                                            ^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
   |
   |
97 |                         ty::PredicateKind::Projection(projection_predicate) => {
   |                                            ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`
    Checking walkdir v2.3.2
    Checking walkdir v2.3.2
error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    |
    |
242 |                 if let ty::PredicateKind::Trait(trait_predicate) = predicate.kind().skip_binder() {
    |                                           ^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    |
661 |             PredicateKind::Trait(p)
    |                            ^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
    |
674 |             PredicateKind::Projection(p)
    |                            ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    |
702 |             PredicateKind::Trait(p)
    |                            ^^^^^ variant or associated item not found in `PredicateKind<'_>`


error[E0599]: no variant or associated item named `Projection` found for enum `PredicateKind` in the current scope
    |
    |
715 |             PredicateKind::Projection(p) if Some(p.projection_ty.item_def_id) == lang_items.fn_once_output() => {
    |                            ^^^^^^^^^^ variant or associated item not found in `PredicateKind<'_>`

error[E0599]: no variant or associated item named `Trait` found for enum `PredicateKind` in the current scope
    |
    |
890 |             if let PredicateKind::Trait(p) = p.kind().skip_binder()
    |                                   ^^^^^ variant or associated item not found in `PredicateKind<'_>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_utils` due to 13 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:04:44
