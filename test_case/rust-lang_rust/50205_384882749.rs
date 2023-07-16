plain
[01:05:36] 
[01:05:36] running 271 tests
[01:05:37] ...........................i........................................................................
[01:05:39] ....................................................................i...............................
[01:05:40] ...............i...............................F.......................
[01:05:40] 
[01:05:40] ---- [parse-fail] parse-fail/trait-object-lifetime-parens.rs stdout ----
[01:05:40]  
[01:05:40]  
[01:05:40] error: /checkout/src/test/parse-fail/trait-object-lifetime-parens.rs:16: expected error not found: parenthesized lifetime bounds are not supported
[01:05:40] 
[01:05:40] error: /checkout/src/test/parse-fail/trait-object-lifetime-parens.rs:17: expected error not found: expected type, found `'a`
[01:05:40] 
[01:05:40] error: 0 unexpected errors found, 2 expected errors not found
[01:05:40] status: exit code: 101
[01:05:40] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/parse-fail/trait-object-lifetime-parens.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-object-lifetime-parens.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "parse-only" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/parse-fail/trait-object-lifetime-parens.stage2-x86_64-unknown-linux-gnu.aux"
[01:05:40] not found errors (from test file): [
[01:05:40]     Error {
[01:05:40]         line_num: 16,
[01:05:40]         kind: Some(
[01:05:40]             Error
[01:05:40]         ),
[01:05:40]         msg: "parenthesized lifetime bounds are not supported"
[01:05:40]     Error {
[01:05:40]         line_num: 17,
[01:05:40]         kind: Some(
[01:05:40]             Error
[01:05:40]             Error
[01:05:40]         ),
[01:05:40]         msg: "expected type, found `\'a`"
[01:05:40] ]
[01:05:40] 
[01:05:40] thread '[parse-fail] parse-fail/trait-object-lifetime-parens.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:1276:13
[01:05:40] note: Run with `RUST_BACKTRACE=1` for a backtrace.
