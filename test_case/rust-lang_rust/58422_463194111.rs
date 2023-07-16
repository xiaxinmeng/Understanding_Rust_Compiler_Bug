plain
travis_time:end:06691ecb:start=1550058005071365345,finish=1550058097613898394,duration=92542533049
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
[01:10:32] 
[01:10:32] running 119 tests
[01:11:01] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:06] i......iii.i.....ii
[01:11:06] 
[01:11:06]  finished in 33.570
[01:11:06] travis_fold:end:test_debuginfo

---
[01:29:21] running 991 tests
[01:29:44] i................................................................................................... 100/991
[01:29:57] .................................................................................................... 200/991
[01:30:05] ..........iii......i......i...i......i.............................................................. 300/991
[01:30:10] ...............................................................................................F.F.. 400/991
[01:30:26] .................................................................................................... 600/991
[01:30:34] .................................................................................................... 700/991
[01:30:43] .............iiii................................................................................... 800/991
[01:30:56] .................................................................................................... 900/991
[01:30:56] .................................................................................................... 900/991
[01:31:04] .......................................iiii................................................
[01:31:04] failures:
[01:31:04] 
[01:31:04] ---- io/mod.rs - io::Seek::stream_len (line 1232) stdout ----
[01:31:04] error[E0658]: use of unstable library feature 'seek_convenience'
[01:31:04]   --> io/mod.rs:1241:17
[01:31:04]    |
[01:31:04] 11 |     let len = f.stream_len()?;
[01:31:04]    |
[01:31:04]    |
[01:31:04]    = help: add #![feature(seek_convenience)] to the crate attributes to enable
[01:31:04] 
[01:31:04] thread 'io/mod.rs - io::Seek::stream_len (line 1232)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:31:04] 
[01:31:04] ---- io/mod.rs - io::Seek::current_position (line 1261) stdout ----
[01:31:04] ---- io/mod.rs - io::Seek::current_position (line 1261) stdout ----
[01:31:04] error[E0658]: use of unstable library feature 'seek_convenience'
[01:31:04]   --> io/mod.rs:1270:20
[01:31:04]    |
[01:31:04] 11 |     let before = f.current_position();
[01:31:04]    |
[01:31:04]    |
[01:31:04]    = help: add #![feature(seek_convenience)] to the crate attributes to enable
[01:31:04] 
[01:31:04] error[E0658]: use of unstable library feature 'seek_convenience'
[01:31:04]   --> io/mod.rs:1272:19
[01:31:04]    |
[01:31:04] 13 |     let after = f.current_position()?;
[01:31:04]    |
[01:31:04]    |
[01:31:04]    = help: add #![feature(seek_convenience)] to the crate attributes to enable
[01:31:04] 
[01:31:04] error[E0277]: cannot subtract `std::result::Result<u64, std::io::Error>` from `u64`
[01:31:04]   --> io/mod.rs:1274:56
[01:31:04]    |
[01:31:04] 15 |     println!("The first line was {} bytes long", after - before);
[01:31:04]    |                                                        ^ no implementation for `u64 - std::result::Result<u64, std::io::Error>`
[01:31:04]    |
[01:31:04]    = help: the trait `std::ops::Sub<std::result::Result<u64, std::io::Error>>` is not implemented for `u64`
[01:31:04] thread 'io/mod.rs - io::Seek::current_position (line 1261)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:354:13
[01:31:04] 
[01:31:04] 
[01:31:04] failures:
---
[01:31:04] 
[01:31:04] error: test failed, to rerun pass '--doc'
[01:31:04] 
[01:31:04] 
[01:31:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:31:04] 
[01:31:04] 
[01:31:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:31:04] Build completed unsuccessfully in 0:32:57
[01:31:04] Build completed unsuccessfully in 0:32:57
[01:31:04] make: *** [check] Error 1
[01:31:04] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:081d3200
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 13 13:12:51 UTC 2019
---
travis_time:end:05960c8a:start=1550063573399702000,finish=1550063573405182950,duration=5480950
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:091a8e90
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$E
