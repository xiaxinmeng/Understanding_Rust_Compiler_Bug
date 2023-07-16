plain
travis_time:end:04b34f30:start=1551549365553964742,finish=1551549366551369162,duration=997404420
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
###############################################################           88.7%
######################################################################## 100.0%
[00:01:49] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:49]     Updating crates.io index
[00:02:04]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:04] error: no matching package named `libtest` found
[00:02:04] location searched: https://github.com/gnzlbg/libtest
[00:02:04] did you mean: test
[00:02:04] required by package `test v0.0.0 (/checkout/src/libtest)`
[00:02:04] Build completed unsuccessfully in 0:00:40
[00:02:04] make: *** [prepare] Error 1
[00:02:04] Makefile:70: recipe for target 'prepare' failed
[00:02:05] Command failed. Attempt 2/5:
[00:02:05] Command failed. Attempt 2/5:
[00:02:06]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:06] error: no matching package named `libtest` found
[00:02:06] location searched: https://github.com/gnzlbg/libtest
[00:02:06] did you mean: test
[00:02:06] required by package `test v0.0.0 (/checkout/src/libtest)`
[00:02:06] Build completed unsuccessfully in 0:00:00
[00:02:06] make: *** [prepare] Error 1
[00:02:06] Makefile:70: recipe for target 'prepare' failed
[00:02:08] Command failed. Attempt 3/5:
[00:02:08] Command failed. Attempt 3/5:
[00:02:08]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:08] error: no matching package named `libtest` found
[00:02:08] location searched: https://github.com/gnzlbg/libtest
[00:02:08] did you mean: test
[00:02:08] required by package `test v0.0.0 (/checkout/src/libtest)`
[00:02:08] Build completed unsuccessfully in 0:00:00
[00:02:08] Makefile:70: recipe for target 'prepare' failed
[00:02:08] make: *** [prepare] Error 1
[00:02:11] Command failed. Attempt 4/5:
[00:02:11] Command failed. Attempt 4/5:
[00:02:11]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:12] error: no matching package named `libtest` found
[00:02:12] location searched: https://github.com/gnzlbg/libtest
[00:02:12] did you mean: test
[00:02:12] required by package `test v0.0.0 (/checkout/src/libtest)`
[00:02:12] Build completed unsuccessfully in 0:00:00
[00:02:12] Makefile:70: recipe for target 'prepare' failed
[00:02:12] make: *** [prepare] Error 1
[00:02:16] Command failed. Attempt 5/5:
[00:02:16] Command failed. Attempt 5/5:
[00:02:16]     Updating git repository `https://github.com/gnzlbg/libtest`
[00:02:16] error: no matching package named `libtest` found
[00:02:16] location searched: https://github.com/gnzlbg/libtest
[00:02:16] did you mean: test
[00:02:16] required by package `test v0.0.0 (/checkout/src/libtest)`
[00:02:16] Build completed unsuccessfully in 0:00:00
[00:02:16] make: *** [prepare] Error 1
[00:02:16] Makefile:70: recipe for target 'prepare' failed
[00:02:16] The command has failed after 5 attempts.
---
travis_time:end:29a1246e:start=1551549515533334955,finish=1551549515539611803,duration=6276848
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:097e3330
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0786da8d
travis_time:start:0786da8d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16077abd
$ dmesg | grep -i kill
