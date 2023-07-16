plain
travis_time:end:006a4677:start=1545158523582730335,finish=1545158580677800628,duration=57095070293
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:53:48] 
[00:53:48] running 119 tests
[00:54:10] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:54:14] i......iii.i.....ii
[00:54:14] 
[00:54:14]  finished in 26.712
[00:54:14] travis_fold:end:test_debuginfo

---
[01:08:52] .................................................................................................... 1600/2218
[01:09:03] .................................................................................................... 1700/2218
[01:09:15] .................................................................................................... 1800/2218
[01:09:28] .................................................................................................... 1900/2218
[01:09:40] .........F.......................................................................................... 2000/2218
[01:10:09] .................................i.......................................................i.......... 2200/2218
Tue, 18 Dec 2018 19:53:22 GMT
travis_time:end:0452facc:start=1545162802702022970,finish=1545162802744076365,duration=42053395
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2c26f2cf:start=1545162803779232971,finish=1545162803784376953,duration=5143982
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a1e742d
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15a94330
travis_time:start:15a94330
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01113078
$ dmesg | grep -i kill
