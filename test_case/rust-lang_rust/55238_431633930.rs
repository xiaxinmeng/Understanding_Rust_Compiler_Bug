plain
[00:03:31]    Compiling syn v0.14.9
[00:03:43]    Compiling serde_json v1.0.31
[00:03:44]    Compiling serde_derive v1.0.75
[00:04:04]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:04:04] error[E0425]: cannot find value `whitelisted` in this scope
[00:04:04]   --> tools/tidy/src/cargo.rs:87:27
[00:04:04]    |
[00:04:04] 87 |         let whitelisted = whitelisted || krate.starts_with("panic");
[00:04:04] 
[00:04:04] error: aborting due to previous error
[00:04:04] 
[00:04:04] For more information about this error, try `rustc --explain E0425`.
[00:04:04] For more information about this error, try `rustc --explain E0425`.
[00:04:04] error: Could not compile `tidy`.
[00:04:04] To learn more, run the command again with --verbose.
[00:04:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--features" "" "--message-format" "json"
[00:04:04] expected success, got: exit code: 101
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:04] Build completed unsuccessfully in 0:00:41
[00:04:04] make: *** [tidy] Error 1
[00:04:04] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0fb07fbe
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2661640e:start=1540088918894153030,finish=1540088918898374219,duration=4221189
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0fd63c58
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0fb7a3b0
travis_time:start:0fb7a3b0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:16a7b79d
$ dmesg | grep -i kill
