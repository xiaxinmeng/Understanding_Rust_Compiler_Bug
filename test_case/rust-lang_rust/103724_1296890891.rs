plain
failures:

---- [incremental] src/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused.rs stdout ----

error in revision `cfail1`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused/issue-59524-layout-scalar-valid-range-is-not-unused.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused/auxiliary"
stdout: none
--- stderr -------------------------------
error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
   |
   |
LL | #[rustc_layout_scalar_valid_range_start(10)]


error: attributes starting with `rustc` are reserved for use by the `rustc` compiler
   |
LL | #[rustc_layout_scalar_valid_range_end(30)]
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^


error: cannot find attribute `rustc_layout_scalar_valid_range_start` in this scope
  --> /checkout/src/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused.rs:12:3
   |
LL | #[rustc_layout_scalar_valid_range_start(10)]

error: cannot find attribute `rustc_layout_scalar_valid_range_end` in this scope
  --> /checkout/src/test/incremental/issue-59524-layout-scalar-valid-range-is-not-unused.rs:13:3
   |
