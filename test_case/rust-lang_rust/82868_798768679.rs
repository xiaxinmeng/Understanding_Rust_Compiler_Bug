plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/librustdoc/clean/mod.rs:1480:13
     |
1480 |             TyKind::TraitObject(ref bounds, ref lifetime) => {
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 fields, found 2
    ::: /checkout/compiler/rustc_hir/src/hir.rs:2328:5
     |
     |
2328 |     TraitObject(&'hir [PolyTraitRef<'hir>], Lifetime, TraitObjectSyntax),
     |     -------------------------------------------------------------------- tuple variant defined here
     |
help: use `_` to explicitly ignore each field
     |
1480 |             TyKind::TraitObject(ref bounds, ref lifetime, _) => {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0023`.
