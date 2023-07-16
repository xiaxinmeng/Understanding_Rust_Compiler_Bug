plain
travis_time:end:0110203e:start=1553012489688875679,finish=1553012491886705941,duration=2197830262
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:30:50]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:32:30] error[E0425]: cannot find value `crate_name` in this scope
[00:32:30]    --> src/librustc_driver/lib.rs:357:89
[00:32:30]     |
[00:32:30] 357 |                         DumpHandler::new(compiler.output_dir().as_ref().map(|p| &**p), &crate_name)
[00:32:30] help: possible candidate is found in another module, you can import it into scope
[00:32:30]     |
[00:32:30] 54  | use rustc::ty::query::Query::crate_name;
[00:32:30]     |
---
156416 ./src/llvm-project/clang
143944 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
143940 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
141928 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex
141924 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex/s-fahw60dhvh-3zpqg3-3d9d4wenbbd1z
123628 ./src/llvm-project/llvm/test/CodeGen
108532 ./src/llvm-project/lldb
97584 ./src/llvm-project/clang/test
94844 ./.git
