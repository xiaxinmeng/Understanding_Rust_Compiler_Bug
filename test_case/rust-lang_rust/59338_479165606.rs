plain
travis_time:end:0208e5d0:start=1554231066558204747,finish=1554231218429726038,duration=151871521291
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:45:25]    Compiling cc v1.0.28
[00:45:25]    Compiling core v0.0.0 (/checkout/src/libcore)
[00:45:25]    Compiling libc v0.2.51
[00:45:25]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:45:25] thread 'rustc' panicked at 'did not find a cycle', src/librustc/ty/query/job.rs:133:9
[00:45:25] 
[00:45:25] error: internal compiler error: unexpected panic
[00:45:25] 
[00:45:25] note: the compiler unexpectedly panicked. this is a bug.
[00:45:25] note: the compiler unexpectedly panicked. this is a bug.
[00:45:25] 
[00:45:25] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:45:25] 
[00:45:25] note: rustc 1.35.0-dev running on x86_64-unknown-linux-gnu
[00:45:25] 
[00:45:25] note: compiler flags: -Z external-macro-backtrace -Z emit-stack-sizes -Z force-unstable-if-unmarked -C opt-level=2 -C prefer-dynamic -C debug-assertions=y -C codegen-units=1 -C link-args=-Wl,-rpath,$ORIGIN/../lib --crate-type lib
[00:45:25] note: some of the compiler flags provided by cargo are hidden
[00:45:25] 
[00:45:25] error: Could not compile `core`.
[00:45:25] warning: build failed, waiting for other jobs to finish...
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1a8d648d
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/home/travic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:021d0689
$ dmesg | grep -i kill
