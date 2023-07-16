plain
travis_time:end:3d357ca3:start=1555760869560415413,finish=1555760870333695020,duration=773279607
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
[01:13:08] 
[01:13:08] running 9 tests
[01:13:08] iiiiiiiii
[01:13:08] 
[01:13:08]  finished in 0.149
[01:13:08] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:24] 
[01:13:24] running 121 tests
[01:13:49] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:13:53] i.i......iii.i.....ii
[01:13:53] 
[01:13:53]  finished in 29.838
[01:13:53] travis_fold:end:test_debuginfo

---
[01:21:26]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:10:17
[01:21:27]    |
[01:21:27] 10 |     let owned1: Cow<str> = Cow::Owned(String::from("Hi, "));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:11:17
[01:21:27]    |
[01:21:27]    |
[01:21:27] 11 |     let owned2: Cow<str> = Cow::Owned(String::from("Rustaceans!"));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:12:22
[01:21:27]    |
[01:21:27]    |
[01:21:27] 12 |     let owned_empty: Cow<str> = Cow::Owned(String::new());
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:39:16
[01:21:27]    |
[01:21:27]    |
[01:21:27] 39 |     let owned: Cow<str> = Cow::Owned(String::from("Hi, "));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:40:22
[01:21:27]    |
[01:21:27]    |
[01:21:27] 40 |     let owned_empty: Cow<str> = Cow::Owned(String::new());
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:63:21
[01:21:27]    |
[01:21:27]    |
[01:21:27] 63 |     let mut owned1: Cow<str> = Cow::Owned(String::from("Hi, "));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:64:17
[01:21:27]    |
[01:21:27]    |
[01:21:27] 64 |     let owned2: Cow<str> = Cow::Owned(String::from("Rustaceans!"));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/cow_str.rs:65:22
[01:21:27]    |
[01:21:27]    |
[01:21:27] 65 |     let owned_empty: Cow<str> = Cow::Owned(String::new());
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]    --> src/liballoc/../liballoc/tests/cow_str.rs:104:20
[01:21:27]     |
[01:21:27]     |
[01:21:27] 104 |     let mut owned: Cow<str> = Cow::Owned(String::from("Hi, "));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]    --> src/liballoc/../liballoc/tests/cow_str.rs:105:22
[01:21:27]     |
[01:21:27]     |
[01:21:27] 105 |     let owned_empty: Cow<str> = Cow::Owned(String::new());
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]    --> src/liballoc/../liballoc/tests/cow_str.rs:135:17
[01:21:27]     |
[01:21:27]     |
[01:21:27] 135 |     let mut c1: Cow<str> = Cow::Owned(String::with_capacity(25));
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]    --> src/liballoc/../liballoc/tests/cow_str.rs:138:13
[01:21:27]     |
[01:21:27]     |
[01:21:27] 138 |     let c2: Cow<str> = Cow::Owned(s);
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/string.rs:57:13
[01:21:27]    |
[01:21:27]    |
[01:21:27] 57 |     let ys: Cow<str> = "hello".into_cow();
[01:21:27] 
[01:21:27] error: hidden lifetime parameters in types are deprecated
[01:21:27]   --> src/liballoc/../liballoc/tests/string.rs:61:13
[01:21:27]    |
[01:21:27]    |
[01:21:27] 61 |     let ys: Cow<str> = "ศไทย中华Việt Nam".into_cow();
[01:21:27] 
[01:21:32] error: aborting due to 14 previous errors
[01:21:32] 
[01:21:32] error: Could not compile `alloc`.
[01:21:32] error: Could not compile `alloc`.
[01:21:32] warning: build failed, waiting for other jobs to finish...
[01:21:39] error: build failed
[01:21:39] 
[01:21:39] 
[01:21:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "alloc" "--" "--quiet"
[01:21:39] 
[01:21:39] 
[01:21:39] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:21:39] Build completed unsuccessfully in 0:20:00
[01:21:39] Build completed unsuccessfully in 0:20:00
[01:21:39] make: *** [check] Error 1
[01:21:39] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1a515e18
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 13:09:41 UTC 2019
---
travis_time:end:18f137aa:start=1555765783512906904,finish=1555765783519809476,duration=6902572
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:146e8d0e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:078d4d37
travis_time:start:078d4d37
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10cb4e4e
$ dmesg | grep -i kill
