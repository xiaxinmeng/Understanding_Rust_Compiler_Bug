plain
travis_time:end:01c428e0:start=1561558515366008479,finish=1561558623042673075,duration=107676664596
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---

[00:03:50] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/os/vxworks/fs.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/os/vxworks/mod.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/os/vxworks/raw.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/fs.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/mod.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/raw.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/ext/ffi.rs
[00:03:50] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/os.rs
[00:03:51] tidy error: binary checked into source: /checkout/src/libstd/sys/vxworks/process/process_vxworks.rs
[00:03:51] tidy error: /checkout/src/libstd/sys/unix/ext/fs.rs:100: mismatches the `issue` in previous
[00:03:52] some tidy checks failed
[00:03:52] 
[00:03:52] 
[00:03:52] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
[00:03:52] 
[00:03:52] 
[00:03:52] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:52] Build completed unsuccessfully in 0:01:12
---
travis_time:end:1066fe00:start=1561558865250045417,finish=1561558865254751215,duration=4705798
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c263160
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:193c9623
travis_time:start:193c9623
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1433c224
$ dmesg | grep -i kill
