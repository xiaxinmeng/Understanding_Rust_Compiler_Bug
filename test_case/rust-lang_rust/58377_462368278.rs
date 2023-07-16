plain
travis_time:end:04c4ef9c:start=1549898362386517985,finish=1549898363353221000,duration=966703015
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:21]    Compiling rustc-demangle v0.1.10
[00:06:22] error[E0412]: cannot find type `Box` in this scope
[00:06:22]    --> src/liballoc/macros.rs:44:43
[00:06:22]     |
[00:06:22] 44  |         $crate::vec::Vec::from(box tmp as Box<[_]>)
[00:06:22]     | 
[00:06:22]    ::: src/liballoc/collections/binary_heap.rs:317:28
[00:06:22]     |
[00:06:22]     |
[00:06:22] 317 |         BinaryHeap { data: vec![] }
[00:06:22] help: possible candidate is found in another module, you can import it into scope
[00:06:22]     |
[00:06:22] 148 | use crate::boxed::Box;
[00:06:22]     |
---
[00:06:24] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:06:24] expected success, got: exit code: 101
[00:06:24] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:24] Build completed unsuccessfully in 0:00:35
[00:06:24] make: *** [all] Error 1
[00:06:24] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d4031a8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Feb 11 15:25:59 UTC 2019
---
travis_time:end:1522bd5c:start=1549898760024050636,finish=1549898760028410234,duration=4359598
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:081dfcc2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:058d246a
travis_time:start:058d246a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:09368430
$ dmesg | grep -i kill
