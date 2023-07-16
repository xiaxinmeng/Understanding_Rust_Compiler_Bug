plain
 finished in 0.611 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 162 tests
..............................................F.FF...FF.................................  88/162

failures:

---- [incremental] tests/incremental/hashes/enum_constructors.rs stdout ----
---- [incremental] tests/incremental/hashes/enum_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_constructor_path_c_like)` should be dirty but is not
   |
   |
LL | pub fn change_constructor_path_c_like() {

error: aborting due to previous error
------------------------------------------



---- [incremental] tests/incremental/hashes/if_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/if_expressions.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/if_expressions.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/if_expressions/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_condition_if_let)` should be dirty but is not
   |
   |
LL | pub fn change_condition_if_let(x: Option<u32>) -> u32 {

error: aborting due to previous error
------------------------------------------



---- [incremental] tests/incremental/hashes/let_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_name)` should be dirty but is not
   |
LL | pub fn change_name() {
   | ^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_type)` should be dirty but is not
   |
LL | pub fn change_type() {
   | ^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_mutability_of_reference_type)` should be dirty but is not
   |
LL | pub fn change_mutability_of_reference_type() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_mutability_of_slot)` should be dirty but is not
   |
LL | pub fn change_mutability_of_slot() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_simple_binding_to_pattern)` should be dirty but is not
   |
LL | pub fn change_simple_binding_to_pattern() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_name_in_pattern)` should be dirty but is not
   |
LL | pub fn change_name_in_pattern() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(add_ref_in_pattern)` should be dirty but is not
   |
LL | pub fn add_ref_in_pattern() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(add_amp_in_pattern)` should be dirty but is not
   |
   |
LL | pub fn add_amp_in_pattern() {


error: `optimized_mir(change_mutability_of_binding_in_pattern)` should be dirty but is not
   |
LL | pub fn change_mutability_of_binding_in_pattern() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to 9 previous errors
------------------------------------------


---- [incremental] tests/incremental/hashes/for_loops.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/for_loops.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/for_loops.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/for_loops/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_iteration_variable_name)` should be dirty but is not
   |
LL | pub fn change_iteration_variable_name() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_iteration_variable_pattern)` should be dirty but is not
   |
LL | pub fn change_iteration_variable_pattern() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: aborting due to 2 previous errors
------------------------------------------


---- [incremental] tests/incremental/hashes/match_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/match_expressions.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/match_expressions.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/match_expressions/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_name_of_at_binding)` should be dirty but is not
   |
   |
LL | pub fn change_name_of_at_binding(x: u32) -> u32 {


error: `optimized_mir(change_simple_name_to_pattern)` should be dirty but is not
   |
   |
LL | pub fn change_simple_name_to_pattern(x: u32) -> u32 {


error: `optimized_mir(change_name_in_pattern)` should be dirty but is not
   |
   |
LL | pub fn change_name_in_pattern(x: u32) -> u32 {


error: `optimized_mir(change_mutability_of_binding_in_pattern)` should be dirty but is not
   |
   |
LL | pub fn change_mutability_of_binding_in_pattern(x: u32) -> u32 {


error: `optimized_mir(add_ref_to_binding_in_pattern)` should be dirty but is not
   |
   |
LL | pub fn add_ref_to_binding_in_pattern(x: u32) -> u32 {


error: `optimized_mir(add_amp_to_binding_in_pattern)` should be dirty but is not
   |
   |
LL | pub fn add_amp_to_binding_in_pattern(x: u32) -> u32 {

error: aborting due to 6 previous errors
------------------------------------------

