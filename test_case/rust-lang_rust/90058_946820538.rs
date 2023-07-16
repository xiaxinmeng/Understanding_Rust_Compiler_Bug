plain
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0433]: failed to resolve: use of undeclared crate or module `parse`
    --> compiler/rustc_session/src/options.rs:272:13
     |
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $optmod:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
272  | |             parse::$parse(&mut redirect_field!(cg.$opt), v)
     | |             ^^^^^ use of undeclared crate or module `parse`
276  | |
277  | | ) }
     | |___- in this expansion of `options!`
...
...
901  | / options! {
902  | |     CodegenOptions, CG_OPTIONS, cgopts, "C", "codegen",
904  | |     // This list is in alphabetical order.
...    |
1009 | |     // - src/doc/rustc/src/codegen-options/index.md
1010 | | }
1010 | | }
     | |_- in this macro invocation

error[E0433]: failed to resolve: use of undeclared crate or module `parse`
    --> compiler/rustc_session/src/options.rs:272:13
     |
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $optmod:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
272  | |             parse::$parse(&mut redirect_field!(cg.$opt), v)
     | |             ^^^^^ use of undeclared crate or module `parse`
276  | |
277  | | ) }
     | |___- in this expansion of `options!`
...
...
1012 | / options! {
1013 | |     DebuggingOptions, DB_OPTIONS, dbopts, "Z", "debugging",
1015 | |     // This list is in alphabetical order.
...    |
1376 | |     // - compiler/rustc_interface/src/tests.rs
1377 | | }
1377 | | }
     | |_- in this macro invocation

error[E0412]: cannot find type `CodegenOptions` in this scope
   --> compiler/rustc_session/src/options.rs:902:5
    |
902 |     CodegenOptions, CG_OPTIONS, cgopts, "C", "codegen",
    |
    = note: consider importing this struct:
            crate::options::CodegenOptions


error[E0412]: cannot find type `DebuggingOptions` in this scope
    --> compiler/rustc_session/src/options.rs:1013:5
     |
1013 |     DebuggingOptions, DB_OPTIONS, dbopts, "Z", "debugging",
     |
     = note: consider importing this struct:
             crate::options::DebuggingOptions

