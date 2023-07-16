plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0412]: cannot find type `UselessSendConstraint` in this scope
    |
    |
245 |             UselessSendConstraint: UselessSendConstraint,


error[E0425]: cannot find value `UselessSendConstraint` in this scope
    |
    |
245 |             UselessSendConstraint: UselessSendConstraint,
    |                                    ^^^^^^^^^^^^^^^^^^^^^ a field by this name exists in `Self`
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0433]: failed to resolve: use of undeclared type `UselessSendConstraint`
    |
    |
245 |             UselessSendConstraint: UselessSendConstraint,
    |             ^^^^^^^^^^^^^^^^^^^^^ use of undeclared type `UselessSendConstraint`
Some errors have detailed explanations: E0412, E0425, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_lint` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
