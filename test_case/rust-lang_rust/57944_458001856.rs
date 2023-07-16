plain
travis_time:end:050da86c:start=1548652237282713225,finish=1548652338972625734,duration=101689912509
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:18] expected success, got: exit code: 101
[00:06:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:18] Build completed unsuccessfully in 0:02:25
[00:06:18] make: *** [all] Error 1
[00:06:18] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:219da02e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 28 05:18:46 UTC 2019
---
199364 ./obj/build/cache/2019-01-18
156148 ./src/llvm-project/clang
154524 ./obj/build/bootstrap/debug/incremental
139756 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e
139752 ./obj/build/bootstrap/debug/incremental/bootstrap-2ahv8almm435e/s-f8y94qxdo8-1he1yav-2gaplnmorxs2x
108528 ./src/llvm-project/lldb
97552 ./src/llvm-project/clang/test
89964 ./src/llvm-emscripten/test/CodeGen
77776 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
---
travis_time:end:02bfa6a6:start=1548652726990423493,finish=1548652726995359165,duration=4935672
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:20e40a3a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25b4fe00
travis_time:start:25b4fe00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:023ad098
$ dmesg | grep -i kill
