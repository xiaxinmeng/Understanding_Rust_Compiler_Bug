plain
travis_time:end:204b1b28:start=1549988529303668691,finish=1549988614641606610,duration=85337937919
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:59]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:07]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:12:30]    Compiling rustc_mir v0.0.0 (/checkout/src/librustc_mir)
[00:12:30]    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
[00:12:31] error: description for error code E0375 contains a line longer than 80 characters.
[00:12:31] if you're inserting a long URL use the footnote style to bypass this check.
[00:12:31]     --> src/librustc_typeck/diagnostics.rs:4:1
[00:12:31]      |
[00:12:31] 4    | / register_long_diagnostics! {
[00:12:31] 5    | |
[00:12:31] 6    | | E0023: r##"
[00:12:31] 7    | | A pattern used to match against an enum variant must provide a sub-pattern for
[00:12:31] 4642 | |
[00:12:31] 4643 | | }
[00:12:31]      | |_^
[00:12:31]      |
[00:12:31]      |
[00:12:31]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:12:31] 
[00:12:31] error: description for error code E0604 contains a line longer than 80 characters.
[00:12:31] if you're inserting a long URL use the footnote style to bypass this check.
[00:12:31]     --> src/librustc_typeck/diagnostics.rs:4:1
[00:12:31]      |
[00:12:31] 4    | / register_long_diagnostics! {
[00:12:31] 5    | |
[00:12:31] 6    | | E0023: r##"
[00:12:31] 7    | | A pattern used to match against an enum variant must provide a sub-pattern for
[00:12:31] 4642 | |
[00:12:31] 4643 | | }
[00:12:31]      | |_^
[00:12:31]      |
[00:12:31]      |
[00:12:31]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:12:31] 
[00:12:31] error: description for error code E0605 contains a line longer than 80 characters.
[00:12:31] if you're inserting a long URL use the footnote style to bypass this check.
[00:12:31]     --> src/librustc_typeck/diagnostics.rs:4:1
[00:12:31]      |
[00:12:31] 4    | / register_long_diagnostics! {
[00:12:31] 5    | |
[00:12:31] 6    | | E0023: r##"
[00:12:31] 7    | | A pattern used to match against an enum variant must provide a sub-pattern for
[00:12:31] 4642 | |
[00:12:31] 4643 | | }
[00:12:31]      | |_^
[00:12:31]      |
[00:12:31]      |
[00:12:31]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:12:31] 
[00:12:31] error: description for error code E0606 contains a line longer than 80 characters.
[00:12:31] if you're inserting a long URL use the footnote style to bypass this check.
[00:12:31]     --> src/librustc_typeck/diagnostics.rs:4:1
[00:12:31]      |
[00:12:31] 4    | / register_long_diagnostics! {
[00:12:31] 5    | |
[00:12:31] 6    | | E0023: r##"
[00:12:31] 7    | | A pattern used to match against an enum variant must provide a sub-pattern for
[00:12:31] 4642 | |
[00:12:31] 4643 | | }
[00:12:31]      | |_^
[00:12:31]      |
[00:12:31]      |
[00:12:31]      = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:12:31] 
[00:12:31] error: description for error code E0607 contains a line longer than 80 characters.
[00:12:31] if you're inserting a long URL use the footnote style to bypass this check.
[00:12:31]     --> src/librustc_typeck/diagnostics.rs:4:1
[00:12:31]      |
[00:12:31] 4    | / register_long_diagnostics! {
[00:12:31] 5    | |
[00:12:31] 6    | | E0023: r##"
[00:12:31] 7    | | A pattern used to match against an enum variant must provide a sub-pattern for
[00:12:31] 4642 | |
[00:12:31] 4643 | | }
[00:12:31]      | |_^
[00:12:31]      |
---
[00:16:26] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:16:26] expected success, got: exit code: 101
[00:16:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:16:26] Build completed unsuccessfully in 0:12:34
[00:16:26] make: *** [all] Error 1
[00:16:26] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:017c5d0c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Feb 12 16:40:10 UTC 2019
---
travis_time:end:08022d50:start=1549989611010831226,finish=1549989611015323730,duration=4492504
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06491802
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03518594
travis_time:start:03518594
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:037bfb2b
$ dmesg | grep -i kill
