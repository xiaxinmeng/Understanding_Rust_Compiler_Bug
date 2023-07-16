plain
travis_time:end:0c9d5786:start=1553589037489687452,finish=1553589138919595391,duration=101429907939
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv6m-none-eabi)
[01:03:40] 
[01:03:40] running 12 tests
[01:03:41] iiiiiiiii...
[01:03:41] 
[01:03:41]  finished in 0.454
[01:03:41] travis_fold:end:test_run-make

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv7m-none-eabi)
[01:04:15] 
[01:04:15] running 12 tests
[01:04:16] iiiiiiiii...
[01:04:16] 
[01:04:16]  finished in 0.433
[01:04:16] travis_fold:end:test_run-make

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv7em-none-eabi)
[01:04:52] 
[01:04:52] running 12 tests
[01:04:52] iiiiiiiii...
[01:04:52] 
[01:04:52]  finished in 0.509
[01:04:52] travis_fold:end:test_run-make

---
travis_time:start:test_run-make
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv7em-none-eabihf)
[01:05:28] 
[01:05:28] running 12 tests
[01:05:28] iiiiiiiii...
[01:05:28] 
[01:05:28]  finished in 0.435
[01:05:28] travis_fold:end:test_run-make

---
[01:07:40]    Compiling compiler_builtins v0.1.5
[01:07:40]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[01:07:40]    Compiling backtrace-sys v0.1.27
[01:07:40]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:07:42] error[E0472]: asm! is unsupported on this target
[01:07:42]    --> src/libcore/hint.rs:105:18
[01:07:42]     |
[01:07:42] 105 |         unsafe { asm!("" : : "r"(&dummy)) }
[01:07:42] 
[01:07:54] error: aborting due to previous error
[01:07:54] 
[01:07:54] For more information about this error, try `rustc --explain E0472`.
---
travis_time:end:026fe8ab:start=1553593223681803487,finish=1553593223688177862,duration=6374375
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03e8c400
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:092a0b5e
travis_time:start:092a0b5e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13183405
$ dmesg | grep -i kill
