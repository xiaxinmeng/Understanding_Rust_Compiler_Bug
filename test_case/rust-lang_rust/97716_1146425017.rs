plain
 finished in 0.517 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 154 tests
....................................F.......F........................................... 88/154
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] src/test/incremental/hashes/call_expressions.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/call_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/call_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/call_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_callee_indirectly_function)` should be clean but is not
   |
LL |     pub fn change_callee_indirectly_function() {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to previous error
------------------------------------------


---- [incremental] src/test/incremental/hashes/indexing_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/indexing_expressions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/indexing_expressions.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/indexing_expressions/auxiliary"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_simple_index)` should be clean but is not
   |
LL | fn change_simple_index(slice: &[u32]) -> u32 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_lower_bound)` should be clean but is not
   |
   |
LL | fn change_lower_bound(slice: &[u32]) -> &[u32] {


error: `optimized_mir(change_upper_bound)` should be clean but is not
   |
   |
LL | fn change_upper_bound(slice: &[u32]) -> &[u32] {


error: `optimized_mir(add_lower_bound)` should be clean but is not
   |
   |
LL | fn add_lower_bound(slice: &[u32]) -> &[u32] {


error: `optimized_mir(add_upper_bound)` should be clean but is not
   |
   |
LL | fn add_upper_bound(slice: &[u32]) -> &[u32] {


error: `optimized_mir(change_mutability)` should be clean but is not
   |
   |
LL | fn change_mutability(slice: &mut [u32]) -> u32 {


error: `optimized_mir(exclusive_to_inclusive_range)` should be clean but is not
   |
   |
LL | fn exclusive_to_inclusive_range(slice: &[u32]) -> &[u32] {

error: aborting due to 7 previous errors
------------------------------------------

