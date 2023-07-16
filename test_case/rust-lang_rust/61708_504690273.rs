plain
travis_time:end:11f421d8:start=1561223310690017083,finish=1561223313096954405,duration=2406937322
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
[01:07:33] 
[01:07:33] running 9 tests
[01:07:33] iiiiiiiii
[01:07:33] 
[01:07:33]  finished in 0.154
[01:07:33] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:49] 
[01:07:49] running 122 tests
[01:08:15] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:08:21] .i.i......iii.i.....ii
[01:08:21] 
[01:08:21]  finished in 31.331
[01:08:21] travis_fold:end:test_debuginfo

---
[01:46:53] test /checkout/src/doc/unstable-book/src/language-features/or-patterns.md - The_tracking_issue_for_this_feature_is__::Examples (line 14) ... FAILED
[01:46:53] 
[01:46:53] failures:
[01:46:53] 
[01:46:53] ---- /checkout/src/doc/unstable-book/src/language-features/or-patterns.md - The_tracking_issue_for_this_feature_is__::Examples (line 14) stdout ----
[01:46:53] error[E0658]: or_patterns syntax is experimental
[01:46:53]   --> /checkout/src/doc/unstable-book/src/language-features/or-patterns.md:24:24
[01:46:53]    |
[01:46:53] 12 |         Some(Foo::Bar(0) | Foo::Baz(0, 0)) => {
[01:46:53]    |
[01:46:53]    = note: for more information, see https://github.com/rust-lang/rust/issues/54883
[01:46:53]    = help: add #![feature(or_patterns)] to the crate attributes to enable
[01:46:53] 
---
travis_time:end:14470680:start=1561229739549867419,finish=1561229739555221119,duration=5353700
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08204886
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:126ccf12
travis_time:start:126ccf12
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:04751696
$ dmesg | grep -i kill
