plain
travis_time:end:1a843158:start=1548948680019938579,finish=1548948829808766440,duration=149788827861
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:37]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:03:37]    |
[00:03:37] 5  | #![deny(warnings)]
[00:03:37]    |         ^^^^^^^^
[00:03:37]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:03:37]    = help: maybe it is overwritten before being read?
[00:03:37] error: aborting due to previous error
[00:03:37] 
[00:03:37] error: Could not compile `bootstrap`.
[00:03:37] warning: build failed, waiting for other jobs to finish...
---
[00:03:42]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:03:42]    |
[00:03:42] 5  | #![deny(warnings)]
[00:03:42]    |         ^^^^^^^^
[00:03:42]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:03:42]    = help: maybe it is overwritten before being read?
[00:03:42] error: aborting due to previous error
[00:03:42] 
[00:03:42] error: Could not compile `bootstrap`.
[00:03:42] warning: build failed, waiting for other jobs to finish...
---
[00:03:47]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:03:47]    |
[00:03:47] 5  | #![deny(warnings)]
[00:03:47]    |         ^^^^^^^^
[00:03:47]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:03:47]    = help: maybe it is overwritten before being read?
[00:03:47] error: aborting due to previous error
[00:03:47] 
[00:03:47] error: Could not compile `bootstrap`.
[00:03:47] 
---
[00:03:50]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:03:50]    |
[00:03:50] 5  | #![deny(warnings)]
[00:03:50]    |         ^^^^^^^^
[00:03:50]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:03:50]    = help: maybe it is overwritten before being read?
[00:03:50] error: aborting due to previous error
[00:03:50] 
[00:03:51] error: Could not compile `bootstrap`.
[00:03:51] 
---
[00:03:55]   --> src/bootstrap/bin/rustdoc.rs:5:9
[00:03:55]    |
[00:03:55] 5  | #![deny(warnings)]
[00:03:55]    |         ^^^^^^^^
[00:03:55]    = note: #[deny(unused_assignments)] implied by #[deny(warnings)]
[00:03:55]    = help: maybe it is overwritten before being read?
[00:03:55] error: aborting due to previous error
[00:03:55] 
[00:03:55] error: Could not compile `bootstrap`.
[00:03:55] 
---
travis_time:end:332c628d:start=1548949077156308382,finish=1548949077162488668,duration=6180286
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cbc46e0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:027e24e8
travis_time:start:027e24e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:050f6bda
$ dmesg | grep -i kill
