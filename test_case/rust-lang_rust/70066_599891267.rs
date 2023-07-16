plain
2020-03-17T05:36:13.1850590Z ========================== Starting Command Output ===========================
2020-03-17T05:36:13.1865570Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3c45a3f7-7ee0-4c2b-b67b-519bf065d815.sh
2020-03-17T05:36:13.2039371Z 
2020-03-17T05:36:13.2103042Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T05:36:13.2119487Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70066/merge to s
2020-03-17T05:36:13.2127201Z Task         : Get sources
2020-03-17T05:36:13.2127462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T05:36:13.2127906Z Version      : 1.0.0
2020-03-17T05:36:13.2128303Z Author       : Microsoft
---
2020-03-17T05:36:14.5484751Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T05:36:14.5533146Z ##[command]git config gc.auto 0
2020-03-17T05:36:14.5560258Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T05:36:14.5590850Z ##[command]git config --get-all http.proxy
2020-03-17T05:36:14.5664770Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70066/merge:refs/remotes/pull/70066/merge
---
2020-03-17T05:40:05.0623176Z Successfully built 803d80d10828
2020-03-17T05:40:05.0664000Z Successfully tagged rust-ci:latest
2020-03-17T05:40:05.1018135Z Built container sha256:803d80d1082889e1b6e456e2a5474318b46214b0f524ce4f2bb4cd127c437124
2020-03-17T05:40:05.1031589Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/60527bffc15381f3e0c55e9d5e0bec1faa9716d5daa2c5185b1eada10957f02df970124fe9c0b0ab4e67baad8366e42bbf86154e29c48a6ddf37e43ab6c347a2
2020-03-17T05:40:55.9929421Z upload failed: - to s3://rust-lang-ci-sccache2/docker/60527bffc15381f3e0c55e9d5e0bec1faa9716d5daa2c5185b1eada10957f02df970124fe9c0b0ab4e67baad8366e42bbf86154e29c48a6ddf37e43ab6c347a2 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-03-17T05:40:56.4320719Z [CI_JOB_NAME=mingw-check]
2020-03-17T05:40:56.4352642Z == clock drift check ==
2020-03-17T05:40:56.4362066Z   local time: Tue Mar 17 05:40:56 UTC 2020
2020-03-17T05:40:56.5782206Z   network time: Tue, 17 Mar 2020 05:40:56 GMT
---
2020-03-17T05:44:46.9476114Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-17T05:44:47.6848522Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-03-17T05:44:50.2262305Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-03-17T05:44:50.2404751Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-03-17T05:44:50.6447669Z error[E0432]: unresolved import `rustc_infer::infer::SuppressRegionErrors`
2020-03-17T05:44:50.6448546Z   --> src/librustc_typeck/impl_wf_check/min_specialization.rs:78:37
2020-03-17T05:44:50.6449002Z    |
2020-03-17T05:44:50.6449778Z 78 | use rustc_infer::infer::{InferCtxt, SuppressRegionErrors, TyCtxtInferExt};
2020-03-17T05:44:50.6450769Z    |                                     ^^^^^^^^^^^^^^^^^^^^ no `SuppressRegionErrors` in `infer`
2020-03-17T05:44:52.3935684Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-03-17T05:44:52.6748585Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-03-17T05:44:53.8042954Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-17T05:44:53.9924737Z error: aborting due to previous error
2020-03-17T05:44:53.9924737Z error: aborting due to previous error
2020-03-17T05:44:53.9928412Z 
2020-03-17T05:44:53.9933674Z For more information about this error, try `rustc --explain E0432`.
2020-03-17T05:44:53.9973828Z error: could not compile `rustc_typeck`.
2020-03-17T05:44:53.9974066Z 
2020-03-17T05:44:53.9974452Z To learn more, run the command again with --verbose.
2020-03-17T05:44:53.9974976Z warning: build failed, waiting for other jobs to finish...
2020-03-17T05:45:04.9768420Z error: build failed
2020-03-17T05:45:04.9788053Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-17T05:45:04.9793887Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-17T05:45:04.9794338Z Build completed unsuccessfully in 0:04:08
2020-03-17T05:45:04.9836391Z == clock drift check ==
2020-03-17T05:45:04.9843215Z   local time: Tue Mar 17 05:45:04 UTC 2020
2020-03-17T05:45:04.9843215Z   local time: Tue Mar 17 05:45:04 UTC 2020
2020-03-17T05:45:05.2534537Z   network time: Tue, 17 Mar 2020 05:45:05 GMT
2020-03-17T05:45:05.2541057Z == end clock drift check ==
2020-03-17T05:45:05.9210194Z 
2020-03-17T05:45:05.9263971Z ##[error]Bash exited with code '1'.
2020-03-17T05:45:05.9299313Z ##[section]Finishing: Run build
2020-03-17T05:45:05.9333398Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70066/merge to s
2020-03-17T05:45:05.9337658Z Task         : Get sources
2020-03-17T05:45:05.9337938Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T05:45:05.9338189Z Version      : 1.0.0
2020-03-17T05:45:05.9338389Z Author       : Microsoft
2020-03-17T05:45:05.9338389Z Author       : Microsoft
2020-03-17T05:45:05.9338670Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T05:45:05.9338993Z ==============================================================================
2020-03-17T05:45:06.2168243Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T05:45:06.2217622Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70066/merge to s
2020-03-17T05:45:06.2298372Z Cleaning up task key
2020-03-17T05:45:06.2299443Z Start cleaning up orphan processes.
2020-03-17T05:45:06.2456726Z Terminate orphan process: pid (3393) (python)
2020-03-17T05:45:06.2637060Z ##[section]Finishing: Finalize Job
