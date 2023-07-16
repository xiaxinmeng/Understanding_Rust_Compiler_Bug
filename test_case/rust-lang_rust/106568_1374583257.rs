plain
failures:

---- [incremental] src/test/incremental/hashes/while_let_loops.rs stdout ----

error in revision `cfail5`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/while_let_loops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail5" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/while_let_loops.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/while_let_loops/auxiliary" "-Z" "query-dep-graph" "-O"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_loop_body)` should be clean but is not
   |
   |
LL | pub fn change_loop_body() {


error: `optimized_mir(change_loop_condition)` should be clean but is not
   |
LL | pub fn change_loop_condition() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(add_break)` should be clean but is not
   |
   |
LL | pub fn add_break() {


error: `optimized_mir(add_loop_label)` should be clean but is not
   |
   |
LL | pub fn add_loop_label() {


error: `optimized_mir(add_loop_label_to_break)` should be clean but is not
   |
   |
LL | pub fn add_loop_label_to_break() {


error: `optimized_mir(change_break_label)` should be clean but is not
   |
   |
LL | pub fn change_break_label() {


error: `optimized_mir(add_loop_label_to_continue)` should be clean but is not
   |
   |
LL | pub fn add_loop_label_to_continue() {


error: `optimized_mir(change_continue_label)` should be clean but is not
   |
LL | pub fn change_continue_label() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_continue_to_break)` should be clean but is not
   |
   |
LL | pub fn change_continue_to_break() {

error: aborting due to 9 previous errors
------------------------------------------

