plain
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:52:01] 
[00:52:01] running 90 tests
inux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/struct_point.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/add_private_fn_at_krate_root_cc/struct_point/auxiliary"
[00:52:13] ------------------------------------------
[00:52:13] 
[00:52:13] ------------------------------------------
[00:52:13] stderr:
[00:52:13] stderr:
[00:52:13] ------------------------------------------
[00:52:13] thread 'main' panicked at 'Found unstable fingerprints for DefinedLibFeatures(point[8787])', librustc/ty/query/plumbing.rs:495:13
[00:52:13] 
[00:52:13] error: internal compiler error: unexpected panic
[00:52:13] 
[00:52:13] 
[00:52:13] note: the compiler unexpectedly panicked. this is a bug.
[00:52:13] 
[00:52:13] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:52:13] note: rustc 1.29.0-dev running on x86_64-unknown-linux-gnu
[00:52:13] 
[00:52:13] 
[00:52:13] note: compiler flags: -Z incremental-verify-ich -Z incremental-queries -Z ui-testing -Z unstable-options -Z query-dep-graph -C incremental -C prefer-dynamic -amic -C rpath
[00:52:13] 
[00:52:13] ------------------------------------------
[00:52:13] 
[00:52:13] thread '[incremental] incremental/remapped_paths_cc/main.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3137:9
---
[00:52:13] test result: FAILED. 86 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out
[00:52:13] 
[00:52:13] 
[00:52:13] 
[00:52:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/incremental" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "incremental" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers3989304 .
2529944 ./obj/build
1936136 ./obj/build/x86_64-unknown-linux-gnu
785768 ./src
564072 ./.git
---
145372 ./obj/build/bootstrap/debug/incremental
134600 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
134596 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
130560 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02
130556 ./obj/build/bootstrap/debug/incremental/bootstrap-2fbxwhl9tnp02/s-f36mnvnraf-3z6edg-3om13ebi8plfd
128672 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknow
