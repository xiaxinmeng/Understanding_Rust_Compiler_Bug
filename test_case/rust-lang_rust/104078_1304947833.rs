plain
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error[E0308]: mismatched types
  --> builder/tests.rs:13:22
   |
13 |     config.dry_run = true;
   |     --------------   ^^^^ expected enum `DryRun`, found `bool`
   |     expected due to the type of this binding

For more information about this error, try `rustc --explain E0308`.
error: could not compile `bootstrap` due to previous error
