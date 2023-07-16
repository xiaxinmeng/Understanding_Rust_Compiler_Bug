plain

[01:00:52] [TIMING] ToolBuild { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-freebsd", tool: "rls", path: "src/tools/rls", mode: ToolRustc, is_ext_tool: true, extra_features: [] } -- 82.679
[01:00:52] Unable to build RLS, skipping dist
[01:00:52] Dist LlvmTools stage2 (x86_64-unknown-freebsd)
[01:00:52] thread 'main' panicked at 'fs::File::open(&src) failed with No such file or directory (os error 2)', bootstrap/lib.rs:1202:25
[01:00:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-freebsd --target x86_64-unknown-freebsd
[01:00:52] Build completed unsuccessfully in 0:57:50
travis_time:end:23fadbb4:start=1530571675165482599,finish=1530575328137977874,duration=3652972495275

---
travis_time:end:0a706b54:start=1530575328637018483,finish=1530575328642793800,duration=5775317
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:001e815d
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06416808
$ dmesg | grep -i kill
