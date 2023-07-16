plain
travis_time:end:09f958b8:start=1555494408314667354,finish=1555494511775175450,duration=103460508096
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:26]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:06:32]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:36]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:36]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:51] error[E0609]: no field `data` on type `std::cell::RefMut<'_, dep_graph::graph::CurrentDepGraph>`
[00:07:51]     --> src/librustc/dep_graph/graph.rs:1068:52
[00:07:51]      |
[00:07:51] 1068 |                                 let source = graph.data[source].node;
[00:07:51] 
[00:08:09] error: aborting due to previous error
[00:08:09] 
[00:08:09] For more information about this error, try `rustc --explain E0609`.
---
travis_time:end:06637445:start=1555495011374053955,finish=1555495011378619543,duration=4565588
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f6f3d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1376641e
travis_time:start:1376641e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2fb08f06
$ dmesg | grep -i kill
