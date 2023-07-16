plain
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0425]: cannot find value `err` in this scope
   --> compiler/rustc_session/src/parse.rs:131:9
    |
131 |         err.note(&format!(
    |         ^^^ help: a tuple variant with a similar name exists: `Err`
   ::: /checkout/library/core/src/result.rs:512:5
    |
    |
512 |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
    |     ---------------------------------------------------- similarly named tuple variant `Err` defined here
error[E0425]: cannot find value `err` in this scope
   --> compiler/rustc_session/src/parse.rs:139:9
    |
    |
139 |         err.help(&format!("add `#![feature({})]` to the crate attributes to enable", feature));
    |         ^^^ help: a tuple variant with a similar name exists: `Err`
   ::: /checkout/library/core/src/result.rs:512:5
    |
    |
512 |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
    |     ---------------------------------------------------- similarly named tuple variant `Err` defined here

error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
    |
112 |     builder: &mut DiagnosticBuilder<'a>,
    |                   ^^^^^^^^^^^^^^^^^ expected 1 generic argument
    |
    |
note: struct defined here, with 1 generic parameter: `G`
    |
    |
19  | pub struct DiagnosticBuilder<'a, G: EmissionGuarantee> {
help: add missing generic argument
    |
    |
112 |     builder: &mut DiagnosticBuilder<'a, G>,


error[E0107]: this struct takes 1 generic argument but 0 generic arguments were supplied
    |
125 |     builder: &mut DiagnosticBuilder<'a>,
    |                   ^^^^^^^^^^^^^^^^^ expected 1 generic argument
    |
    |
note: struct defined here, with 1 generic parameter: `G`
    |
    |
19  | pub struct DiagnosticBuilder<'a, G: EmissionGuarantee> {
help: add missing generic argument
    |
    |
125 |     builder: &mut DiagnosticBuilder<'a, G>,

Some errors have detailed explanations: E0107, E0425.
For more information about an error, try `rustc --explain E0107`.
error: could not compile `rustc_session` due to 4 previous errors
