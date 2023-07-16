plain
travis_fold:start:worker_info
Worker information
hostname: eec0f2d9-1897-4532-8b04-afc067312976@1.production-1-worker-com-gce-83fd
version: v6.2.0 https://github.com/travis-ci/worker/tree/5e5476e01646095f48eec13196fdb3faf8f5cbf7
instance: travis-job-00ff0a13-145c-48a0-a5e6-bc1ccd526706 travis-ci-stevonnie-xenial-1547455581-2c98a19 (via amqp)
travis_fold:end:worker_info
travis_fold:start:system_info
Build system information
Build language: shell
---
nvm version
0.34.0
perlbrew version
/home/travis/perl5/perlbrew/bin/perlbrew  - App::perlbrew/0.85
phpenv version
rbenv 1.1.1-39-g59785f6
rvm 1.29.7 (latest) by Michal Papis, Piotr Kuczynski, Wayne E. Seguin [https://rvm.io]
default ruby version
ruby 2.5.3p105 (2018-10-18 revision 65156) [x86_64-linux]
travis_fold:end:system_info
---
travis_time:end:18ff9b5b:start=1547479011664861308,finish=1547479012792918611,duration=1128057303
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:05:40]    Compiling arena v0.0.0 (/checkout/src/libarena)
[00:05:41]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:05:45]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:06]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:07] error: no rules expected the token `,`
[00:07:07]     --> src/librustc/session/config.rs:1353:64
[00:07:07] 696  | macro_rules! options {
[00:07:07]      | -------------------- when calling this macro
[00:07:07] ...
[00:07:07] ...
[00:07:07] 1353 |         `mir` (the MIR), or `mir-cfg` (graphviz formatted MIR)",
[00:07:07]      |                                                                ^ no rules expected this token in macro call
[00:07:08] error: aborting due to previous error
[00:07:08] 
[00:07:08] error: Could not compile `rustc`.
[00:07:08] warning: build failed, waiting for other jobs to finish...
---
186348 ./obj/build/cache/2019-01-04
162588 ./obj/build/bootstrap/debug/incremental
153280 ./src/tools/clang
146472 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405
146468 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405/s-f8ja3r10vc-gipfiu-1e6r9o7pag48r
110108 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107424 ./src/tools/lldb
95108 ./src/tools/clang/test
89976 ./src/llvm-emscripten/test/CodeGen
