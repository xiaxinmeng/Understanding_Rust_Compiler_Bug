plain
travis_time:end:0b0459d8:start=1547519611772240881,finish=1547519709311175564,duration=97538934683
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:36]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:36]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:41]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:58]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:34] error: unused variable: `dep_node_index`
[00:07:34]   --> src/librustc/ty/query/plumbing.rs:42:23
[00:07:34]    |
[00:07:34] 42 |                       dep_node_index: DepNodeIndex)
[00:07:34]    |                       ^^^^^^^^^^^^^^ help: consider using `_dep_node_index` instead
[00:07:34]    = note: `-D unused-variables` implied by `-D warnings`
[00:07:34] 
travis_time:end:0068c4f0:start=1547519718341126408,finish=1547520173559972259,duration=455218845851
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
---
travis_time:end:09fcf3c3:start=1547520174173259214,finish=1547520174177719192,duration=4459978
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e457568
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0298bea3
travis_time:start:0298bea3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
