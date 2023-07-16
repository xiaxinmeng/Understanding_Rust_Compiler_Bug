plain
travis_time:end:02af8f28:start=1561155629675043354,finish=1561155632237822597,duration=2562779243
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:41]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:45]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:45]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:57]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:11] error[E0599]: no method named `is_none` found for type `dep_graph::graph::DepNodeState` in the current scope
[00:09:11]    --> src/librustc/dep_graph/graph.rs:792:59
[00:09:11] 59  | pub enum DepNodeState {
[00:09:11]     | --------------------- method `is_none` not found for this
[00:09:11] ...
[00:09:11] ...
[00:09:11] 792 |             debug_assert!(data.colors.get(dep_node_index).is_none());
[00:09:11] 
[00:09:29] error: aborting due to previous error
[00:09:29] 
[00:09:29] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:04bf8e98:start=1561156214333698866,finish=1561156214338742184,duration=5043318
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:15e54686
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09b9d842
travis_time:start:09b9d842
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07039368
$ dmesg | grep -i kill
