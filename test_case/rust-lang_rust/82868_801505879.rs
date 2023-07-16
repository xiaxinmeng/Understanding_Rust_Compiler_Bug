plain
    Checking semver v0.11.0
    Checking url v2.1.1
    Checking clippy_utils v0.1.52 (/checkout/src/tools/clippy/clippy_utils)
    Checking cargo_metadata v0.12.0
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/clippy/clippy_utils/src/hir_utils.rs:895:13
     |
895  |             TyKind::TraitObject(_, lifetime) => {
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2330:5
     |
     |
2330 |     TraitObject(&'hir [PolyTraitRef<'hir>], Lifetime, TraitObjectSyntax),
     |     -------------------------------------------------------------------- tuple variant defined here
     |
help: use `_` to explicitly ignore each field
     |
895  |             TyKind::TraitObject(_, lifetime, _) => {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
