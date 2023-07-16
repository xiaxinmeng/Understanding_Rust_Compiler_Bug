plain
travis_time:end:37bc24e2:start=1547412611150068669,finish=1547412688602059107,duration=77451990438
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:26:03]    Compiling ena v0.11.0
[00:26:03]    Compiling rustc_cratesio_shim v0.0.0 (/checkout/src/librustc_cratesio_shim)
[00:26:03]    Compiling rls-span v0.4.0
[00:26:04]    Compiling smallvec v0.6.7
[00:26:04] error[E0730]: unions may not contain fields that need dropping
[00:26:04]     |
[00:26:04] 273 |     inline: A,
[00:26:04]     |     ^^^^^^^^^
[00:26:04]     |
---
[00:26:05] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:26:05] expected success, got: exit code: 101
[00:26:05] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:26:05] Build completed unsuccessfully in 0:22:54
[00:26:05] Makefile:18: recipe for target 'all' failed
[00:26:05] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:15cb93d3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 13 21:17:41 UTC 2019
---
travis_time:end:006cb644:start=1547414262445868884,finish=1547414262450579986,duration=4711102
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07808bce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:306fbb04
travis_time:start:306fbb04
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1f66c0b4
$ dmesg | grep -i kill
