plain
travis_time:end:195af161:start=1544153882770979344,finish=1544153937311882841,duration=54540903497
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:04:11]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:04:11]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:04:12]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:26]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:04:29] error[E0432]: unresolved import `ty::query::job::QueryResult`
[00:04:29]    --> src/librustc/ty/query/plumbing.rs:799:13
[00:04:29]     |
[00:04:29] 799 |           use ty::query::job::QueryResult;
[00:04:29]     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `QueryResult` in `ty::query::job`
[00:04:29]     | 
[00:04:29]    ::: src/librustc/ty/query/mod.rs:106:1
[00:04:29]     |
[00:04:29] 106 | / define_queries! { <'tcx>
[00:04:29] 107 | |     Other {
[00:04:29] 108 | |         /// Records the type of every item.
[00:04:29] 109 | |         [] fn type_of: TypeOfItem(DefId) -> Ty<'tcx>,
[00:04:29] 698 | |     },
[00:04:29] 699 | | }
[00:04:29]     | |_- in this macro invocation
[00:04:29] 
[00:04:29] 
[00:04:51] error: aborting due to previous error
[00:04:51] 
[00:04:51] For more information about this error, try `rustc --explain E0432`.
[00:04:51] error: Could not compile `rustc`.
[00:04:51] 
[00:04:51] To learn more, run the command again with --verbose.
[00:04:51] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:04:51] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:04:51] Build completed unsuccessfully in 0:02:57
travis_time:end:188ec392:start=1544153945870553535,finish=1544154237707463516,duration=291836909981
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:094c1e0e:start=1544154238165053916,finish=1544154238172113887,duration=7059971
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:013bc7b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0829ed8c
travis_time:start:0829ed8c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:06ddb3e8
$ dmesg | grep -i kill
