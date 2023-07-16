plain
travis_time:end:02b6b528:start=1549556847809816605,finish=1549556850752375486,duration=2942558881
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
tidy check
[00:16:57] * 565 error codes
[00:16:57] * highest error code: E0722
[00:16:57] * 249 features
[00:16:58] Stray file with UI testing output: "/checkout/src/test/ui/rust-2018/uniform-paths/issue-54390.stderr"
[00:16:58] some tidy checks failed
[00:16:58] 
[00:16:58] 
[00:16:58] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:16:58] 
[00:16:58] 
[00:16:58] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:16:58] Build completed unsuccessfully in 0:00:44
[00:16:58] Build completed unsuccessfully in 0:00:44
[00:16:58] Makefile:68: recipe for target 'tidy' failed
[00:16:58] make: *** [tidy] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:176587ce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 16:44:39 UTC 2019
---
travis_time:end:1aa696cb:start=1549557880372771267,finish=1549557880376933469,duration=4162202
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:137b2f97
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b9a890
travis_time:start:03b9a890
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:128df362
$ dmesg | grep -i kill
