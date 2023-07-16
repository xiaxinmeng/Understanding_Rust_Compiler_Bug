plain
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0428]: the name `parse_bool` is defined multiple times
    --> compiler/rustc_session/src/options.rs:270:9
     |
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_bool` redefined here
     | |         previous definition of the value `parse_bool` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_bool` must be defined only once in the value namespace of this module
error[E0428]: the name `parse_string` is defined multiple times
    --> compiler/rustc_session/src/options.rs:270:9
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_string` redefined here
     | |         previous definition of the value `parse_string` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_string` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_opt_bool` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_opt_bool` redefined here
     | |         previous definition of the value `parse_opt_bool` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_opt_bool` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_opt_number` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_opt_number` redefined here
     | |         previous definition of the value `parse_opt_number` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_opt_number` must be defined only once in the value namespace of this module
error[E0428]: the name `parse_list` is defined multiple times
    --> compiler/rustc_session/src/options.rs:270:9
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_list` redefined here
     | |         previous definition of the value `parse_list` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_list` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_no_flag` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_no_flag` redefined here
     | |         previous definition of the value `parse_no_flag` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_no_flag` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_opt_pathbuf` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_opt_pathbuf` redefined here
     | |         previous definition of the value `parse_opt_pathbuf` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_opt_pathbuf` must be defined only once in the value namespace of this module
error[E0428]: the name `parse_opt_string` is defined multiple times
    --> compiler/rustc_session/src/options.rs:270:9
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_opt_string` redefined here
     | |         previous definition of the value `parse_opt_string` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_opt_string` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_string_push` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_string_push` redefined here
     | |         previous definition of the value `parse_string_push` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_string_push` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_sanitizers` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_sanitizers` redefined here
     | |         previous definition of the value `parse_sanitizers` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: `parse_sanitizers` must be defined only once in the value namespace of this module

error[E0428]: the name `parse_switch_with_opt_path` is defined multiple times
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_switch_with_opt_path` redefined here
     | |         previous definition of the value `parse_switch_with_opt_path` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
904  | / options! {
905  | |     CodegenOptions, CG_OPTIONS, "C", "codegen",
907  | |     // This list is in alphabetical order.
...    |
1012 | |     // - src/doc/rustc/src/codegen-options/index.md
1013 | | }
1013 | | }
     | |_- in this macro invocation
     |
     = note: `parse_switch_with_opt_path` must be defined only once in the value namespace of this module
error[E0428]: the name `parse_opt_comma_list` is defined multiple times
    --> compiler/rustc_session/src/options.rs:270:9
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
270  | |         fn $parse(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         |
     | |         |
     | |         `parse_opt_comma_list` redefined here
     | |         previous definition of the value `parse_opt_comma_list` here
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: `parse_opt_comma_list` must be defined only once in the value namespace of this module
error[E0308]: mismatched types
    --> compiler/rustc_session/src/options.rs:267:34
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
267  | |         &[ $( (stringify!($opt), $parse, desc::$parse, $desc) ),* ];
     | |                                  ^^^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: expected fn pointer `for<'r, 's> fn(&'r mut options::DebuggingOptions, Option<_>) -> _`
                   found fn item `for<'r, 's> fn(&'r mut options::CodegenOptions, Option<_>) -> _ {options::parse_bool}`
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
267  | |         &[ $( (stringify!($opt), $parse, desc::$parse, $desc) ),* ];
     | |                                  ^^^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: expected fn pointer `for<'r, 's> fn(&'r mut options::DebuggingOptions, Option<_>) -> _`
                   found fn item `for<'r, 's> fn(&'r mut options::CodegenOptions, Option<_>) -> _ {options::parse_string}`
error[E0308]: mismatched types
    --> compiler/rustc_session/src/options.rs:267:34
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
267  | |         &[ $( (stringify!($opt), $parse, desc::$parse, $desc) ),* ];
     | |                                  ^^^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: expected fn pointer `for<'r, 's> fn(&'r mut options::DebuggingOptions, Option<_>) -> _`
                   found fn item `for<'r, 's> fn(&'r mut options::CodegenOptions, Option<_>) -> _ {options::parse_opt_string}`
error[E0308]: mismatched types
    --> compiler/rustc_session/src/options.rs:267:34
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
267  | |         &[ $( (stringify!($opt), $parse, desc::$parse, $desc) ),* ];
     | |                                  ^^^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: expected fn pointer `for<'r, 's> fn(&'r mut options::DebuggingOptions, Option<_>) -> _`
                   found fn item `for<'r, 's> fn(&'r mut options::CodegenOptions, Option<_>) -> _ {options::parse_string_push}`
error[E0308]: mismatched types
    --> compiler/rustc_session/src/options.rs:267:34
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
267  | |         &[ $( (stringify!($opt), $parse, desc::$parse, $desc) ),* ];
     | |                                  ^^^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
1015 | / options! {
1016 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1018 | |     // This list is in alphabetical order.
...    |
1379 | |     // - compiler/rustc_interface/src/tests.rs
1380 | | }
1380 | | }
     | |_- in this macro invocation
     |
     = note: expected fn pointer `for<'r, 's> fn(&'r mut options::DebuggingOptions, Option<_>) -> _`
                   found fn item `for<'r, 's> fn(&'r mut options::CodegenOptions, Option<_>) -> _ {options::parse_opt_bool}`
error[E0308]: mismatched types
    --> compiler/rustc_session/src/options.rs:267:34
     |
221  | / macro_rules! options {
221  | / macro_rules! options {
222  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
223  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
224  | |         $init:expr,
...    |
267  | |         &[ $( (stringify!($opt), $parse, desc::$parse, $desc) ),* ];
     | |                                  ^^^^^^ expected struct `options::DebuggingOptions`, found struct `options::CodegenOptions`
274  | |
275  | | ) }
     | |___- in this expansion of `options!`
...
...
