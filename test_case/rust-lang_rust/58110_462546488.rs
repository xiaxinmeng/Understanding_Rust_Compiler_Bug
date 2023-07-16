plain
travis_time:end:38d132b4:start=1549926145966202930,finish=1549926148023732451,duration=2057529521
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[01:03:02]     Checking term v0.0.0 (/checkout/src/libterm)
[01:03:02]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:03:02]     Checking getopts v0.2.17
[01:03:06]  Documenting test v0.0.0 (/checkout/src/libtest)
[01:03:06] error: internal compiler error: src/librustc/hir/def.rs:259: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[01:03:06] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:595:9
[01:03:06] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:03:06] 
[01:03:06] error: Unrecognized option: 'crate-version'
[01:03:06] error: Unrecognized option: 'crate-version'
[01:03:06] 
[01:03:06] error: Could not document `test`.
[01:03:06] 
[01:03:06] Caused by:
[01:03:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --edition=2018 --crate-name test src/libtest/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[01:03:06] 
[01:03:06] 
[01:03:06] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[01:03:06] 
[01:03:06] 
[01:03:06] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[01:03:06] Build completed unsuccessfully in 0:06:00
[01:03:06] Build completed unsuccessfully in 0:06:00
[01:03:06] make: *** [all] Error 1
[01:03:06] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08aef890
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 00:05:47 UTC 2019
---
travis_time:end:173befb1:start=1549929948354846608,finish=1549929948360079636,duration=5233028
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:023b829a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:143f01af
travis_time:start:143f01af
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:030a2401
$ dmesg | grep -i kill
