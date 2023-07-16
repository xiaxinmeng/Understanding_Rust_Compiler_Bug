plain
travis_time:end:04e01f14:start=1560686139767714649,finish=1560686140851385746,duration=1083671097
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
[01:07:17] 
[01:07:17] running 9 tests
[01:07:17] iiiiiiiii
[01:07:17] 
[01:07:17]  finished in 0.152
[01:07:17] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:33] 
[01:07:33] running 122 tests
[01:07:59] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:08:04] .i.i......iii.i.....ii
[01:08:04] 
[01:08:04]  finished in 30.776
[01:08:04] travis_fold:end:test_debuginfo

---
[01:46:11] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0478 (line 7708) stdout ----
[01:46:11] warning: trait objects without an explicit `dyn` are deprecated
[01:46:11]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7713:16
[01:46:11]   |
[01:46:11] 7 |     child: Box<Wedding<'kiss> + 'SnowWhite>, // And now it's all good!
[01:46:11]   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn Wedding<'kiss> + 'SnowWhite`
[01:46:11]   = note: #[warn(bare_trait_objects)] on by default
[01:46:11] 
[01:46:11] error[E0478]: lifetime bound not satisfied
[01:46:11]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7713:5
[01:46:11]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:7713:5
[01:46:11]   |
[01:46:11] 7 |     child: Box<Wedding<'kiss> + 'SnowWhite>, // And now it's all good!
[01:46:11]   |
[01:46:11]   |
[01:46:11] note: lifetime parameter instantiated with the lifetime 'SnowWhite as defined on the struct at 5:22
[01:46:11]   |
[01:46:11]   |
[01:46:11] 5 | struct Prince<'kiss, 'SnowWhite> { // You say here that 'kiss must live
[01:46:11]   |                      ^^^^^^^^^^
[01:46:11] note: but lifetime parameter must outlive the lifetime 'kiss as defined on the struct at 5:15
[01:46:11]   |
[01:46:11]   |
[01:46:11] 5 | struct Prince<'kiss, 'SnowWhite> { // You say here that 'kiss must live
[01:46:11] 
[01:46:11] error: aborting due to previous error
[01:46:11] 
[01:46:11] For more information about this error, try `rustc --explain E0478`.
[01:46:11] For more information about this error, try `rustc --explain E0478`.
[01:46:11] Couldn't compile the test.
[01:46:11] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0700 (line 11389) stdout ----
[01:46:11] error: expected `:`, found `{`
[01:46:11]    |
[01:46:11] 10 | where 'x
[01:46:11]    |         - expected `:`
[01:46:11] 11 | {
---
travis_time:end:333001f5:start=1560692525497668438,finish=1560692525502856065,duration=5187627
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b5ac54c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08b48780
travis_time:start:08b48780
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003a59d2
$ dmesg | grep -i kill
