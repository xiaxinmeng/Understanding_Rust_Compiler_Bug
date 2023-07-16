plain
travis_time:end:2be6318c:start=1549205987189677815,finish=1549206059916418023,duration=72726740208
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:59:08]     Checking getopts v0.2.17
[00:59:08]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[00:59:08]     Checking term v0.0.0 (/checkout/src/libterm)
[00:59:12]  Documenting test v0.0.0 (/checkout/src/libtest)
[00:59:13] error: internal compiler error: src/librustc/hir/def.rs:257: attempted .def_id() on invalid def: NonMacroAttr(Builtin)
[00:59:13] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:605:9
[00:59:13] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[00:59:13] 
[00:59:13] error: Unrecognized option: 'crate-version'
[00:59:13] error: Unrecognized option: 'crate-version'
[00:59:13] 
[00:59:13] error: Could not document `test`.
[00:59:13] 
[00:59:13] Caused by:
[00:59:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name test src/libtest/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/release/deps --extern getopts=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libgetopts-2cd31f015e79e3a8.rmeta --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libproc_macro-9b2503eb72968b11.rmeta --extern term=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-test/x86_64-unknown-linux-gnu/release/deps/libterm-0bd89f567a02c34d.rmeta` (exit code: 1)
[00:59:13] 
[00:59:13] 
[00:59:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--no-deps" "-p" "test"
[00:59:13] 
[00:59:13] 
[00:59:13] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap doc
[00:59:13] Build completed unsuccessfully in 0:06:24
[00:59:13] Build completed unsuccessfully in 0:06:24
[00:59:13] make: *** [all] Error 1
[00:59:13] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0628093c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 16:00:22 UTC 2019
---
travis_time:end:0cf22440:start=1549209623510304227,finish=1549209623516193620,duration=5889393
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e051c78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:144246cf
travis_time:start:144246cf
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0c87c4b9
$ dmesg | grep -i kill
