plain
Building stage0 tool jsondoclint (x86_64-unknown-linux-gnu)
   Compiling rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
   Compiling serde_json v1.0.85
   Compiling jsondoclint v0.1.0 (/checkout/src/tools/jsondoclint)
error[E0004]: non-exhaustive patterns: `&GenericParamDefKind::Effect` not covered
    |
    |
296 |         match &gpd.kind {
    |               ^^^^^^^^^ pattern `&GenericParamDefKind::Effect` not covered
note: `GenericParamDefKind` defined here
   --> /checkout/src/rustdoc-json-types/lib.rs:479:5
    |
442 | pub enum GenericParamDefKind {
442 | pub enum GenericParamDefKind {
    | ----------------------------
...
479 |     Effect,
    |     ^^^^^^ not covered
    = note: the matched value is of type `&GenericParamDefKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
306 ~             }
306 ~             }
307 +             &GenericParamDefKind::Effect => todo!()

For more information about this error, try `rustc --explain E0004`.
error: could not compile `jsondoclint` due to previous error
Build completed unsuccessfully in 0:32:24
