plain
2020-02-22T00:02:47.5816750Z ========================== Starting Command Output ===========================
2020-02-22T00:02:47.5820633Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/26bb7ad7-4f1b-40b7-beb1-e4c4efd6d540.sh
2020-02-22T00:02:47.5820908Z 
2020-02-22T00:02:47.5824796Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T00:02:47.5846348Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T00:02:47.5850080Z Task         : Get sources
2020-02-22T00:02:47.5850377Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T00:02:47.5850666Z Version      : 1.0.0
2020-02-22T00:02:47.5850932Z Author       : Microsoft
---
2020-02-22T00:02:48.5897562Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T00:02:48.5905662Z ##[command]git config gc.auto 0
2020-02-22T00:02:48.5911004Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T00:02:48.5916690Z ##[command]git config --get-all http.proxy
2020-02-22T00:02:48.5925468Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-02-22T00:07:26.5284584Z Successfully built db94d8747e4c
2020-02-22T00:07:26.5325374Z Successfully tagged rust-ci:latest
2020-02-22T00:07:26.6255643Z Built container sha256:db94d8747e4cc89421d4746735c645157fd0a2165798ff187c69649ed37f20d0
2020-02-22T00:07:26.6290535Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/b175035bb9494ad265dae3d026be5e15672a5178af6266123c0aad3ada75246fe46bacb82fd18605414b17c550fdbf2628609dc1cd02f9158b0eff1b8b3a6279
2020-02-22T00:08:25.9358334Z upload failed: - to s3://rust-lang-ci-sccache2/docker/b175035bb9494ad265dae3d026be5e15672a5178af6266123c0aad3ada75246fe46bacb82fd18605414b17c550fdbf2628609dc1cd02f9158b0eff1b8b3a6279 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-02-22T00:08:26.4429424Z [CI_JOB_NAME=mingw-check]
2020-02-22T00:08:26.4464596Z == clock drift check ==
2020-02-22T00:08:26.4472191Z   local time: Sat Feb 22 00:08:26 UTC 2020
2020-02-22T00:08:26.7358631Z   network time: Sat, 22 Feb 2020 00:08:26 GMT
---
2020-02-22T00:14:09.7328133Z 
2020-02-22T00:14:09.7378371Z error[E0425]: cannot find value `containing_item` in this scope
2020-02-22T00:14:09.7379549Z     --> src/librustc_typeck/collect.rs:2359:71
2020-02-22T00:14:09.7380295Z      |
2020-02-22T00:14:09.7381126Z 2359 |                     if let hir::ItemKind::Impl { ref of_trait, .. } = containing_item {
2020-02-22T00:14:09.7383681Z 
2020-02-22T00:14:09.8619838Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-02-22T00:14:10.2490884Z     Checking rustc_mir_build v0.0.0 (/checkout/src/librustc_mir_build)
2020-02-22T00:14:13.3217048Z error[E0308]: mismatched types
---
2020-02-22T00:14:13.5940920Z For more information about an error, try `rustc --explain E0308`.
2020-02-22T00:14:13.5981527Z error: could not compile `rustc_typeck`.
2020-02-22T00:14:13.6006868Z warning: build failed, waiting for other jobs to finish...
2020-02-22T00:14:14.4323453Z error: build failed
2020-02-22T00:14:14.4342671Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-22T00:14:14.4349323Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-22T00:14:14.4349692Z Build completed unsuccessfully in 0:05:47
2020-02-22T00:14:14.4397449Z == clock drift check ==
2020-02-22T00:14:14.4405051Z   local time: Sat Feb 22 00:14:14 UTC 2020
2020-02-22T00:14:14.4405051Z   local time: Sat Feb 22 00:14:14 UTC 2020
2020-02-22T00:14:14.5996885Z   network time: Sat, 22 Feb 2020 00:14:14 GMT
2020-02-22T00:14:14.5998305Z == end clock drift check ==
2020-02-22T00:14:15.3445986Z 
2020-02-22T00:14:15.3478810Z ##[error]Bash exited with code '1'.
2020-02-22T00:14:15.3494088Z ##[section]Finishing: Run build
2020-02-22T00:14:15.3548731Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T00:14:15.3553669Z Task         : Get sources
2020-02-22T00:14:15.3554036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T00:14:15.3554400Z Version      : 1.0.0
2020-02-22T00:14:15.3554652Z Author       : Microsoft
2020-02-22T00:14:15.3554652Z Author       : Microsoft
2020-02-22T00:14:15.3555026Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T00:14:15.3555486Z ==============================================================================
2020-02-22T00:14:15.6880372Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T00:14:15.6883936Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-02-22T00:14:15.6977672Z Cleaning up task key
2020-02-22T00:14:15.6979395Z Start cleaning up orphan processes.
2020-02-22T00:14:15.7174517Z Terminate orphan process: pid (4310) (python)
2020-02-22T00:14:15.7405123Z ##[section]Finishing: Finalize Job
