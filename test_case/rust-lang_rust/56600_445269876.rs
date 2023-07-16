plain
travis_time:end:1cb40c05:start=1544196760871610917,finish=1544196761932529386,duration=1060918469
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:01:46]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:01:47] error[E0412]: cannot find type `Build` in the crate root
[00:01:47]    --> src/bootstrap/lib.rs:191:39
[00:01:47]     |
[00:01:47] 191 |     pub unsafe fn setup(build: &mut ::Build) {
[00:01:47]     |                                       ^^^^^ not found in the crate root
[00:01:47]     |
[00:01:47]     |
[00:01:47] 189 |     use cc::Build;
[00:01:47] 189 |     use crate::Build;
[00:01:47]     |
[00:01:47] 189 |     use crate::config::Build;
[00:01:47]     |
[00:01:47]     |
[00:01:47] 189 |     use petgraph::data::Build;
[00:01:47] 
[00:01:50] error: aborting due to previous error
[00:01:50] 
[00:01:50] For more information about this error, try `rustc --explain E0412`.
---
[00:01:51]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:01:52] error[E0412]: cannot find type `Build` in the crate root
[00:01:52]    --> src/bootstrap/lib.rs:191:39
[00:01:52]     |
[00:01:52] 191 |     pub unsafe fn setup(build: &mut ::Build) {
[00:01:52]     |                                       ^^^^^ not found in the crate root
[00:01:52]     |
[00:01:52]     |
[00:01:52] 189 |     use cc::Build;
[00:01:52] 189 |     use crate::Build;
[00:01:52]     |
[00:01:52] 189 |     use crate::config::Build;
[00:01:52]     |
[00:01:52]     |
[00:01:52] 189 |     use petgraph::data::Build;
[00:01:52] 
[00:01:56] error: aborting due to previous error
[00:01:56] 
[00:01:56] For more information about this error, try `rustc --explain E0412`.
---
[00:01:58]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:01:59] error[E0412]: cannot find type `Build` in the crate root
[00:01:59]    --> src/bootstrap/lib.rs:191:39
[00:01:59]     |
[00:01:59] 191 |     pub unsafe fn setup(build: &mut ::Build) {
[00:01:59]     |                                       ^^^^^ not found in the crate root
[00:01:59]     |
[00:01:59]     |
[00:01:59] 189 |     use cc::Build;
[00:01:59] 189 |     use crate::Build;
[00:01:59]     |
[00:01:59] 189 |     use crate::config::Build;
[00:01:59]     |
[00:01:59]     |
[00:01:59] 189 |     use petgraph::data::Build;
[00:01:59] 
[00:02:02] error: aborting due to previous error
[00:02:02] 
[00:02:02] For more information about this error, try `rustc --explain E0412`.
---
[00:02:05]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:06] error[E0412]: cannot find type `Build` in the crate root
[00:02:06]    --> src/bootstrap/lib.rs:191:39
[00:02:06]     |
[00:02:06] 191 |     pub unsafe fn setup(build: &mut ::Build) {
[00:02:06]     |                                       ^^^^^ not found in the crate root
[00:02:06]     |
[00:02:06]     |
[00:02:06] 189 |     use cc::Build;
[00:02:06] 189 |     use crate::Build;
[00:02:06]     |
[00:02:06] 189 |     use crate::config::Build;
[00:02:06]     |
[00:02:06]     |
[00:02:06] 189 |     use petgraph::data::Build;
[00:02:06] 
[00:02:09] error: aborting due to previous error
[00:02:09] 
[00:02:09] For more information about this error, try `rustc --explain E0412`.
---
[00:02:14]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:15] error[E0412]: cannot find type `Build` in the crate root
[00:02:15]    --> src/bootstrap/lib.rs:191:39
[00:02:15]     |
[00:02:15] 191 |     pub unsafe fn setup(build: &mut ::Build) {
[00:02:15]     |                                       ^^^^^ not found in the crate root
[00:02:15]     |
[00:02:15]     |
[00:02:15] 189 |     use cc::Build;
[00:02:15] 189 |     use crate::Build;
[00:02:15]     |
[00:02:15] 189 |     use crate::config::Build;
[00:02:15]     |
[00:02:15]     |
[00:02:15] 189 |     use petgraph::data::Build;
[00:02:15] 
[00:02:18] error: aborting due to previous error
[00:02:18] 
[00:02:18] For more information about this error, try `rustc --explain E0412`.
---
travis_time:end:11f3694b:start=1544196909680210186,finish=1544196909688117904,duration=7907718
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0dc5b8e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2b683ffe
travis_time:start:2b683ffe
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e900224
$ dmesg | grep -i kill
