plain
[00:01:16] 
################################                                          45.8%
######################################################################## 100.0%
[00:01:16] extracting /checkout/obj/build/cache/2018-07-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:16]     Updating git repository `https://github.com/kennytm/racer`
[00:01:17]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:39]     Updating git repository `https://github.com/nrc/rls-vfs`
[00:01:41]  Downloading num_cpus v1.8.0
[00:01:41]  Downloading toml v0.4.6
[00:01:41]  Downloading petgraph v0.4.12
[00:01:41]  Downloading lazy_static v0.2.11
---
tidy check
[00:03:57] * 553 error codes
[00:03:57] * highest error code: E0710
[00:03:57] * 211 features
[00:03:58] invalid source: "git+https://github.com/kennytm/racer?rev=5f711af0dfc7f4c2bf4b2e670429c5a61c43f8d3#5f711af0dfc7f4c2bf4b2e670429c5a61c43f8d3"
[00:03:58] invalid source: "git+https://github.com/nrc/rls-vfs#f9ec4074af231c9f9e2f1f89772c05caf2a744b0"
[00:03:58] some tidy checks failed
[00:03:58] 
[00:03:58] 
[00:03:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:03:58] 
[00:03:58] 
[00:03:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:58] Build completed unsuccessfully in 0:00:51
[00:03:58] Build completed unsuccessfully in 0:00:51
[00:03:58] make: *** [tidy] Error 1
[00:03:58] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:011acd4c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:177a0160:start=1532660662101193651,finish=1532660662108806878,duration=7613227
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:009c76a0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04e6fd06
travis_time:start:04e6fd06
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00cf671e
$ dmesg | grep -i kill
