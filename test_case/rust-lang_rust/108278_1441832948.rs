plain
  |
5 | use rustc_errors::Handler;
  |     --------------------- previous import of the type `Handler` here
...
8 |     Handler, IntoDiagnostic, MultiSpan, SubdiagnosticMessage,
  |     |
  |     |
  |     `Handler` reimported here
  |     help: remove unnecessary import
  |
  = note: `Handler` must be defined only once in the type namespace of this module
error[E0433]: failed to resolve: could not find `fluent` in `rustc_errors`
    --> compiler/rustc_mir_build/src/errors.rs:1023:59
     |
     |
1023 |                     AddArmKind::Wildcard => rustc_errors::fluent::mir_build_suggest_wildcard_arm,
     |                                                           ^^^^^^ could not find `fluent` in `rustc_errors`
error[E0433]: failed to resolve: could not find `fluent` in `rustc_errors`
    --> compiler/rustc_mir_build/src/errors.rs:1026:39
     |
1026 |                         rustc_errors::fluent::mir_build_suggest_single_arm
1026 |                         rustc_errors::fluent::mir_build_suggest_single_arm
     |                                       ^^^^^^ could not find `fluent` in `rustc_errors`

error[E0433]: failed to resolve: could not find `fluent` in `rustc_errors`
    --> compiler/rustc_mir_build/src/errors.rs:1028:59
     |
1028 |                     AddArmKind::Multiple => rustc_errors::fluent::mir_build_suggest_multiple_arms,
     |                                                           ^^^^^^ could not find `fluent` in `rustc_errors`
error[E0433]: failed to resolve: could not find `fluent` in `rustc_errors`
    --> compiler/rustc_mir_build/src/errors.rs:1046:59
     |
     |
1046 |                     AddArmKind::Wildcard => rustc_errors::fluent::mir_build_suggest_wildcard_arm,
     |                                                           ^^^^^^ could not find `fluent` in `rustc_errors`
error[E0433]: failed to resolve: could not find `fluent` in `rustc_errors`
    --> compiler/rustc_mir_build/src/errors.rs:1049:39
     |
1049 |                         rustc_errors::fluent::mir_build_suggest_single_arm
1049 |                         rustc_errors::fluent::mir_build_suggest_single_arm
     |                                       ^^^^^^ could not find `fluent` in `rustc_errors`

error[E0433]: failed to resolve: could not find `fluent` in `rustc_errors`
    --> compiler/rustc_mir_build/src/errors.rs:1051:59
     |
1051 |                     AddArmKind::Multiple => rustc_errors::fluent::mir_build_suggest_multiple_arms,
     |                                                           ^^^^^^ could not find `fluent` in `rustc_errors`
error[E0422]: cannot find struct, variant or union type `Variant` in this scope
   --> compiler/rustc_mir_build/src/errors.rs:861:13
    |
    |
861 |         for Variant { span } in self.variants {
    |
help: consider importing one of these items
    |
1   | use crate::thir::pattern::deconstruct_pat::Constructor::Variant;
---

error: unused import: `Handler`
 --> compiler/rustc_mir_build/src/errors.rs:8:5
  |
8 |     Handler, IntoDiagnostic, MultiSpan, SubdiagnosticMessage,
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
Some errors have detailed explanations: E0252, E0422, E0433.
For more information about an error, try `rustc --explain E0252`.
error: could not compile `rustc_mir_build` due to 9 previous errors
warning: build failed, waiting for other jobs to finish...
