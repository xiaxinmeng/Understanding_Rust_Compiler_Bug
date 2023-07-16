plain
    Checking chalk-solve v0.55.0
    Checking rustc_errors v0.0.0 (/checkout/compiler/rustc_errors)
    Checking rustc_session v0.0.0 (/checkout/compiler/rustc_session)
    Checking rustc_hir_pretty v0.0.0 (/checkout/compiler/rustc_hir_pretty)
error[E0425]: cannot find value `assert_incr_state` in this scope
     |
2195 |         assert_incr_state,
     |         ^^^^^^^^^^^^^^^^^ not found in this scope
     |
     |
note: function `crate::options::assert_incr_state` exists but is inaccessible
     |
222  | / macro_rules! options {
222  | / macro_rules! options {
223  | |     ($struct_name:ident, $stat:ident, $prefix:expr, $outputname:expr,
224  | |      $($( #[$attr:meta] )* $opt:ident : $t:ty = (
225  | |         $init:expr,
...    |
271  | |         fn $opt(cg: &mut $struct_name, v: Option<&str>) -> bool {
     | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not accessible
275  | |
276  | | ) }
     | |___- in this expansion of `options!`
...
...
1030 | / options! {
1031 | |     DebuggingOptions, DB_OPTIONS, "Z", "debugging",
1033 | |     // This list is in alphabetical order.
...    |
1401 | |     // - compiler/rustc_interface/src/tests.rs
1402 | | }
1402 | | }
     | |_- in this macro invocation

error[E0425]: cannot find value `parse_incr_state_assertion` in module `desc`
     |
     |
1046 |     assert_incr_state: Option<IncrementalState> = (None, parse_incr_state_assertion, [UNTRACKED],
     |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `desc`
     |
note: function `crate::config::parse_incr_state_assertion` exists but is inaccessible
     |
     |
1627 | / fn parse_incr_state_assertion(// matches: &getopts::Matches,
1628 | |     // assertion: Option<String>,
1629 | | ) -> Option<IncrementalState> {
1630 | |     // to to find existing dir
1639 | |     None
1640 | | }
     | |_^ not accessible


error[E0425]: cannot find function `parse_incr_state_assertion` in module `parse`
     |
     |
1046 |     assert_incr_state: Option<IncrementalState> = (None, parse_incr_state_assertion, [UNTRACKED],
     |                                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `parse`
     |
note: function `crate::config::parse_incr_state_assertion` exists but is inaccessible
     |
     |
1627 | / fn parse_incr_state_assertion(// matches: &getopts::Matches,
1628 | |     // assertion: Option<String>,
1629 | | ) -> Option<IncrementalState> {
1630 | |     // to to find existing dir
1639 | |     None
1640 | | }
     | |_^ not accessible

