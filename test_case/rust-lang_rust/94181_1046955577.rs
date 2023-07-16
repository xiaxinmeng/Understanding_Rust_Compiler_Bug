plain
    Checking rustc_ast_pretty v0.0.0 (/checkout/compiler/rustc_ast_pretty)
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0425]: cannot find value `parse_debug_name_table_kind` in module `desc`
     |
     |
1189 |     debug_name_table_kind: DebugNameTableKind = (DebugNameTableKind::Default, parse_debug_name_table_kind, [TRACKED],
     |                                                                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `desc`
help: consider importing this function
     |
1    | use crate::options::parse::parse_debug_name_table_kind;
     |
     |

error[E0560]: struct `options::Options` has no field named `debug_name_table_kind`
    |
    |
779 |             debug_name_table_kind: DebugNameTableKind::None,
    |             ^^^^^^^^^^^^^^^^^^^^^ `options::Options` does not have this field
    |
    = note: available fields are: `crate_types`, `optimize`, `debug_assertions`, `debuginfo`, `lint_opts` ... and 31 others

error[E0277]: the trait bound `config::DebugNameTableKind: DepTrackingHash` is not satisfied
     |
244  | / macro_rules! options {
244  | / macro_rules! options {
245  | |     ($struct_name:ident, $stat:ident, $optmod:ident, $prefix:expr, $outputname:expr,
246  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
247  | |         $init:expr,
...    |
274  | |                             &self.$opt,
     | |                             ^^^^^^^^^^ the trait `DepTrackingHash` is not implemented for `config::DebugNameTableKind`
299  | |
300  | | ) }
     | |___- in this expansion of `options!`
...
...
1145 | / options! {
1146 | |     DebuggingOptions, DB_OPTIONS, dbopts, "Z", "debugging",
1148 | |     // This list is in alphabetical order.
...    |
1535 | |     // - compiler/rustc_interface/src/tests.rs
1536 | | }
