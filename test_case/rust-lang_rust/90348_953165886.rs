plain
    Checking rustc_ast v0.0.0 (/checkout/compiler/rustc_ast)
    Checking rustc_target v0.0.0 (/checkout/compiler/rustc_target)
    Checking rustc_feature v0.0.0 (/checkout/compiler/rustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/compiler/rustc_parse_format)
error[E0425]: cannot find value `asm_experimental_arch` in module `sym`
   --> compiler/rustc_feature/src/active.rs:698:14
    |
698 |     (active, asm_experimental_arch, "1.57.0", Some(72016), None),
    |              ^^^^^^^^^^^^^^^^^^^^^ help: a constant with a similar name exists: `asm_experiemental_arch`
   ::: /checkout/compiler/rustc_span/src/symbol.rs:23:1
    |
23  | symbols! {
23  | symbols! {
    | -------- similarly named constant `asm_experiemental_arch` defined here

error[E0531]: cannot find unit struct, unit variant or constant `asm_experimental_arch` in module `sym`
   --> compiler/rustc_feature/src/active.rs:698:14
    |
698 |     (active, asm_experimental_arch, "1.57.0", Some(72016), None),
    |              ^^^^^^^^^^^^^^^^^^^^^ help: a constant with a similar name exists: `asm_experiemental_arch`
   ::: /checkout/compiler/rustc_span/src/symbol.rs:23:1
    |
23  | symbols! {
23  | symbols! {
    | -------- similarly named constant `asm_experiemental_arch` defined here
Some errors have detailed explanations: E0425, E0531.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_feature` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
