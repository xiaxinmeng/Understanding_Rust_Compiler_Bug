plain
   Compiling rustc_macros v0.1.0 (/checkout/compiler/rustc_macros)
error[E0433]: failed to resolve: could not find `LanguageIdentifier` in the crate root
  --> compiler/rustc_macros/src/diagnostics/fluent.rs:91:45
   |
91 |     let mut bundle = FluentBundle::new(vec![langid!("en-US")]);
   |                                             ^^^^^^^^^^^^^^^^ not found in `$crate`
   |
   = note: this error originates in the macro `proc_macro_call` which comes from the expansion of the macro `langid` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
1  | use unic_langid::LanguageIdentifier;
   |
   |
help: if you import `LanguageIdentifier`, refer to it directly


4  - #[proc_macro_hack]
4  + #[proc_macro_hack]

error[E0433]: failed to resolve: unresolved import
  --> compiler/rustc_macros/src/diagnostics/fluent.rs:91:45
   |
   |
91 |     let mut bundle = FluentBundle::new(vec![langid!("en-US")]);
   |                                             ^^^^^^^^^^^^^^^^ not found in `$crate::subtags`
   |
   = note: this error originates in the macro `proc_macro_call` which comes from the expansion of the macro `langid` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
   |
1  | use unic_langid::subtags::Language;
   |
help: if you import `Language`, refer to it directly


4  - #[proc_macro_hack]
4  + #[proc_macro_hack]

error[E0433]: failed to resolve: unresolved import
  --> compiler/rustc_macros/src/diagnostics/fluent.rs:91:45
   |
   |
91 |     let mut bundle = FluentBundle::new(vec![langid!("en-US")]);
   |                                             ^^^^^^^^^^^^^^^^ not found in `$crate::subtags`
   |
   = note: this error originates in the macro `proc_macro_call` which comes from the expansion of the macro `langid` (in Nightly builds, run with -Z macro-backtrace for more info)
   |
1  | use unic_langid::subtags::Region;
   |
   |
help: if you import `Region`, refer to it directly


4  - #[proc_macro_hack]
4  + #[proc_macro_hack]

   Compiling tracing-tree v0.2.0
   Compiling chalk-solve v0.80.0
   Compiling rustc_log v0.0.0 (/checkout/compiler/rustc_log)
