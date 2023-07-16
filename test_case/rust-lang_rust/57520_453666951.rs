plain
travis_time:end:2fc0438d:start=1547240863930085381,finish=1547240866986974954,duration=3056889573
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:55]    Compiling syn v0.15.22
[00:02:57]    Compiling serde_json v1.0.33
[00:03:16]    Compiling serde_derive v1.0.81
[00:03:33]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:33] error: expected one of `:`, `;`, `=`, or `@`, found `==`
[00:03:33]   --> src/tools/tidy/src/style.rs:96:22
[00:03:33]    |
[00:03:33] 96 |     let copyright_re == Regex::new(r"^(// )? Copyright .+ The Rust Developers").unwrap();
[00:03:33]    |                      ^^ expected one of `:`, `;`, `=`, or `@` here
[00:03:33] error: unused import: `std::fs::File`
[00:03:33]   --> src/tools/tidy/src/style.rs:15:5
[00:03:33]    |
[00:03:33] 15 | use std::fs::File;
---
[00:03:33]    |         ^^^^^^^^^^^^
[00:03:33] 
[00:03:33] error: aborting due to 4 previous errors
[00:03:33] 
[00:03:33] error: Could not compile `tidy`.
[00:03:33] To learn more, run the command again with --verbose.
[00:03:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:33] expected success, got: exit code: 101
[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:33] Build completed unsuccessfully in 0:00:55
[00:03:33] make: *** [tidy] Error 1
[00:03:33] Makefile:69: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:08971234
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 11 21:11:30 UTC 2019
---
travis_time:end:0204fab0:start=1547241091613143848,finish=1547241091617665556,duration=4521708
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06a8a4f4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09c39d5b
travis_time:start:09c39d5b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1353f6d2
$ dmesg | grep -i kill
