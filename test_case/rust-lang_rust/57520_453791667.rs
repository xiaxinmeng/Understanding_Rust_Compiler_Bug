plain
travis_time:end:0b775b97:start=1547336876380378443,finish=1547336972607273219,duration=96226894776
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:45]    Compiling syn v0.15.22
[00:02:56]    Compiling serde_json v1.0.33
[00:02:57]    Compiling serde_derive v1.0.81
[00:03:16]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:16] error: incorrect close delimiter: `}`
[00:03:16]    --> src/tools/tidy/src/style.rs:159:9
[00:03:16]     |
[00:03:16] 116 |         for (i, line) in contents.split('\n').enumerate() {
[00:03:16]     |                                                           - close delimiter possibly meant for this
[00:03:16] ...
[00:03:16] 144 |                                && (line.contains("Rust Developers") ||
[00:03:16]     |                                   - un-closed delimiter
[00:03:16] 159 |         }
[00:03:16]     |         ^ incorrect close delimiter
[00:03:16] 
[00:03:16] 
[00:03:16] error: expected one of `)`, `,`, `.`, `?`, or an operator, found `{`
[00:03:16]    --> src/tools/tidy/src/style.rs:145:77
[00:03:16]     |
[00:03:16] 145 |                                    line.contains("Rust Project Developers") {
[00:03:16]     |                                                                             ^ expected one of `)`, `,`, `.`, `?`, or an operator here
[00:03:17] error: aborting due to 2 previous errors
[00:03:17] 
[00:03:17] 
[00:03:17] error: Could not compile `tidy`.
[00:03:17] To learn more, run the command again with --verbose.
[00:03:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:17] expected success, got: exit code: 101
[00:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:17] Build completed unsuccessfully in 0:00:39
[00:03:17] make: *** [tidy] Error 1
[00:03:17] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b2caf00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Jan 12 23:52:58 UTC 2019
---
travis_time:end:0a7c20b3:start=1547337179473425936,finish=1547337179478285519,duration=4859583
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0a60864e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05b40ff0
travis_time:start:05b40ff0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:32774964
$ dmesg | grep -i kill
