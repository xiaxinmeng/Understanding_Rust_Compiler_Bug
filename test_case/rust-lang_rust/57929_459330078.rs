plain
travis_time:end:19743168:start=1548937996145548201,finish=1548938106431457840,duration=110285909639
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:44]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:05:44]    |
[00:05:44] 5  | #![deny(warnings)]
[00:05:44]    |         ^^^^^^^^
[00:05:44]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:05:44]    = help: maybe it is overwritten before being read?
[00:05:44] error: aborting due to previous error
[00:05:44] 
[00:05:44] error: Could not compile `bootstrap`.
[00:05:44] warning: build failed, waiting for other jobs to finish...
---
[00:05:48]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:05:48]    |
[00:05:48] 5  | #![deny(warnings)]
[00:05:48]    |         ^^^^^^^^
[00:05:48]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:05:48]    = help: maybe it is overwritten before being read?
[00:05:48] error: aborting due to previous error
[00:05:48] 
[00:05:48] error: Could not compile `bootstrap`.
[00:05:48] warning: build failed, waiting for other jobs to finish...
---
[00:05:53]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:05:53]    |
[00:05:53] 5  | #![deny(warnings)]
[00:05:53]    |         ^^^^^^^^
[00:05:53]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:05:53]    = help: maybe it is overwritten before being read?
[00:05:53] error: aborting due to previous error
[00:05:53] 
[00:05:53] error: Could not compile `bootstrap`.
[00:05:53] 
---
[00:05:57]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:05:57]    |
[00:05:57] 5  | #![deny(warnings)]
[00:05:57]    |         ^^^^^^^^
[00:05:57]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:05:57]    = help: maybe it is overwritten before being read?
[00:05:57] error: aborting due to previous error
[00:05:57] 
[00:05:57] error: Could not compile `bootstrap`.
[00:05:57] 
---
[00:06:01]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:06:01]    |
[00:06:01] 5  | #![deny(warnings)]
[00:06:01]    |         ^^^^^^^^
[00:06:01]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:06:01]    = help: maybe it is overwritten before being read?
[00:06:01] error: aborting due to previous error
[00:06:01] 
[00:06:01] error: Could not compile `bootstrap`.
[00:06:01] 
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:038e7382
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15ce429a
$ dmesg | grep -i kill
