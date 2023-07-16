plain
travis_time:end:13fb9648:start=1550645470308470394,finish=1550645566607705667,duration=96299235273
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:38] 
[01:08:38] running 130 tests
[01:08:41] i..iii...i.ii.iiii.....i.................i..i................i.....i.........ii...i..i.ii........... 100/130
[01:08:42] test result: ok. 101 passed; 0 failed; 29 ignored; 0 measured; 0 filtered out
[01:08:42] 
[01:08:42]  finished in 3.977
[01:08:42] travis_fold:end:test_codegen
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:58] 
[01:08:58] running 119 tests
[01:09:22] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:09:27] i......iii.i.....ii
[01:09:27] 
[01:09:27]  finished in 28.345
[01:09:27] travis_fold:end:test_debuginfo

---
[01:23:57]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:24:16] error: function is never used: `print_to`
[01:24:16]    --> src/libstd/io/stdio.rs:720:1
[01:24:16]     |
[01:24:16] 720 | / fn print_to<T>(
[01:24:16] 721 | |     args: fmt::Arguments,
[01:24:16] 722 | |     local_s: &'static LocalKey<RefCell<Option<Box<dyn Write+Send>>>>,
[01:24:16] 723 | |     global_s: fn() -> T,
[01:24:16] 742 | |     }
[01:24:16] 743 | | }
[01:24:16]     | |_^
[01:24:16]     |
---
[01:24:17] 
[01:24:17] To learn more, run the command again with --verbose.
[01:24:17] 
[01:24:17] 
[01:24:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:24:17] 
[01:24:17] 
[01:24:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:17] Build completed unsuccessfully in 0:26:26
[01:24:17] Build completed unsuccessfully in 0:26:26
[01:24:17] make: *** [check] Error 1
[01:24:17] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:06cd56c0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed Feb 20 08:17:12 UTC 2019
---
travis_time:end:05241d78:start=1550650634245243828,finish=1550650634250075912,duration=4832084
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01299281
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03b56fad
travis_time:start:03b56fad
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:322a16b0
$ dmesg | grep -i kill
