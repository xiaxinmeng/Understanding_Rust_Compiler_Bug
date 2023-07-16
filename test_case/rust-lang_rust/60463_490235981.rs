plain
travis_time:end:18865068:start=1557253256465536297,finish=1557253343487733333,duration=87022197036
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
[01:19:58] 
[01:19:58] running 9 tests
[01:19:58] iiiiiiiii
[01:19:58] 
[01:19:58]  finished in 0.153
[01:19:58] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:14] 
[01:20:14] running 122 tests
[01:20:39] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:20:43] .i.i......iii.i.....ii
[01:20:43] 
[01:20:43]  finished in 29.676
[01:20:43] travis_fold:end:test_debuginfo

---
[01:44:37] ---- /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md - transparent_enums (line 13) stdout ----
[01:44:37] error[E0658]: transparent enums are unstable
[01:44:37]  --> /checkout/src/doc/unstable-book/src/language-features/transparent-enums.md:18:1
[01:44:37]   |
[01:44:37] 6 | / enum Enum<'a, T> {
[01:44:37] 7 | |     Variant(T, std::marker::PhantomData<&'a u8>),
[01:44:37]   | |_^
[01:44:37]   |
[01:44:37]   = note: for more information, see https://github.com/rust-lang/rust/issues/60405
[01:44:37]   = help: add #![feature(transparent_enums)] to the crate attributes to enable
---
[01:44:37] 
[01:44:37] 
[01:44:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:44:37] Build completed unsuccessfully in 0:36:16
[01:44:37] make: *** [check] Error 1
[01:44:37] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:05e3e078
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue May  7 20:07:10 UTC 2019
---
travis_time:end:0179595c:start=1557259631929815434,finish=1557259631934347828,duration=4532394
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:21b950c8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00e26d6e
travis_time:start:00e26d6e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a789da2
$ dmesg | grep -i kill
