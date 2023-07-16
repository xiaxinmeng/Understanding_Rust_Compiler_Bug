
---- [incremental] incremental/spans_in_type_debuginfo.rs stdout ----

error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/spans_in_type_debuginfo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/spans_in_type_debuginfo.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/spans_in_type_debuginfo/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
warning: variant is never constructed: `B`
  --> /checkout/src/test/incremental/spans_in_type_debuginfo.rs:39:9
   |
LL |         B(u32),
   |         ^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

error: CGU-reuse for `spans_in_type_debuginfo-enums` is `No` but should be at least `PreLto`
  --> /checkout/src/test/incremental/spans_in_type_debuginfo.rs:9:1
   |
LL | #![rustc_partition_reused(module="spans_in_type_debuginfo-enums", cfg="rpass2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: CGU-reuse for `spans_in_type_debuginfo-structs` is `No` but should be at least `PreLto`
  --> /checkout/src/test/incremental/spans_in_type_debuginfo.rs:8:1
   |
LL | #![rustc_partition_reused(module="spans_in_type_debuginfo-structs", cfg="rpass2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
error: aborting due to 2 previous errors; 1 warning emitted

------------------------------------------
