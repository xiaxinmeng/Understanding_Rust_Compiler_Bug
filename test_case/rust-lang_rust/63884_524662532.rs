
error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit code: 1
command: "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/home/r/src/rust/rustc.3/src/test/incremental/thinlto/cgu_invalidated_via_import.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/cgu_invalidated_via_import.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-L" "/home/r/src/rust/rustc.3/build/x86_64-unknown-linux-gnu/test/incremental/thinlto/cgu_invalidated_via_import/auxiliary"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error: CGU-reuse for `cgu_invalidated_via_import-bar` is `No` but should be `PreLto`
  --> /home/r/src/rust/rustc.3/src/test/incremental/thinlto/cgu_invalidated_via_import.rs:19:1
   |
LL | / #![rustc_expected_cgu_reuse(module="cgu_invalidated_via_import-bar",
LL | |                             cfg="cfail2",
LL | |                             kind="pre-lto")]
   | |____________________________________________^

error: aborting due to previous error
