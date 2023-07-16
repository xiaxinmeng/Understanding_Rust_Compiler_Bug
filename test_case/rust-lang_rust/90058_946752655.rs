plain
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0428]: the name `strip` is defined multiple times
    --> compiler/rustc_session/src/options.rs:270:9
     |
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $opt(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `strip` redefined here
     | |         previous definition of the value `strip` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
899  | / options! {
900  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
902  | |     // This list is in alphabetical order.
...    |
1007 | |     // - src/doc/rustc/src/codegen-options/index.md
1008 | | }
1008 | | }
     | |_- in this macro invocation
     |
     = note: `strip` must be defined only once in the value namespace of this module
    Checking chalk-engine v0.55.0
error[E0308]: mismatched types
    --> compiler/rustc_session/src/options.rs:267:34
     |
     |
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
267  | |         &[ $( (stringify!($opt), $opt, desc::$parse, $desc) ),* ];
     | |                                  ^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1010 | / options! {
1011 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1013 | |     // This list is in alphabetical order.
...    |
1374 | |     // - compiler/rustc_interface/src/tests.rs
1375 | | }
1375 | | }
     | |_- in this macro invocation
     |
     = note: expected fn pointer `for<'r, 's> fn(&'r mut options::DebuggingOptions, Option<_>) -> _`
                   found fn item `for<'r, 's> fn(&'r mut options::CodegenOptions, Option<_>) -> _ {options::strip}`
Some errors have detailed explanations: E0308, E0428.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_session` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
