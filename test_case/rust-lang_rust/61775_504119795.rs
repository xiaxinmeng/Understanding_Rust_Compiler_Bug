plain
travis_time:end:12bd0af8:start=1561046711090583043,finish=1561046713432714913,duration=2342131870
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:49] 
[01:05:49] running 9 tests
[01:05:49] iiiiiiiii
[01:05:49] 
[01:05:49]  finished in 0.154
[01:05:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:05] 
[01:06:05] running 122 tests
[01:06:29] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:34] .i.i......iii.i.....ii
[01:06:34] 
[01:06:34]  finished in 29.584
[01:06:34] travis_fold:end:test_debuginfo

---
[01:41:57] ---- /checkout/src/doc/unstable-book/src/language-features/member-constraints.md - The_tracking_issue_for_this_feature_is__ (line 14) stdout ----
[01:41:57] error[E0107]: wrong number of lifetime arguments: expected 0, found 2
[01:41:57]  --> /checkout/src/doc/unstable-book/src/language-features/member-constraints.md:18:15
[01:41:57]   |
[01:41:57] 5 | impl<T> Trait<'_, '_> for T {}
[01:41:57]   |               ^^  ^^ unexpected lifetime argument
[01:41:57]   |               unexpected lifetime argument
[01:41:57] 
[01:41:57] error: aborting due to previous error
[01:41:57] 
---
travis_time:end:28ae8664:start=1561052843445757374,finish=1561052843450802299,duration=5044925
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dfcf516
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03daff65
travis_time:start:03daff65
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02ada262
$ dmesg | grep -i kill
