plain
travis_time:end:1d47cbf6:start=1544187673569784372,finish=1544187675898480269,duration=2328695897
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:04:28]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:04:29]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:04:30]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:45]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:04:47] error[E0432]: unresolved import `ty::query::job::QueryResult`
[00:04:47]    --> src/librustc/ty/query/plumbing.rs:816:13
[00:04:47]     |
[00:04:47] 816 |           use ty::query::job::QueryResult;
[00:04:47]     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `QueryResult` in `ty::query::job`
[00:04:47]     | 
[00:04:47]    ::: src/librustc/ty/query/mod.rs:106:1
[00:04:47]     |
[00:04:47] 106 | / define_queries! { <'tcx>
[00:04:47] 107 | |     Other {
[00:04:47] 108 | |         /// Records the type of every item.
[00:04:47] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:04:47] 698 | |     },
[00:04:47] 699 | | }
[00:04:47]     | |_- in this macro invocation
[00:04:47] 
[00:04:47] 
[00:05:11] error: aborting due to previous error
[00:05:11] 
[00:05:11] For more information about this error, try `rustc --explain E0432`.
[00:05:11] error: Could not compile `rustc`.
[00:05:11] 
[00:05:11] To learn more, run the command again with --verbose.
[00:05:11] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:11] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:05:11] Build completed unsuccessfully in 0:03:07
travis_time:end:02fb5403:start=1544187684949644919,finish=1544187996855895432,duration=311906250513
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:1177f928:start=1544187997318949468,finish=1544187997326050627,duration=7101159
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:021450ed
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00f7f57e
travis_time:start:00f7f57e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00eef080
$ dmesg | grep -i kill
