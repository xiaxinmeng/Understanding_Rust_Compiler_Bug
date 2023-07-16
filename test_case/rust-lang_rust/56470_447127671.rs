plain
travis_time:end:0186608a:start=1544734297386294686,finish=1544734352251437070,duration=54865142384
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:37:34]    Compiling parking_lot_core v0.3.0
[00:37:36]    Compiling tempfile v3.0.5
[00:37:36]    Compiling parking_lot v0.6.4
[00:37:37]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:37:42] error[E0277]: the trait bound `std::string::String: std::iter::Extend<&&str>` is not satisfied
[00:37:42]     |
[00:37:42]     |
[00:37:42] 496 |         prog.extend(&[main_pre, everything_else, main_post]);
[00:37:42]     |              ^^^^^^ the trait `std::iter::Extend<&&str>` is not implemented for `std::string::String`
[00:37:42]     = help: the following implementations were found:
[00:37:42]     = help: the following implementations were found:
[00:37:42]               <std::string::String as std::iter::Extend<&'a char>>
[00:37:42]               <std::string::String as std::iter::Extend<&'a str>>
[00:37:42]               <std::string::String as std::iter::Extend<char>>
[00:37:42]               <std::string::String as std::iter::Extend<std::borrow::Cow<'a, str>>>
[00:37:42]               <std::string::String as std::iter::Extend<std::string::String>>
[00:37:43] error: aborting due to previous error
[00:37:43] 
[00:37:43] For more information about this error, try `rustc --explain E0277`.
[00:37:43] error: Could not compile `rustdoc`.
---
36996 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools
36476 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/release
36476 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/release
36020 ./src/tools/clang/lib
34948 /x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0010d01d
$ dmesg | grep -i kill
