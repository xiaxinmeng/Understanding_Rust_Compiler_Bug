plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/librustdoc/clean/utils.rs:33:25
     |
33   |     for &cnum in cx.tcx.crates().iter() {
     |
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/query/mod.rs:1447:11
     |
     |
12   | / rustc_queries! {
13   | |     query trigger_delay_span_bug(key: DefId) -> () {
14   | |         desc { "trigger a delay span bug" }
...    |
...    |
1447 | |     query crates(_: ()) -> &'tcx [CrateNum] {
...    |
1714 | |     }
1715 | | }
     | |_- in this expansion of `rustc_query_append!`
     | |_- in this expansion of `rustc_query_append!`
     | 
    ::: /checkout/compiler/rustc_middle/src/ty/query/mod.rs:284:1
     |
284  |   rustc_query_append! { [define_callbacks!][<'tcx>] }
     |   --------------------------------------------------- in this macro invocation
help: expected the unit value `()`; create it with empty parentheses
     |
33   |     for &cnum in cx.tcx.crates(()).iter() {

error[E0061]: this function takes 1 argument but 0 arguments were supplied
    --> src/librustdoc/passes/collect_trait_impls.rs:33:25
     |
     |
33   |     for &cnum in cx.tcx.crates().iter() {
     |
note: associated function defined here
    --> /checkout/compiler/rustc_middle/src/query/mod.rs:1447:11
     |
     |
12   | / rustc_queries! {
13   | |     query trigger_delay_span_bug(key: DefId) -> () {
14   | |         desc { "trigger a delay span bug" }
...    |
...    |
1447 | |     query crates(_: ()) -> &'tcx [CrateNum] {
...    |
1714 | |     }
1715 | | }
     | |_- in this expansion of `rustc_query_append!`
     | |_- in this expansion of `rustc_query_append!`
     | 
    ::: /checkout/compiler/rustc_middle/src/ty/query/mod.rs:284:1
     |
284  |   rustc_query_append! { [define_callbacks!][<'tcx>] }
     |   --------------------------------------------------- in this macro invocation
help: expected the unit value `()`; create it with empty parentheses
     |
33   |     for &cnum in cx.tcx.crates(()).iter() {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0061`.
