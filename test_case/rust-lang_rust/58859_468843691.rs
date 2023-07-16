plain
travis_time:end:14ec117a:start=1551481737030638070,finish=1551481737915848603,duration=885210533
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:19]    Compiling libc v0.2.46
[00:05:19]    Compiling rand_core v0.3.0
[00:05:19]    Compiling nodrop v0.1.12
[00:05:19]    Compiling cfg-if v0.1.6
[00:05:20] error[E0277]: the trait bound `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>: std::error::NotBoxDynError` is not satisfied in `error::Error`
[00:05:20]     |
[00:05:20]     |
[00:05:20] 170 |             ErrorKind::Unavailable => io::Error::new(NotFound, error),
[00:05:20]     |                                       ^^^^^^^^^^^^^^ within `error::Error`, the trait `std::error::NotBoxDynError` is not implemented for `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>`
[00:05:20]     = help: the following implementations were found:
[00:05:20]     = help: the following implementations were found:
[00:05:20]               <std::boxed::Box<(dyn std::error::Error + 'a)> as std::error::NotBoxDynError>
[00:05:20]               <std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'a)> as std::error::NotBoxDynError>
[00:05:20]     = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>>`
[00:05:20]     = note: required because it appears within the type `error::Error`
[00:05:20]     = note: required because of the requirements on the impl of `std::convert::From<error::Error>` for `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>`
[00:05:20]     = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>>` for `error::Error`
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0277]: the trait bound `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>: std::error::NotBoxDynError` is not satisfied in `error::Error`
[00:05:20]     |
[00:05:20]     |
[00:05:20] 172 |             ErrorKind::Transient => io::Error::new(Other, error),
[00:05:20]     |                                     ^^^^^^^^^^^^^^ within `error::Error`, the trait `std::error::NotBoxDynError` is not implemented for `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>`
[00:05:20]     = help: the following implementations were found:
[00:05:20]     = help: the following implementations were found:
[00:05:20]               <std::boxed::Box<(dyn std::error::Error + 'a)> as std::error::NotBoxDynError>
[00:05:20]               <std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'a)> as std::error::NotBoxDynError>
[00:05:20]     = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>>`
[00:05:20]     = note: required because it appears within the type `error::Error`
[00:05:20]     = note: required because of the requirements on the impl of `std::convert::From<error::Error>` for `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>`
[00:05:20]     = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>>` for `error::Error`
[00:05:20] 
[00:05:20] 
[00:05:20] error[E0277]: the trait bound `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>: std::error::NotBoxDynError` is not satisfied in `error::Error`
[00:05:20]     |
[00:05:20]     |
[00:05:20] 173 |             ErrorKind::NotReady => io::Error::new(WouldBlock, error),
[00:05:20]     |                                    ^^^^^^^^^^^^^^ within `error::Error`, the trait `std::error::NotBoxDynError` is not implemented for `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>`
[00:05:20]     = help: the following implementations were found:
[00:05:20]     = help: the following implementations were found:
[00:05:20]               <std::boxed::Box<(dyn std::error::Error + 'a)> as std::error::NotBoxDynError>
[00:05:20]               <std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'a)> as std::error::NotBoxDynError>
[00:05:20]     = note: required because it appears within the type `std::option::Option<std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>>`
[00:05:20]     = note: required because it appears within the type `error::Error`
[00:05:20]     = note: required because of the requirements on the impl of `std::convert::From<error::Error>` for `std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>`
[00:05:20]     = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<(dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static)>>` for `error::Error`
[00:05:20] 
[00:05:20] error: aborting due to 3 previous errors
[00:05:20] 
[00:05:20] For more information about this error, try `rustc --explain E0277`.
---
travis_time:end:0211281b:start=1551482069527762154,finish=1551482069532340569,duration=4578415
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:30af8ef7
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02b9f64d
travis_time:start:02b9f64d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:040b5271
$ dmesg | grep -i kill
