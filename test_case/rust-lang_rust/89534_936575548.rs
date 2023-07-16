plain
     |                     ^^^^^^^^^^ help: a local variable with a similar name exists: `derives`

   Compiling rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
   Compiling rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: no method named `get` found for reference `&DiagnosticItems` in the current scope
    --> compiler/rustc_typeck/src/check/method/suggest.rs:1120:61
     |
1120 |                     .any(|trait_derivable| diagnostic_items.get(trait_derivable) == Some(&did))
     |                                                             ^^^ method not found in `&DiagnosticItems`
Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
