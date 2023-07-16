plain
[00:57:16] ....i...............................................................................................
[00:57:24] ........i..ii.......................................................................................
[00:57:34] ....................................................................................................
[00:57:43] ....................................................................................................
[00:57:52] ..........................................................................i...F.FF..................
[00:58:13] ....................................................................................................
[00:58:22] ....................................................................................................
mpile-fail] compile-fail/rfc-2126-crate-paths/crate-visibility-ambiguity.rs stdout ----
[00:58:33]  
[00:58:33]  
[00:58:33] error: compile-fail test compiled successfully!
[00:58:33] status: exit code: 0
[00:58:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/rfc-2126-crate-paths/crate-visibility-ambiguity.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/rfc-2126-crate-paths/crate-visibility-ambiguity.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/rfc-2126-crate-paths/crate-visibility-ambiguity.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:58:33] ------------------------------------------
[00:58:33] 
[00:58:33] ------------------------------------------
[00:58:33] stderr:
[00:58:33] stderr:
[00:58:33] ------------------------------------------
[00:58:33] 
[00:58:33] ------------------------------------------
[00:58:33] 
[00:58:33] thread '[compile-fail] compile-fail/rfc-2126-crate-paths/crate-visibility-ambiguity.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2938:9
[00:58:33] ---- [compile-fail] compile-fail/rfc-2126-crate-paths/keyword-crate-as-identifier.rs stdout ----
[00:58:33]  
[00:58:33]  
[00:58:33] error: /checkout/src/test/compile-fail/rfc-2126-crate-paths/keyword-crate-as-identifier.rs:14: unexpected error: '14:9: 14:14: expected unit struct/variant or constant, found module `crate` [E0532]'
[00:58:33] 
[00:58:33] error: /checkout/src/test/compile-fail/rfc-2126-crate-paths/keyword-crate-as-identifier.rs:14: expected error not found: `crate` can only be used in absolute paths
[00:58:33] 
[00:58:33] error: 1 unexpected errors found, 1 expected errors not found
[00:58:33] status: exit code: 101
[00:58:33] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/rfc-2126-crate-paths/keyword-crate-as-identifier.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/rfc-2126-crate-paths/keyword-crate-as-identifier.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/rfc-2126-crate-paths/keyword-crate-as-identifier.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
[00:58:33] unexpected errors (from JSON output): [
[00:58:33]     Error {
[00:58:33]         line_num: 14,
[00:58:33]         kind: Some(
[00:58:33]             Error
[00:58:33]         ),
[00:58:33]         msg: "14:9: 14:14: expected unit struct/variant or constant, found module `crate` [E0532]"
[00:58:33] ]
[00:58:33] 
[00:58:33] not found errors (from test file): [
[00:58:33]     Error {
