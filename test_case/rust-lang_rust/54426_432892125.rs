plain
travis_time:end:187675a6:start=1540434032979305349,finish=1540434093249434299,duration=60270128950
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:07:08]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:12] error[E0425]: cannot find value `place_ty` in this scope
[00:07:12]    --> librustc/mir/tcx.rs:308:27
[00:07:12]     |
[00:07:12] 308 |             let next_ty = place_ty.last().unwrap().projection_ty(tcx, elem);
[00:07:12]     |                           ^^^^^^^^ did you mean `place_tys`?
3677288 .
1407740 ./obj
1407700 ./obj/build
1197412 ./.git
---
151412 ./src/tools/clang
149628 ./obj/build/bootstrap/debug/incremental
149128 ./src/llvm-emscripten/test
134172 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134168 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f61dq0eul8-wxil4x-1wbz4rzqyzyro
107668 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
104700 ./src/tools/lldb
93748 ./src/tools/clang/test
87536 ./obj/build/x86_64-uild/x86_64-unknown-linux-gnu/native/jemalloc/src
---
travis_time:end:0b07a0a4:start=1540434562952605312,finish=1540434562958969409,duration=6364097
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:006f1fc0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:199115d6
travis_time:start:199115d6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
