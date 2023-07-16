plain
    Checking cargo_metadata v0.14.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.60 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    |
    |
456 |             ArgumentNamed(n) => {
    |                           ^ expected 2 fields, found 1
   ::: /checkout/compiler/rustc_parse_format/src/lib.rs:98:19
    |
    |
98  |     ArgumentNamed(Symbol, InnerSpan),
    |                   ------  --------- tuple variant has 2 fields
    |
help: use `_` to explicitly ignore each field
    |
456 |             ArgumentNamed(n, _) => {

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:26
