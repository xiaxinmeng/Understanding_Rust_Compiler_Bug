plain
travis_time:end:0517ebe8:start=1549276030004706574,finish=1549276134317371181,duration=104312664607
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:18:10]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:18:10]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:18:12]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:18:45]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:18:46] error: expected `;`, found keyword `use`
[00:18:46]    |
[00:18:46]    |
[00:18:46] 10 | use errors::{Applicability, DiagnosticBuilder, DiagnosticId}
[00:18:46]    |                                                             - expected `;`
[00:18:46] 11 | use macros::ParentScope;
[00:18:46]    | ^^^ unexpected token
[00:18:46] error: aborting due to previous error
[00:18:46] 
[00:18:46] error: Could not compile `rustc_resolve`.
[00:18:46] warning: build failed, waiting for other jobs to finish...
[00:18:46] warning: build failed, waiting for other jobs to finish...
[00:19:10] error: build failed
[00:19:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:19:10] expected success, got: exit code: 101
[00:19:10] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:19:10] Build completed unsuccessfully in 0:15:26
[00:19:10] make: *** [all] Error 1
[00:19:10] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:228b3880
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb  4 10:48:14 UTC 2019
---
travis_time:end:15d3d406:start=1549277295111268708,finish=1549277295115679905,duration=4411197
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1133df51
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:19800dc8
travis_time:start:19800dc8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1277b6ce
$ dmesg | grep -i kill
