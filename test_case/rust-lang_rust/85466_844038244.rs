plain
    Checking semver v0.11.0
    Checking toml v0.5.7
    Checking url v2.2.2
    Checking clippy_utils v0.1.54 (/checkout/src/tools/clippy/clippy_utils)
error[E0599]: no associated item named `AutoTrait` found for struct `Binder<'_, rustc_middle::ty::WhereClause<'_>>` in the current scope
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:108:51
    |
108 |                         ty::ExistentialPredicate::AutoTrait(_) | ty::ExistentialPredicate::Projection(_) => {
    |                                                   ^^^^^^^^^ associated item not found in `Binder<'_, rustc_middle::ty::WhereClause<'_>>`

error[E0599]: no associated item named `Projection` found for struct `Binder<'_, rustc_middle::ty::WhereClause<'_>>` in the current scope
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:108:92
    |
108 |                         ty::ExistentialPredicate::AutoTrait(_) | ty::ExistentialPredicate::Projection(_) => {
    |                                                                                            ^^^^^^^^^^ associated item not found in `Binder<'_, rustc_middle::ty::WhereClause<'_>>`

error[E0599]: no associated item named `Trait` found for struct `Binder<'_, rustc_middle::ty::WhereClause<'_>>` in the current scope
   --> src/tools/clippy/clippy_utils/src/qualify_min_const_fn.rs:116:51
    |
116 |                         ty::ExistentialPredicate::Trait(trait_ref) => {
    |                                                   ^^^^^ associated item not found in `Binder<'_, rustc_middle::ty::WhereClause<'_>>`

error[E0599]: no associated item named `Trait` found for struct `Binder<'_, rustc_middle::ty::WhereClause<'_>>` in the current scope
   --> src/tools/clippy/clippy_utils/src/ty.rs:169:50
    |
169 |                 if let ty::ExistentialPredicate::Trait(ref trait_ref) = predicate.skip_binder() {
    |                                                  ^^^^^ associated item not found in `Binder<'_, rustc_middle::ty::WhereClause<'_>>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_utils`
