plain
travis_time:end:06f3541a:start=1544709911813303195,finish=1544709914339528575,duration=2526225380
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:22:11]    Compiling rustc-demangle v0.1.9
[00:22:18]    Compiling memmap v0.6.2
[00:22:18]    Compiling num_cpus v1.8.0
[00:22:19]    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
[00:22:39] error: expected item, found `>>`
[00:22:39]    |
[00:22:39]    |
[00:22:39] 19 | >>>>>>> cd9b37c1721... Implement optimize(size) and optimize(speed)
[00:22:39]    | ^^ expected item
[00:22:39] error: aborting due to previous error
[00:22:39] 
[00:22:39] error: Could not compile `rustc_codegen_llvm`.
[00:22:39] 
[00:22:39] 
[00:22:39] To learn more, run the command again with --verbose.
[00:22:39] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" "" "--message-format" "json"
[00:22:39] expected success, got: exit code: 101
[00:22:39] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:22:39] Build completed unsuccessfully in 0:16:31
[00:22:39] make: *** [all] Error 1
[00:22:39] Makefile:28: recipe for target 'all' failed
73052 ./.git/modules
73048 ./.git/modules/src
69912 ./src/llvm-emscripten/lib
68396 ./src/test
---
travis_time:end:316144da:start=1544711284370007834,finish=1544711284377851121,duration=7843287
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:118f7de1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:019f1c98
travis_time:start:019f1c98
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_
