plain
failures:

---- [incremental] tests/incremental/change_private_fn/struct_point.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/incremental/change_private_fn/struct_point.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/change_private_fn/struct_point/auxiliary" "-Z" "query-dep-graph"
stdout: none
--- stderr -------------------------------
--- stderr -------------------------------
error: `typeck(fn_calls_methods_in_same_impl::check)` should have been loaded from disk but it was not
   |
LL |     pub fn check() {
   |     ^^^^^^^^^^^^^^

