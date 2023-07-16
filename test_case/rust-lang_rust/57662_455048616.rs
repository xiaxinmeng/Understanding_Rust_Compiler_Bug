plain
travis_time:end:032b207e:start=1547702093936394375,finish=1547702096142544476,duration=2206150101
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:53]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:18:53] error: unused import: `Mac`
[00:18:53]  --> src/librustc_allocator/expand.rs:6:9
[00:18:53]   |
[00:18:53] 6 |         Mac, Mod, Mutability, Ty, TyKind, Unsafety, VisibilityKind, NodeId,
[00:18:53]   |
[00:18:53]   = note: `-D unused-imports` implied by `-D warnings`
[00:18:53] 
[00:18:53] error: aborting due to previous error
---
[00:20:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:20:36] expected success, got: exit code: 101
[00:20:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:20:36] Build completed unsuccessfully in 0:17:05
[00:20:36] make: *** [all] Error 1
[00:20:36] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001b03ca
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 17 05:35:43 UTC 2019
---
travis_time:end:03e54fb0:start=1547703344471008513,finish=1547703344476549427,duration=5540914
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2b04016a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10478575
travis_time:start:10478575
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3019462a
$ dmesg | grep -i kill
