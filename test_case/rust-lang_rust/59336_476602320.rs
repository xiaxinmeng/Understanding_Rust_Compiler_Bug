plain
travis_time:end:07c0b432:start=1553602128486465582,finish=1553602225779875446,duration=97293409864
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:03:05]    Compiling syn v0.15.22
[00:03:12]    Compiling serde_json v1.0.33
[00:03:18]    Compiling serde_derive v1.0.81
[00:03:37]    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
[00:03:37] error: expected one of `,`, `.`, `?`, `]`, or an operator, found `"src/libstd/sys/"`
[00:03:37]   --> src/tools/tidy/src/pal.rs:49:5
[00:03:37] 48 |     "src/libcore/hint.rs"
[00:03:37] 48 |     "src/libcore/hint.rs"
[00:03:37]    |                          - expected one of `,`, `.`, `?`, `]`, or an operator here
[00:03:37] 49 |     "src/libstd/sys/", // Platform-specific code for std lives here.
[00:03:37] 
[00:03:37] error: aborting due to previous error
[00:03:37] 
[00:03:37] 
[00:03:37] error: Could not compile `tidy`.
[00:03:37] To learn more, run the command again with --verbose.
[00:03:37] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/tidy/Cargo.toml" "--message-format" "json"
[00:03:37] expected success, got: exit code: 101
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:37] Build completed unsuccessfully in 0:00:41
[00:03:37] make: *** [tidy] Error 1
[00:03:37] Makefile:67: recipe for target 'tidy' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01161edc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Mar 26 12:14:14 UTC 2019
---
travis_time:end:1df9f5a0:start=1553602455395459005,finish=1553602455402117812,duration=6658807
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:22dbc8c6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0452ae2e
travis_time:start:0452ae2e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0dfc3ea2
$ dmesg | grep -i kill
