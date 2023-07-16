plain
travis_time:end:0732fcc9:start=1544827860614210423,finish=1544827924867238006,duration=64253027583
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:05:12]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:13] error[E0308]: mismatched types
[00:05:13]   --> src/librustc_target/spec/apple_ios_base.rs:77:22
[00:05:13]    |
[00:05:13] 77 |                      "-isysroot",
[00:05:13]    |
[00:05:13]    = note: expected type `std::string::String`
[00:05:13]               found type `&'static str`
[00:05:13] 
---
travis_time:end:0eb92dc4:start=1544828250246224483,finish=1544828250250461268,duration=4236785
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1cb008bc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09b95798
travis_time:start:09b95798
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
