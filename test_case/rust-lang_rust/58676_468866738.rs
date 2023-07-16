plain
travis_time:end:28679552:start=1551489804996251458,finish=1551489892629514745,duration=87633263287
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:00]    Compiling serde_derive v1.0.81
[00:02:01]    Compiling serde_json v1.0.33
[00:02:01]    Compiling toml v0.4.10
[00:02:09]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:16] error[E0382]: borrow of moved value: `cmd`
[00:02:16]   --> src/bootstrap/sanity.rs:41:45
[00:02:16]    |
[00:02:16] 41 |                 let target = dbg!(path.join(&cmd));
[00:02:16]    |                                             ^^^^ value borrowed here after move
[00:02:16] ...
[00:02:16] 45 |                 dbg!(dbg!(target.join(&PathBuf::from(cmd).join(".exe"))).exists()) { // some/path/git/git.exe
[00:02:16]    |                                                      --- value moved here, in previous iteration of loop
[00:02:16]    |
[00:02:16]    = note: move occurs because `cmd` has type `std::ffi::OsString`, which does not implement the `Copy` trait
[00:02:17] error: aborting due to previous error
[00:02:17] 
[00:02:17] For more information about this error, try `rustc --explain E0382`.
[00:02:17] error: Could not compile `bootstrap`.
---
[00:02:17] make: *** [prepare] Error 1
[00:02:17] Makefile:70: recipe for target 'prepare' failed
[00:02:18] Command failed. Attempt 2/5:
[00:02:18]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:25] error[E0382]: borrow of moved value: `cmd`
[00:02:25]   --> src/bootstrap/sanity.rs:41:45
[00:02:25]    |
[00:02:25] 41 |                 let target = dbg!(path.join(&cmd));
[00:02:25]    |                                             ^^^^ value borrowed here after move
[00:02:25] ...
[00:02:25] 45 |                 dbg!(dbg!(target.join(&PathBuf::from(cmd).join(".exe"))).exists()) { // some/path/git/git.exe
[00:02:25]    |                                                      --- value moved here, in previous iteration of loop
[00:02:25]    |
[00:02:25]    = note: move occurs because `cmd` has type `std::ffi::OsString`, which does not implement the `Copy` trait
[00:02:25] error: aborting due to previous error
[00:02:25] 
[00:02:25] For more information about this error, try `rustc --explain E0382`.
[00:02:25] error: Could not compile `bootstrap`.
---
[00:02:25] make: *** [prepare] Error 1
[00:02:25] Makefile:70: recipe for target 'prepare' failed
[00:02:27] Command failed. Attempt 3/5:
[00:02:28]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:35] error[E0382]: borrow of moved value: `cmd`
[00:02:35]   --> src/bootstrap/sanity.rs:41:45
[00:02:35]    |
[00:02:35] 41 |                 let target = dbg!(path.join(&cmd));
[00:02:35]    |                                             ^^^^ value borrowed here after move
[00:02:35] ...
[00:02:35] 45 |                 dbg!(dbg!(target.join(&PathBuf::from(cmd).join(".exe"))).exists()) { // some/path/git/git.exe
[00:02:35]    |                                                      --- value moved here, in previous iteration of loop
[00:02:35]    |
[00:02:35]    = note: move occurs because `cmd` has type `std::ffi::OsString`, which does not implement the `Copy` trait
[00:02:35] error: aborting due to previous error
[00:02:35] 
[00:02:35] For more information about this error, try `rustc --explain E0382`.
[00:02:35] error: Could not compile `bootstrap`.
---
[00:02:35] Makefile:70: recipe for target 'prepare' failed
[00:02:35] make: *** [prepare] Error 1
[00:02:38] Command failed. Attempt 4/5:
[00:02:39]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:45] error[E0382]: borrow of moved value: `cmd`
[00:02:45]   --> src/bootstrap/sanity.rs:41:45
[00:02:45]    |
[00:02:45] 41 |                 let target = dbg!(path.join(&cmd));
[00:02:45]    |                                             ^^^^ value borrowed here after move
[00:02:45] ...
[00:02:45] 45 |                 dbg!(dbg!(target.join(&PathBuf::from(cmd).join(".exe"))).exists()) { // some/path/git/git.exe
[00:02:45]    |                                                      --- value moved here, in previous iteration of loop
[00:02:45]    |
[00:02:45]    = note: move occurs because `cmd` has type `std::ffi::OsString`, which does not implement the `Copy` trait
[00:02:46] error: aborting due to previous error
[00:02:46] 
[00:02:46] For more information about this error, try `rustc --explain E0382`.
[00:02:46] error: Could not compile `bootstrap`.
---
[00:02:46] make: *** [prepare] Error 1
[00:02:46] Makefile:70: recipe for target 'prepare' failed
[00:02:50] Command failed. Attempt 5/5:
[00:02:50]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:57] error[E0382]: borrow of moved value: `cmd`
[00:02:57]   --> src/bootstrap/sanity.rs:41:45
[00:02:57]    |
[00:02:57] 41 |                 let target = dbg!(path.join(&cmd));
[00:02:57]    |                                             ^^^^ value borrowed here after move
[00:02:57] ...
[00:02:57] 45 |                 dbg!(dbg!(target.join(&PathBuf::from(cmd).join(".exe"))).exists()) { // some/path/git/git.exe
[00:02:57]    |                                                      --- value moved here, in previous iteration of loop
[00:02:57]    |
[00:02:57]    = note: move occurs because `cmd` has type `std::ffi::OsString`, which does not implement the `Copy` trait
[00:02:58] error: aborting due to previous error
[00:02:58] 
[00:02:58] For more information about this error, try `rustc --explain E0382`.
[00:02:58] error: Could not compile `bootstrap`.
---
travis_time:end:2a679b1e:start=1551490081106625867,finish=1551490081113528248,duration=6902381
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2c80b618
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2d58a9a0
travis_time:start:2d58a9a0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2e8c10b7
$ dmesg | grep -i kill
