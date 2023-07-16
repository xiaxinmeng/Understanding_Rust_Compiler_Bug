plain
travis_time:end:04b28fb9:start=1557147506185357089,finish=1557147593944804622,duration=87759447533
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:04:52]    Compiling rustc-demangle v0.1.10
[00:04:57]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:04:57]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:57]    Compiling hashbrown v0.3.0
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:171:13
[00:05:01]     |
[00:05:01] 171 |             cvt_r(|| libc::dup2(fd, libc::STDIN_FILENO))?;
[00:05:01]     |             |
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:174:13
[00:05:01]     |
[00:05:01] 174 |             cvt_r(|| libc::dup2(fd, libc::STDOUT_FILENO))?;
[00:05:01]     |             |
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:177:13
[00:05:01]     |
[00:05:01] 177 |             cvt_r(|| libc::dup2(fd, libc::STDERR_FILENO))?;
[00:05:01]     |             |
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:182:17
[00:05:01]     |
[00:05:01] 182 |                 cvt(libc::setgid(u as gid_t))?;
[00:05:01]     |                 |
[00:05:01]     |                 cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |                 cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |                 in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:194:17
[00:05:01]     |
[00:05:01] 194 |                 cvt(libc::setuid(u as uid_t))?;
[00:05:01]     |                 |
[00:05:01]     |                 cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |                 cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |                 in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:198:13
[00:05:01]     |
[00:05:01] 198 |             cvt(libc::chdir(cwd.as_ptr()))?;
[00:05:01]     |             |
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:221:17
[00:05:01]     |
[00:05:01] 221 |                 cvt(libc::sigemptyset(&mut set))?;
[00:05:01]     |                 |
[00:05:01]     |                 cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |                 cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |                 in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:223:13
[00:05:01]     |
[00:05:01] 223 | /             cvt(libc::pthread_sigmask(libc::SIG_SETMASK, &set,
[00:05:01] 224 | |                                          ptr::null_mut()))?;
[00:05:01]     | |                                                           |
[00:05:01]     | |                                                           cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     | |                                                           cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     | |___________________________________________________________in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
[00:05:01] 
[00:05:01] error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `core::ops::Try`)
[00:05:01]    --> src/libstd/sys/unix/process/process_unix.rs:232:13
[00:05:01] 232 |             callback()?;
[00:05:01]     |             ^^^^^^^^^^^
[00:05:01]     |             |
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             cannot use the `?` operator in a function that returns `io::error::Error`
[00:05:01]     |             in this expansion of `desugaring of `?``
[00:05:01]     |
[00:05:01]     = help: the trait `core::ops::Try` is not implemented for `io::error::Error`
[00:05:01]     = note: required by `core::ops::Try::from_error`
[00:05:01] 
---
travis_time:end:08ba5560:start=1557147905190159569,finish=1557147905194530753,duration=4371184
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1c9ccf55
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14b5dff4
travis_time:start:14b5dff4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file 
