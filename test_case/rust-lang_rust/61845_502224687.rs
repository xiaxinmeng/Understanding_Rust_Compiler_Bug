plain
travis_time:end:0559caac:start=1560537662320006111,finish=1560537824015264745,duration=161695258634
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:21]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:25]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:25]    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
[00:08:35]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:49] error[E0599]: no method named `borrow` found for type `dep_graph::graph::CurrentDepGraph` in the current scope
[00:08:49]    --> src/librustc/dep_graph/graph.rs:636:33
[00:08:49] 636 |                                .borrow()
[00:08:49]     |                                 ^^^^^^
[00:08:49] ...
[00:08:49] ...
[00:08:49] 965 | pub(super) struct CurrentDepGraph {
[00:08:49]     | --------------------------------- method `borrow` not found for this
[00:08:49]     = help: items from traits can only be used if the trait is in scope
[00:08:49]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:08:49]             `use std::borrow::Borrow;`
[00:08:49] 
[00:08:49] 
[00:08:49] error[E0599]: no method named `lock` found for type `rustc_data_structures::sharded::Sharded<std::vec::Vec<dep_graph::graph::DepNodeData>>` in the current scope
[00:08:49]     --> src/librustc/dep_graph/graph.rs:1159:58
[00:08:49]      |
[00:08:49] 1159 |                             let data = self.current.data.lock();
[00:08:49] 
[00:09:07] error: aborting due to 2 previous errors
[00:09:07] 
[00:09:07] For more information about this error, try `rustc --explain E0599`.
---
travis_time:end:179f1c10:start=1560538381243750175,finish=1560538381248333700,duration=4583525
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:018330ae
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:065d8e3e
travis_time:start:065d8e3e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a556485
$ dmesg | grep -i kill
