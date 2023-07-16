plain
 finished in 0.600 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 162 tests
............................................F....F......................................  88/162

failures:

---- [incremental] tests/incremental/hashes/enum_constructors.rs stdout ----
---- [incremental] tests/incremental/hashes/enum_constructors.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/enum_constructors.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/enum_constructors.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_constructors/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(change_constructor_variant_c_like)` should be dirty but is not
   |
   |
LL | pub fn change_constructor_variant_c_like() {

error: aborting due to previous error
------------------------------------------



---- [incremental] tests/incremental/hashes/let_expressions.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/hashes/let_expressions.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/let_expressions.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/let_expressions/auxiliary" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans"
stdout: none
--- stderr -------------------------------
error: `optimized_mir(add_initializer)` should be dirty but is not
   |
LL | pub fn add_initializer() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^


error: `optimized_mir(change_initializer)` should be dirty but is not
   |
LL | pub fn change_initializer() {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^

