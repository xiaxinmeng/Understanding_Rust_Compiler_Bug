plain
[00:02:30] Successfully tagged rust-ci:latest
[00:02:30] Built container sha256:7f08d3ca0feb4f66945bf3db784259c2d637561f1d6512bfc8d1575fc70da42b
[00:02:30] Uploading finished image to s3://rust-lang-ci-sccache2/docker/254673b822b84233b3d8510170c1c331460198676199202c3bf8d1c7c9e0e1b16875995a19ea0d0d82a3789a612f21c840d7e6245ccdc54916e891d31de24244
[00:02:30] 
[00:02:30] Partial credentials found in env, missing: AWS_SECRET_ACCESS_KEY
[00:02:37] xargs: docker: terminated by signal 13

[00:02:38] travis_time:end:0907dfe8:start=1532651011057770338,finish=1532651157687976722,duration=146630206384
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:02:38] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:02:52] 
#####################                                                     30.3%
######################################################################## 100.0%
[00:02:53] extracting /checkout/obj/build/cache/2018-07-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:02:53] error: package `/checkout/src/tools/rls/test_data/bin_lib/Cargo.toml` is a member of the wrong workspace
[00:02:53] expected: /checkout/src/Cargo.toml
[00:02:53] actual:   /checkout/src/tools/rls/test_data/Cargo.toml
[00:02:53] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:53] make: *** [prepare] Error 1
[00:02:53] Makefile:81: recipe for target 'prepare' failed
[00:02:54] Command failed. Attempt 2/5:
[00:02:54] Command failed. Attempt 2/5:
[00:02:54] error: package `/checkout/src/tools/rls/test_data/bin_lib/Cargo.toml` is a member of the wrong workspace
[00:02:54] expected: /checkout/src/Cargo.toml
[00:02:54] actual:   /checkout/src/tools/rls/test_data/Cargo.toml
[00:02:54] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:54] Makefile:81: recipe for target 'prepare' failed
[00:02:54] make: *** [prepare] Error 1
[00:02:56] Command failed. Attempt 3/5:
[00:02:56] Command failed. Attempt 3/5:
[00:02:56] error: package `/checkout/src/tools/rls/test_data/bin_lib/Cargo.toml` is a member of the wrong workspace
[00:02:56] expected: /checkout/src/Cargo.toml
[00:02:56] actual:   /checkout/src/tools/rls/test_data/Cargo.toml
[00:02:56] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:56] make: *** [prepare] Error 1
[00:02:56] Makefile:81: recipe for target 'prepare' failed
[00:02:59] Command failed. Attempt 4/5:
[00:02:59] Command failed. Attempt 4/5:
[00:02:59] error: package `/checkout/src/tools/rls/test_data/bin_lib/Cargo.toml` is a member of the wrong workspace
[00:02:59] expected: /checkout/src/Cargo.toml
[00:02:59] actual:   /checkout/src/tools/rls/test_data/Cargo.toml
[00:02:59] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:59] Makefile:81: recipe for target 'prepare' failed
[00:02:59] make: *** [prepare] Error 1
[00:03:03] Command failed. Attempt 5/5:
[00:03:03] Command failed. Attempt 5/5:
[00:03:03] error: package `/checkout/src/tools/rls/test_data/bin_lib/Cargo.toml` is a member of the wrong workspace
[00:03:03] expected: /checkout/src/Cargo.toml
[00:03:03] actual:   /checkout/src/tools/rls/test_data/Cargo.toml
[00:03:03] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:03:03] Makefile:81: recipe for target 'prepare' failed
[00:03:03] make: *** [prepare] Error 1
[00:03:03] The command has failed after 5 attempts.
travis_time:end:17cc12e6:start=1532650999695311636,finish=1532651183409034952,duration=183713723316
---
travis_time:end:0c448274:start=1532651183662219362,finish=1532651183669341194,duration=7121832
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:14d4bd0e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:136c9056
travis_time:start:136c9056
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:335e6be0
$ dmesg | grep -i kill
