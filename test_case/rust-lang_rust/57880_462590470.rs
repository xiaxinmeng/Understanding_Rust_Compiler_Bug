plain
travis_time:end:000b46d4:start=1549932841453215883,finish=1549932843084944192,duration=1631728309
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
    73% |███████████████████████▍        | 51kB 45.7MB/s eta 0:00:01
    87% |████████████████████████████    | 61kB 45.8MB/s eta 0:00:01
    100% |████████████████████████████████| 71kB 26.2MB/s 
Collecting botocore==1.12.92 (from awscli)
  Downloading https://files.pythonhosted.org/packages/a6/ec/e68d5d9b5eaa53d3552de0638231a8678c327737f4fc9fa62733483260fc/botocore-1.12.92-py2.py3-none-any.whl (5.3MB)
    0% |▏                               | 20kB 23.7MB/s eta 0:00:01
    0% |▏                               | 30kB 28.6MB/s eta 0:00:01
    0% |▎                               | 40kB 30.8MB/s eta 0:00:01
    0% |▎                               | 51kB 31.8MB/s eta 0:00:01
---
[00:04:37]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:04:37]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:04:38]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:52]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:04:57] error: unused import: `rustc_data_structures::cold_path`
[00:04:57]   --> src/librustc/ty/query/plumbing.rs:23:5
[00:04:57] 23 | use rustc_data_structures::cold_path;
[00:04:57]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:57]    |
[00:04:57]    = note: `-D unused-imports` implied by `-D warnings`
[00:04:57]    = note: `-D unused-imports` implied by `-D warnings`
[00:04:57] 
[00:05:16] error: aborting due to previous error
[00:05:16] 
[00:05:16] error: Could not compile `rustc`.
[00:05:16] 
[00:05:16] To learn more, run the command again with --verbose.
[00:05:16] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:16] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:05:16] Build completed unsuccessfully in 0:03:12
travis_time:end:0df3fdb5:start=1549932854158230584,finish=1549933171447112833,duration=317288882249
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:10295d09:start=1549933172281680011,finish=1549933172288237714,duration=6557703
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:3664f439
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:084f7998
travis_time:start:084f7998
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0932a9de
$ dmesg | grep -i kill
