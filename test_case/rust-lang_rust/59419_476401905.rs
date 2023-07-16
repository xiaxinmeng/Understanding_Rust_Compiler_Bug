plain
travis_time:end:14c381a0:start=1553552382442023990,finish=1553552383338734284,duration=896710294
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:21]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:45]    Compiling synstructure v0.10.1
[00:07:04]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:51]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:51] error: expected one of `.`, `?`, `{`, or an operator, found `;`
[00:07:51]    --> src/librustc/traits/util.rs:295:45
[00:07:51]     |
[00:07:51] 295 |         let def_id = match self.stack.pop()?;
[00:07:51]     |                      -----                  ^ expected one of `.`, `?`, `{`, or an operator here
[00:07:51]     |                      while parsing this match expression
[00:07:51]     |                      help: try removing this `match`
[00:07:51] 
[00:08:24] error: aborting due to previous error
---
198068 ./obj/build/cache/2019-02-27
157396 ./obj/build/bootstrap/debug/incremental
156416 ./src/llvm-project/clang
142512 ./obj/build/bootstrap/debug/incremental/bootstrap-pz4opbgkzvy
142508 ./obj/build/bootstrap/debug/incremental/bootstrap-pz4opbgkzvy/s-faos6w1wbn-1gdqdwt-3u618d595ya6l
108532 ./src/llvm-project/lldb
103952 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
97584 ./src/llvm-project/clang/test
94360 ./.git
---
ls: cannot travis_time:end:0290c126:start=1553552899831616293,finish=1553552899839003586,duration=7387293
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0f3190ce
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2720cd9c
$ dmesg | grep -i kill
