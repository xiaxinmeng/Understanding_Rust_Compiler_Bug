
failures:

---- [incremental] incremental/hygiene/load_cached_hygiene.rs stdout ----

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
error in revision `rpass2`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hygiene/load_cached_hygiene.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--cfg" "rpass2" "-C" "incremental=/checkout/obj/build/i686-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/load_cached_hygiene.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/a" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/incremental/hygiene/load_cached_hygiene/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: CGU-reuse for `load_cached_hygiene-call_changed_function` is `PostLto` but should be `No`
  --> /checkout/src/test/incremental/hygiene/load_cached_hygiene.rs:27:1
   |
LL | #![rustc_partition_codegened(module="load_cached_hygiene-call_changed_function", cfg="rpass2")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error


------------------------------------------


---- [incremental] incremental/remapped_paths_cc/main.rs stdout ----

error in revision `rpass3`: compilation failed!
status: exit code: 1
command: "/checkout/obj/build/i686-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/remapped_paths_cc/main.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--cfg" "rpass3" "-C" "incremental=/checkout/obj/build/i686-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/main.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/i686-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/a" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-g" "-L" "/checkout/obj/build/i686-unknown-linux-gnu/test/incremental/remapped_paths_cc/main/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: CGU-reuse for `main-some_mod` is `PostLto` but should be `No`
  --> /checkout/src/test/incremental/remapped_paths_cc/main.rs:14:1
   |
LL | #![rustc_partition_codegened(module="main-some_mod", cfg="rpass3")]
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error


------------------------------------------



failures:
    [incremental] incremental/hygiene/load_cached_hygiene.rs
    [incremental] incremental/remapped_paths_cc/main.rs

test result: FAILED. 121 passed; 2 failed; 1 ignored; 0 measured; 0 filtered out
