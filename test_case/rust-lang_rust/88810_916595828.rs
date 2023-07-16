plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0425]: cannot find value `hir_id` in this scope
    --> src/librustdoc/clean/mod.rs:1295:63
     |
1295 |                 trait_: Box::new(resolve_type(cx, trait_path, hir_id)),
     |                                                               ^^^^^^ help: a local variable with a similar name exists: `hir_ty`

error[E0425]: cannot find value `hir_id` in this scope
    --> src/librustdoc/clean/mod.rs:1311:63
     |
1311 |                 trait_: Box::new(resolve_type(cx, trait_path, hir_id)),
     |                                                               ^^^^^^ help: a local variable with a similar name exists: `hir_ty`
error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> src/librustdoc/clean/mod.rs:1295:34
     |
     |
1295 |                 trait_: Box::new(resolve_type(cx, trait_path, hir_id)),
     |                                  |
     |                                  expected 2 arguments
     |
note: function defined here
note: function defined here
    --> src/librustdoc/clean/utils.rs:410:10
     |
410  | crate fn resolve_type(cx: &mut DocContext<'_>, path: Path) -> Type {

error[E0061]: this function takes 2 arguments but 3 arguments were supplied
    --> src/librustdoc/clean/mod.rs:1311:34
     |
     |
1311 |                 trait_: Box::new(resolve_type(cx, trait_path, hir_id)),
     |                                  |
     |                                  expected 2 arguments
     |
note: function defined here
note: function defined here
    --> src/librustdoc/clean/utils.rs:410:10
     |
410  | crate fn resolve_type(cx: &mut DocContext<'_>, path: Path) -> Type {

Some errors have detailed explanations: E0061, E0425.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustdoc` due to 4 previous errors
