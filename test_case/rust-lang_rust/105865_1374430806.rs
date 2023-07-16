plain
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [incremental] src/test/incremental/thinlto/cgu_keeps_identical_fn.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/incremental/thinlto/cgu_keeps_identical_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn/cgu_keeps_identical_fn.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_keeps_identical_fn/auxiliary" "-Z" "query-dep-graph" "-O"
stdout: none
--- stderr -------------------------------
error: CGU-reuse for `cgu_keeps_identical_fn-foo` is `PostLto ` but should be `No`
   |
   |
LL | #![rustc_expected_cgu_reuse(module = "cgu_keeps_identical_fn-foo", cfg = "cfail2", kind = "no")]

error: aborting due to previous error
------------------------------------------

