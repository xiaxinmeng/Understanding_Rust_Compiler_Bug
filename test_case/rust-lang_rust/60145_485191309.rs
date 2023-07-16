plain
travis_time:end:1c50ce88:start=1555795217421336216,finish=1555795307023823946,duration=89602487730
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
[01:16:19] 
[01:16:19] running 9 tests
[01:16:19] iiiiiiiii
[01:16:19] 
[01:16:19]  finished in 0.156
[01:16:19] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:35] 
[01:16:35] running 121 tests
[01:17:01] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:17:06] i.i......iii.i.....ii
[01:17:06] 
[01:17:06]  finished in 30.771
[01:17:06] travis_fold:end:test_debuginfo

---
[01:36:22] .................................................................................................... 200/785
[01:36:22] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/buffered.rs:1367:17
[01:36:22] .................................................................................................... 300/785
[01:36:22] thread '<unnamed>' panicked at 'explicit panic', src/libstd/io/stdio.rs:854:13
[01:36:22] ....................................................F........F...................................... 400/785
[01:36:23] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:999:5
[01:36:23] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:999:5
[01:36:23] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "SendError(..)"', src/libcore/result.rs:999:5
[01:36:23] thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: RecvError', src/libcore/result.rs:999:5
---
[01:36:25] thread '<unnamed>' panicked at 'Box<Any>', src/libstd/thread/mod.rs:1684:13
[01:36:34] .....................................................................................
[01:36:34] failures:
[01:36:34] 
[01:36:34] ---- net::ip::tests::ip_properties stdout ----
[01:36:34] thread '<unnamed>' panicked at 'assertion failed: !ip!("fec0::").is_global()', src/libstd/net/ip.rs:2035:9
[01:36:34] ---- net::ip::tests::ipv6_properties stdout ----
[01:36:34] ---- net::ip::tests::ipv6_properties stdout ----
[01:36:34] thread '<unnamed>' panicked at 'assertion failed: !ip!("fec0::").is_global()', src/libstd/net/ip.rs:2311:9
[01:36:34] 
[01:36:34] failures:
[01:36:34]     net::ip::tests::ip_properties
[01:36:34]     net::ip::tests::ipv6_properties
[01:36:34]     net::ip::tests::ipv6_properties
[01:36:34] 
[01:36:34] test result: FAILED. 783 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
[01:36:34] 
[01:36:34] error: test failed, to rerun pass '--lib'
[01:36:34] 
[01:36:34] 
[01:36:34] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:36:34] 
[01:36:34] 
[01:36:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:34] Build completed unsuccessfully in 0:32:10
[01:36:34] Build completed unsuccessfully in 0:32:10
[01:36:34] make: *** [check] Error 1
[01:36:34] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02e0b800
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 22:58:30 UTC 2019
---
travis_time:end:057d0d48:start=1555801112270158960,finish=1555801112276065832,duration=5906872
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c4ae804
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
