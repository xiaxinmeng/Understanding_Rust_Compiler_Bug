plain
travis_time:end:1dfc8b41:start=1553255332022655938,finish=1553255412763729794,duration=80741073856
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:07] 
[01:25:07] running 9 tests
[01:25:07] iiiiiiiii
[01:25:07] 
[01:25:07]  finished in 0.156
[01:25:07] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:25:23] 
[01:25:23] running 120 tests
[01:25:49] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:25:54] .i......iii.i.....ii
[01:25:54] 
[01:25:54]  finished in 30.859
[01:25:54] travis_fold:end:test_debuginfo

---
[01:40:53] .................................................................................................... 1600/2300
[01:41:07] .................................................................................................... 1700/2300
[01:41:21] .................................................................................................... 1800/2300
[01:41:36] .................................................................................................... 1900/2300
[01:41:50] .....................................................F.............................................. 2000/2300
[01:42:06] ..................................................F................................................. 2100/2300
[01:42:40] .......i............................................................................................ 2300/2300
[01:42:40] 
[01:42:40] failures:
[01:42:40] 
[01:42:40] 
[01:42:40] ---- option.rs - option::Option<V>::from_iter (line 1321) stdout ----
[01:42:40] error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `;`
[01:42:40]   --> option.rs:1328:25
[01:42:40]    |
[01:42:40] 10 |     .map(|x| shared += x; x.checked_sub(2))
[01:42:40]    |                         ^ expected one of 8 possible tokens here
[01:42:40] error[E0425]: cannot find value `x` in this scope
[01:42:40]   --> option.rs:1328:27
[01:42:40]    |
[01:42:40]    |
[01:42:40] 10 |     .map(|x| shared += x; x.checked_sub(2))
[01:42:40]    |                           ^ not found in this scope
[01:42:40] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[01:42:40]   --> option.rs:1328:6
[01:42:40]    |
[01:42:40]    |
[01:42:40] 10 |     .map(|x| shared += x; x.checked_sub(2))
[01:42:40] 
[01:42:40] error: aborting due to 3 previous errors
[01:42:40] 
[01:42:40] Some errors occurred: E0061, E0425.
[01:42:40] Some errors occurred: E0061, E0425.
[01:42:40] For more information about an error, try `rustc --explain E0061`.
[01:42:40] thread 'option.rs - option::Option<V>::from_iter (line 1321)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:310:13
[01:42:40] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:42:40] 
[01:42:40] ---- result.rs - result::Result<V, E>::from_iter (line 1220) stdout ----
[01:42:40] error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `;`
[01:42:40]   |
[01:42:40] 7 |     shared += x;
[01:42:40]   |                ^ expected one of 8 possible tokens here
[01:42:40] 
[01:42:40] 
[01:42:40] error[E0425]: cannot find value `x` in this scope
[01:42:40]  --> result.rs:1225:5
[01:42:40]   |
[01:42:40] 8 |     x.checked_sub(2).ok_or("Underflow!")
[01:42:40]   |     ^ help: a local variable with a similar name exists: `v`
[01:42:40] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[01:42:40]  --> result.rs:1223:52
[01:42:40]   |
[01:42:40]   |
[01:42:40] 6 | let res: Result<Vec<u32>, &'static str> = v.iter().map(|x: &u32|
[01:42:40] 
[01:42:40] error: aborting due to 3 previous errors
[01:42:40] 
[01:42:40] Some errors occurred: E0061, E0425.
---
[01:42:40] 
[01:42:40] error: test failed, to rerun pass '--doc'
[01:42:40] 
[01:42:40] 
[01:42:40] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:42:40] 
[01:42:40] 
[01:42:40] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:42:40] Build completed unsuccessfully in 0:29:33
[01:42:40] Build completed unsuccessfully in 0:29:33
[01:42:40] make: *** [check] Error 1
[01:42:40] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08c35b2c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 22 13:33:01 UTC 2019
---
travis_time:end:19fe9dfe:start=1553261583421882948,finish=1553261583427197903,duration=5314955
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:129e781a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06060120
travis_time:start:06060120
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:113dd558
$ dmesg | grep -i kill
