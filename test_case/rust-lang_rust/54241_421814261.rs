plain
[00:16:28]    Compiling rustc_privacy v0.0.0 (file:///checkout/src/librustc_privacy)
[00:18:03]    Compiling rustc_save_analysis v0.0.0 (file:///checkout/src/librustc_save_analysis)
[00:18:05]    Compiling rustc_codegen_utils v0.0.0 (file:///checkout/src/librustc_codegen_utils)
[00:18:05]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[00:18:05] error[E0252]: the name `Applicability` is defined multiple times
[00:18:05]   --> librustc_borrowck/borrowck/mod.rs:48:62
[00:18:05]    |
[00:18:05] 48 | use errors::{Applicability, DiagnosticBuilder, DiagnosticId, Applicability};
[00:18:05]    |              -------------                                   ^^^^^^^^^^^^^ `Applicability` reimported here
[00:18:05]    |              |
[00:18:05]    |              previous import of the type `Applicability` here
[00:18:05]    |
[00:18:05]    = note: `Applicability` must be defined only once in the type namespace of this module
[00:18:05] help: You can use `as` to change the binding name of the import
[00:18:05]    |
[00:18:05] 48 | use errors::{Applicability, DiagnosticBuilder, DiagnosticId, Applicability as OtherApplicability};
[00:18:05] 
[00:18:06] error: unused import: `Applicability`
[00:18:06]   --> librustc_borrowck/borrowck/mod.rs:48:62
[00:18:06]    |
[00:18:06]    |
[00:18:06] 48 | use errors::{Applicability, DiagnosticBuilder, DiagnosticId, Applicability};
[00:18:06]    |
[00:18:06]    = note: `-D unused-imports` implied by `-D warnings`
[00:18:06] 
[00:18:06]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
