plain
2019-08-19T16:07:47.8054701Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-19T16:07:47.8243078Z ##[command]git config gc.auto 0
2019-08-19T16:07:47.8308303Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-19T16:07:47.8355913Z ##[command]git config --get-all http.proxy
2019-08-19T16:07:47.8471640Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63705/merge:refs/remotes/pull/63705/merge
---
2019-08-19T16:08:22.5832585Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-19T16:08:22.5832777Z 
2019-08-19T16:08:22.5833129Z   git checkout -b <new-branch-name>
2019-08-19T16:08:22.5833490Z 
2019-08-19T16:08:22.5838226Z HEAD is now at d093c8c79 Merge 0100f8b84c0bf7052759ccf370ed29359aa7bd64 into cdff9189556bb7de2b9a8a72344c9d8ec6099fcd
2019-08-19T16:08:22.5961418Z ##[section]Finishing: Checkout
2019-08-19T16:08:22.5967098Z ##[section]Starting: Decide whether to run this job
2019-08-19T16:08:22.5969996Z Task         : Bash
2019-08-19T16:08:22.5970042Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-19T16:08:22.5970088Z Version      : 3.151.3
2019-08-19T16:08:22.5970152Z Author       : Microsoft Corporation
2019-08-19T16:08:22.5970152Z Author       : Microsoft Corporation
2019-08-19T16:08:22.5970215Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-19T16:08:22.5970266Z ==============================================================================
2019-08-19T16:08:22.7137946Z Generating script.
2019-08-19T16:08:22.7165784Z ========================== Starting Command Output ===========================
2019-08-19T16:08:22.7182837Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/9c152950-48ee-40ec-bb1a-b1c74ebeffbd.sh
2019-08-19T16:08:23.0065314Z Executing the job since submodules are updated
2019-08-19T16:08:23.0145603Z ##[section]Finishing: Decide whether to run this job
2019-08-19T16:08:23.0154732Z ==============================================================================
2019-08-19T16:08:23.0154816Z Task         : Bash
2019-08-19T16:08:23.0154851Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-19T16:08:23.0155028Z Version      : 3.151.3
---
2019-08-19T18:25:44.6160709Z thread panicked while panicking. aborting.
2019-08-19T18:25:44.8629497Z [2019-08-19T18:25:44Z ERROR rls::server] Can't read message
2019-08-19T18:25:44.8630751Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/libcore/result.rs:1084:5
2019-08-19T18:25:44.8637766Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-19T18:25:44.8638593Z error: process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-4789b7d22e4fb0f9` (signal: 4, SIGILL: illegal instruction)
2019-08-19T18:25:44.8642753Z 
2019-08-19T18:25:44.8643533Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--"
2019-08-19T18:25:44.8643764Z expected success, got: exit code: 101
2019-08-19T18:25:44.8643854Z 
---
2019-08-19T18:28:47.5590371Z Verifying status of embedded-book...
2019-08-19T18:28:47.5602004Z Verifying status of rustc-guide...
2019-08-19T18:28:47.5613585Z This PR updated 'src/doc/rustc-guide', verifying if status is 'test-pass'...
2019-08-19T18:28:47.5622321Z 
2019-08-19T18:28:47.5630252Z ⚠️ We detected that this PR updated 'rustc-guide', but its tests failed.
2019-08-19T18:28:47.5630349Z 
2019-08-19T18:28:47.5630663Z If you do intend to update 'rustc-guide', please check the error messages above and
2019-08-19T18:28:47.5630739Z commit another update.
2019-08-19T18:28:47.5630767Z 
2019-08-19T18:28:47.5631024Z If you do NOT intend to update 'rustc-guide', please ensure you did not accidentally
2019-08-19T18:28:47.5631278Z change the submodule at 'src/doc/rustc-guide'. You may ask your reviewer for the
2019-08-19T18:28:47.5631346Z proper steps.
2019-08-19T18:28:47.5641835Z   local time: Mon Aug 19 18:28:47 UTC 2019
2019-08-19T18:28:47.7448393Z   network time: Mon, 19 Aug 2019 18:28:47 GMT
2019-08-19T18:28:47.7452906Z == end clock drift check ==
2019-08-19T18:28:47.7452906Z == end clock drift check ==
2019-08-19T18:28:48.7124105Z ##[error]Bash exited with code '3'.
2019-08-19T18:28:48.7171964Z ##[section]Starting: Checkout
2019-08-19T18:28:48.7173576Z ==============================================================================
2019-08-19T18:28:48.7173618Z Task         : Get sources
2019-08-19T18:28:48.7173658Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
