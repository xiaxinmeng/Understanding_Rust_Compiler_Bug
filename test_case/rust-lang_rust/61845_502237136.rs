plain
travis_time:end:02b22433:start=1560540217176911629,finish=1560540218140250375,duration=963338746
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:19]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:23]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:23]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:49] error[E0599]: no method named `lock` found for type `rustc_data_structures::sharded::Sharded<std::vec::Vec<dep_graph::graph::DepNodeData>>` in the current scope
[00:08:49]     --> src/librustc/dep_graph/graph.rs:1159:58
[00:08:49]      |
[00:08:49] 1159 |                             let data = self.current.data.lock();
[00:08:49] 
[00:09:07] error: aborting due to previous error
[00:09:07] 
[00:09:07] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:01306e58:start=1560540777840848403,finish=1560540777845541415,duration=4693012
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:07302b98
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0344e214
travis_time:start:0344e214
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2a913724
$ dmesg | grep -i kill
