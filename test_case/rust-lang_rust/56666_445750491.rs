plain
travis_time:end:0295a60a:start=1544429469773414786,finish=1544429470901596774,duration=1128181988
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-tools
---
[01:20:54]    Compiling rustc-ap-rustc_target v306.0.0
[01:20:54]    Compiling rustc-ap-syntax_pos v306.0.0
[01:21:07]    Compiling rustc-ap-rustc_errors v306.0.0
[01:21:26]    Compiling rustc-ap-syntax v306.0.0
[01:24:25] error[E0433]: failed to resolve: could not find `AsFail` in `failure`
[01:24:25]   --> src/tools/rustfmt/src/format-diff/main.rs:39:10
[01:24:25]    |
[01:24:25] 39 | #[derive(Fail, Debug)]
[01:24:25]    |          ^^^^ could not find `AsFail` in `failure`
[01:24:25] error: aborting due to previous error
[01:24:25] 
[01:24:25] For more information about this error, try `rustc --explain E0433`.
[01:24:25] error: Could not compile `rustfmt-nightly`.
---
[01:26:08] This PR updated 'src/tools/rustfmt', verifying if status is 'test-pass'...
[01:26:08] 
[01:26:08] ⚠️ We detected that this PR updated 'rustfmt', but its tests failed.
[01:26:08] 
[01:26:08] If you do intend to update 'rustfmt', please check the error messages above and
[01:26:08] commit another update.
[01:26:08] 
[01:26:08] If you do NOT intend to update 'rustfmt', please ensure you did not accidentally
[01:26:08] change t-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release
214760 ./obj/build/x86_64-unknown-linux-gnu/stage0-codegen/x86_64-unknown-linux-gnu/release/deps
210876 ./src/llvm-emscripten/test
204532 ./obj/build/x86_64-unknown-linux-gnu/stage1
204512 ./obj/build/x86_64-unknown-linux-gnu/stage1/lib
---
137492 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
134420 ./.git/modules
134416 ./.git/modules/src
133856 ./obj/build/bootstrap/debug/incremental/bootstrap-2sokbknxy3png
133852 ./obj/build/bootstrap/debug/incremental/bootstrap-2sokbknxy3png/s-f7gd5f0xvg-1qd9e0u-l8salou8bz1b
115356 ./src/llvm/test/CodeGen
111060 ./obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps
107888 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
107416 ./src/tools/lldb
