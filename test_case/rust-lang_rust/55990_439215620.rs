plain
travis_time:end:0f6aa206:start=1542320052079563219,finish=1542320107816344753,duration=55736781534
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:02:23] Successfully built 529c72080102
[00:02:23] Successfully tagged rust-ci:latest
[00:02:23] Built container sha256:529c72080102726d1c13b67f5de39a245eef721454b612fa915a26dd5ab69623
[00:02:23] Uploading finished image to s3://rust-lang-ci-sccache2/docker/7a3bf0b96d607373ffdabaf23c50904e87f12308d35eb78fa95065e7c2e4b326fbb64a2d08068abf170dd6538ba472ee9b9b18fe6a5724b38bb5d93935b7c744
[00:03:04] upload failed: - to s3://rust-lang-ci-sccache2/docker/7a3bf0b96d607373ffdabaf23c50904e87f12308d35eb78fa95065e7c2e4b326fbb64a2d08068abf170dd6538ba472ee9b9b18fe6a5724b38bb5d93935b7c744 Unable to locate credentials

[00:03:04] travis_time:end:08b4b004:start=1542320195526533842,finish=1542320301884826537,duration=106358292695
[CI_JOB_NAME=x86_64-gnu-llvm-5.0]
[00:03:04] [CI_JOB_NAME=x86_64-gnu-llvm-5.0]
---
[00:20:49]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:20:51] error[E0507]: cannot move out of an `Rc`
[00:20:51]   --> librustc_lint/unused.rs:71:43
[00:20:51]    |
[00:20:51] 71 |                     for (predicate, _) in cx.tcx.predicates_of(def).predicates {
[00:20:51]    |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of an `Rc`
[00:20:52] error: aborting due to previous error
[00:20:52] 
[00:20:52] For more information about this error, try `rustc --explain E0507`.
[00:20:52] error: Could not compile `rustc_lint`.
---
travis_time:end:19a180cc:start=1542321390614795353,finish=1542321390619769393,duration=4974040
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1036a5c1
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04d278b8
travis_time:start:04d278b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:050f714c
$ dmesg | grep -i kill
