plain
[00:44:53] ....................................................................................................
[00:44:59] ....................................................................................................
[00:45:05] ....................................................................................................
[00:45:12] ..................................i.................................................................
[00:45:17] ..........i.........................................F...............................................
[00:45:30] ....................................................................................................
##############################################################################################################################################################################################################################
[00:45:35] 5    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:45:35] 
[00:45:35] 
[00:45:35] 
[00:45:35] The actual stderr differed from the expected stderr.
[00:45:35] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stderr
[00:45:35] To update references, run this command from build directory:
[00:45:35] /checkout/src/test/ui/update-references.sh '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui' 'raw_string_hash_count.rs'
[00:45:35] error: 1 errors occurred comparing output.
[00:45:35] status: exit code: 101
[00:45:35] status: exit code: 101
[00:45:35] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/raw_string_hash_count.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/raw_string_hash_count.stage2-x86_64-unkn#################################################\"test\"################################################################################################################################################################################################################################################################\n   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
[00:45:35] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:45:35] ------------------------------------------
[00:45:35] 
[00:45:35] thread '[ui] ui/raw_string_hash_count.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2965:9
[00:45:35] note: Run with `RUST_BACKTRACE=1` for a backtrace.
---
[00:45:35] 
[00:45:35] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[00:45:35] 
[00:45:35] 
[00:45:35] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental
111308 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release
107424 ./obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps
102820 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
102808 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
102808 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm
102804 ./obj/build/bootstrap/debug/incremental/bootstrap-2wettvttcntnm/s-f0j68ebhzh-17j24r9-v7hez0f2m5v0
90648 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90648 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz
90644 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/incremental/core-31lccp6wy7orz/s-f0j65upgpu-17ogum3-8jol7z8wxzfn
89732 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
89684 ./src/llvm/test/CodeGen
86660 ./obj/build/x86_64-unknown-linux-gnu/doc/core
84396 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
