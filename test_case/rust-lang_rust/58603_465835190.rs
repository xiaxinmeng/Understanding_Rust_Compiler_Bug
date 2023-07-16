plain
travis_time:end:132e15a0:start=1550712464301602914,finish=1550712466393211495,duration=2091608581
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:52:24] travis_time:end:stage2-rustdoc:start=1550715461313415521,finish=1550715621160360520,duration=159846944999

[00:52:24] Build completed successfully in 0:47:38
[00:52:25]     Finished dev [unoptimized] target(s) in 0.36s
[00:52:26] thread 'main' panicked at 'file not found: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:997:5
[00:52:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:52:26] Build completed unsuccessfully in 0:00:01
[00:52:26] Build completed unsuccessfully in 0:00:01
[00:52:26] make: *** [all] Error 1
[00:52:26] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2b1f84e5
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb 21 02:20:23 UTC 2019
---
150916 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
150912 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
148132 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
141340 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn
141336 ./obj/build/bootstrap/debug/incremental/bootstrap-3i6jt5nchoxcn/s-f9ojjyahph-1ucaarn-27ikm78ap3un2
138524 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
135744 ./obj/build/x86_64-unknown-linux-deGen/X86
40496 ./src/llvm-emscripten/lib/Target
39308 ./src/tools
---
travis_time:end:13957602:start=1550715624864988533,finish=1550715624869868017,duration=4879484
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:007cd64c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00fef60a
travis_time:start:00fef60a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a0b7a00
$ dmesg | grep -i kill
