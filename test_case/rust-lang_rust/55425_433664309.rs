plain
travis_time:end:03ca8b70:start=1540680758297197831,finish=1540680816237660404,duration=57940462573
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:46:33] .................................................................................................... 400/4975
[00:46:36] .................................................................................................... 500/4975
[00:46:39] .....................i.............................................................................. 600/4975
[00:46:43] .................................................................................................... 700/4975
[00:46:49] ............................................F..................i...........i........................ 800/4975
[00:46:52] .................................................................................iiiii.............. 900/4975
[00:46:57] .................................................................................................... 1100/4975
[00:47:00] .................................................................................................... 1200/4975
[00:47:02] .................................................................................................... 1300/4975
[00:47:04] .................................................................................................... 1400/4975
---
[00:48:57] 
[00:48:57] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[00:48:57] 
[00:48:57] 
[00:48:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-5.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options " "--target-rustcflags" "-Crpage1-rustc/x86_64-unknown-linux-gnu/release/deps
134188 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0
134184 ./obj/build/bootstrap/debug/incremental/bootstrap-32pr67l4sa8g0/s-f64j2dcyaz-145kilc-ycmuivyxm2px
129328 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
122284 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
112168 ./obj/build/x86_64-unknown-linux-gnu/stage1-std
111072 ./src/llvm/test/CodeGen
---
72796 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot
72792 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib
72788 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib
72784 ./obj/build/x86_64-unknown-linux-gnu/stage0-sysroot/lib/rustlib/x86_64-unknown-linux-gnu
72780 ./obj/build/x86_64-unknownckout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:007216d8
travis_time:start:007216d8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
