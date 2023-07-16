plain
travis_time:end:007c3116:start=1548356913735356130,finish=1548356915038419954,duration=1303063824
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:05]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:06:06]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:10]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:24]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:10] error[E0502]: cannot borrow `lock` as mutable because it is also borrowed as immutable
[00:08:10]    --> src/librustc/ty/query/plumbing.rs:120:21
[00:08:10]     |
[00:08:10] 115 |             if let Some(value) = lock.results.get(key) {
[00:08:10] ...
[00:08:10] ...
[00:08:10] 120 |                     lock.cache_hits += 1;
[00:08:10] 121 |                 }
[00:08:10] 121 |                 }
[00:08:10] 122 |                 return TryGetJob::JobCompleted((value.value.clone(), value.index));
[00:08:10] 
[00:08:14] error: aborting due to previous error
[00:08:14] 
[00:08:14] For more information about this error, try `rustc --explain E0502`.
[00:08:14] For more information about this error, try `rustc --explain E0502`.
[00:08:14] error: Could not compile `rustc`.
[00:08:14] 
[00:08:14] To learn more, run the command again with --verbose.
[00:08:14] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:14] expected success, got: exit code: 101
[00:08:14] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:08:14] Build completed unsuccessfully in 0:04:07
[00:08:14] make: *** [all] Error 1
[00:08:14] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03e04b04
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Jan 24 19:17:00 UTC 2019
---
186348 ./obj/build/cache/2019-01-04
161192 ./obj/build/bootstrap/debug/incremental
153280 ./src/tools/clang
145084 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405
145080 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405/s-f8uhfakhgm-ezrgfz-2c88epco22fc8
110108 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107424 ./src/tools/lldb
95108 ./src/tools/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
---
travis_time:end:035a7d88:start=1548357421547692183,finish=1548357421553731501,duration=6039318
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1412ae60
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00b9082c
travis_time:start:00b9082c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
