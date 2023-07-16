plain
travis_time:end:0ab05926:start=1548929256641362155,finish=1548929369322863662,duration=112681501507
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:12]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:12] error: expected item, found `*`
[00:02:12]  --> src/bootstrap/doc.rs:1:1
[00:02:12]   |
[00:02:12] 1 | *1;2802;0c//! Documentation generation for rustbuilder.
[00:02:12]   | ^ expected item
[00:02:12] error: aborting due to previous error
[00:02:12] 
[00:02:12] error: Could not compile `bootstrap`.
[00:02:12] 
---
[00:02:14]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:14] error: expected item, found `*`
[00:02:14]  --> src/bootstrap/doc.rs:1:1
[00:02:14]   |
[00:02:14] 1 | *1;2802;0c//! Documentation generation for rustbuilder.
[00:02:14]   | ^ expected item
[00:02:14] error: aborting due to previous error
[00:02:14] 
[00:02:14] error: Could not compile `bootstrap`.
[00:02:14] 
---
[00:02:16]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:16] error: expected item, found `*`
[00:02:16]  --> src/bootstrap/doc.rs:1:1
[00:02:16]   |
[00:02:16] 1 | *1;2802;0c//! Documentation generation for rustbuilder.
[00:02:16]   | ^ expected item
[00:02:16] error: aborting due to previous error
[00:02:16] 
[00:02:16] error: Could not compile `bootstrap`.
[00:02:16] 
---
[00:02:20]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:20] error: expected item, found `*`
[00:02:20]  --> src/bootstrap/doc.rs:1:1
[00:02:20]   |
[00:02:20] 1 | *1;2802;0c//! Documentation generation for rustbuilder.
[00:02:20]   | ^ expected item
[00:02:20] error: aborting due to previous error
[00:02:20] 
[00:02:20] error: Could not compile `bootstrap`.
[00:02:20] 
---
[00:02:24]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:24] error: expected item, found `*`
[00:02:24]  --> src/bootstrap/doc.rs:1:1
[00:02:24]   |
[00:02:24] 1 | *1;2802;0c//! Documentation generation for rustbuilder.
[00:02:24]   | ^ expected item
[00:02:24] error: aborting due to previous error
[00:02:24] 
[00:02:24] error: Could not compile `bootstrap`.
[00:02:24] 
---
travis_time:end:03fd7dfa:start=1548929524379909908,finish=1548929524385903908,duration=5994000
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:24091668
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:24f76470
travis_time:start:24f76470
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0077981e
$ dmesg | grep -i kill
