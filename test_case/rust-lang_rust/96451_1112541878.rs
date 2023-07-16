plain
failures:

---- [incremental] src/test/incremental/hashes/match_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_mutability_of_binding_in_pattern)` should be dirty but is not
   |
   |
LL | pub fn change_mutability_of_binding_in_pattern(x: u32) -> u32 {

error: aborting due to previous error
------------------------------------------

