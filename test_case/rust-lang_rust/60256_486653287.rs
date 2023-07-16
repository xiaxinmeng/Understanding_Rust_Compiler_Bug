plain
travis_time:end:08ea626e:start=1556193910441730489,finish=1556193911300274503,duration=858544014
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:26]    Compiling chalk-macros v0.1.0
[00:27:30]    Compiling humantime v1.2.0
[00:27:30]    Compiling backtrace-sys v0.1.27
[00:27:30]    Compiling miniz-sys v0.1.11
[00:27:30] error[E0283]: type annotations required: cannot resolve `std::option::Option<_>: std::convert::Into<std::option::Option<&str>>`
[00:27:30]    |
[00:27:30]    |
[00:27:30] 12 |         .define("MINIZ_NO_STDIO", None)
[00:27:30] 
[00:27:30] error: aborting due to previous error
[00:27:30] 
[00:27:30] For more information about this error, try `rustc --explain E0283`.
---
156512 ./src/llvm-project/clang
144704 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
144700 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc
142504 ./obj/build/bootstrap/debug/incremental/bootstrap-hfsog967tquc/s-fbmhov9905-1nmkeo-nmt2l0hs44c2
123640 ./src/llvm-project/llvm/test/CodeGen
108536 ./src/llvm-project/lldb
101624 ./.git
97600 ./src/llvm-project/clang/test
---
19256 ./src/llvm-project/lldb/www/cpp_reference/html
18100 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen
travis_time:end:27d9563a:start=1556195574024855381,finish=1556195574585706607,duration=560851226
travis_fold:end:after_failure.1
san/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00731020
$ dmesg | grep -i kill
