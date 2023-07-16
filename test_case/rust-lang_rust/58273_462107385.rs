plain
travis_time:end:103a7cf0:start=1549778314219868599,finish=1549778316582096125,duration=2362227526
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:19:12]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:19:12] error[E0432]: unresolved import `crate::errors`
[00:19:12]   --> src/librustc_resolve/error_reporting.rs:12:12
[00:19:12]    |
[00:19:12] 12 | use crate::errors::{Applicability, DiagnosticBuilder, DiagnosticId};
[00:19:12]    |            ^^^^^^ did you mean `syntax::errors`?
[00:19:14] error: aborting due to previous error
[00:19:14] 
[00:19:14] For more information about this error, try `rustc --explain E0432`.
[00:19:14] error: Could not compile `rustc_resolve`.
---
travis_time:end:02f80ad6:start=1549779531477024746,finish=1549779531483289639,duration=6264893
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d1dcac0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15166d90
travis_time:start:15166d90
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b9f5878
$ dmesg | grep -i kill
