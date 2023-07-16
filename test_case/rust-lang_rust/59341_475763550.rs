plain
travis_time:end:20427f59:start=1553285034749457228,finish=1553285127303569759,duration=92554112531
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:01:41]    Compiling serde_derive v1.0.81
[00:01:43]    Compiling serde_json v1.0.33
[00:01:43]    Compiling toml v0.4.10
[00:01:51]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:01:56] error[E0382]: use of moved value: `libdir`
[00:01:56]     |
[00:01:56]     |
[00:01:56] 476 |             let libdir = builder.rustc_libdir(compiler);
[00:01:56]     |                 ------ move occurs because `libdir` has type `std::path::PathBuf`, which does not implement the `Copy` trait
[00:01:56] ...
[00:01:56] 490 |                             builder.install(&entry.path(), &image.join(libdir), 0o644);
[00:01:56]     |                                                                        ^^^^^^ value moved here, in previous iteration of loop
[00:01:57] error: aborting due to previous error
[00:01:57] 
[00:01:57] For more information about this error, try `rustc --explain E0382`.
[00:01:57] error: Could not compile `bootstrap`.
---
[00:01:57] make: *** [prepare] Error 1
[00:01:57] Makefile:69: recipe for target 'prepare' failed
[00:01:58] Command failed. Attempt 2/5:
[00:01:58]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:03] error[E0382]: use of moved value: `libdir`
[00:02:03]     |
[00:02:03]     |
[00:02:03] 476 |             let libdir = builder.rustc_libdir(compiler);
[00:02:03]     |                 ------ move occurs because `libdir` has type `std::path::PathBuf`, which does not implement the `Copy` trait
[00:02:03] ...
[00:02:03] 490 |                             builder.install(&entry.path(), &image.join(libdir), 0o644);
[00:02:03]     |                                                                        ^^^^^^ value moved here, in previous iteration of loop
[00:02:04] error: aborting due to previous error
[00:02:04] 
[00:02:04] For more information about this error, try `rustc --explain E0382`.
[00:02:04] error: Could not compile `bootstrap`.
---
[00:02:04] Makefile:69: recipe for target 'prepare' failed
[00:02:04] make: *** [prepare] Error 1
[00:02:06] Command failed. Attempt 3/5:
[00:02:07]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:12] error[E0382]: use of moved value: `libdir`
[00:02:12]     |
[00:02:12]     |
[00:02:12] 476 |             let libdir = builder.rustc_libdir(compiler);
[00:02:12]     |                 ------ move occurs because `libdir` has type `std::path::PathBuf`, which does not implement the `Copy` trait
[00:02:12] ...
[00:02:12] 490 |                             builder.install(&entry.path(), &image.join(libdir), 0o644);
[00:02:12]     |                                                                        ^^^^^^ value moved here, in previous iteration of loop
[00:02:13] error: aborting due to previous error
[00:02:13] 
[00:02:13] For more information about this error, try `rustc --explain E0382`.
[00:02:13] error: Could not compile `bootstrap`.
---
[00:02:13] Makefile:69: recipe for target 'prepare' failed
[00:02:13] make: *** [prepare] Error 1
[00:02:16] Command failed. Attempt 4/5:
[00:02:16]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:22] error[E0382]: use of moved value: `libdir`
[00:02:22]     |
[00:02:22]     |
[00:02:22] 476 |             let libdir = builder.rustc_libdir(compiler);
[00:02:22]     |                 ------ move occurs because `libdir` has type `std::path::PathBuf`, which does not implement the `Copy` trait
[00:02:22] ...
[00:02:22] 490 |                             builder.install(&entry.path(), &image.join(libdir), 0o644);
[00:02:22]     |                                                                        ^^^^^^ value moved here, in previous iteration of loop
[00:02:23] error: aborting due to previous error
[00:02:23] 
[00:02:23] For more information about this error, try `rustc --explain E0382`.
[00:02:23] error: Could not compile `bootstrap`.
---
[00:02:23] Makefile:69: recipe for target 'prepare' failed
[00:02:23] make: *** [prepare] Error 1
[00:02:27] Command failed. Attempt 5/5:
[00:02:27]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:33] error[E0382]: use of moved value: `libdir`
[00:02:33]     |
[00:02:33]     |
[00:02:33] 476 |             let libdir = builder.rustc_libdir(compiler);
[00:02:33]     |                 ------ move occurs because `libdir` has type `std::path::PathBuf`, which does not implement the `Copy` trait
[00:02:33] ...
[00:02:33] 490 |                             builder.install(&entry.path(), &image.join(libdir), 0o644);
[00:02:33]     |                                                                        ^^^^^^ value moved here, in previous iteration of loop
[00:02:34] error: aborting due to previous error
[00:02:34] 
[00:02:34] For more information about this error, try `rustc --explain E0382`.
[00:02:34] error: Could not compile `bootstrap`.
---
travis_time:end:19e1ee28:start=1553285291603155788,finish=1553285291608856174,duration=5700386
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:126f6924
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a9a6876
travis_time:start:1a9a6876
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:286a6a00
$ dmesg | grep -i kill
