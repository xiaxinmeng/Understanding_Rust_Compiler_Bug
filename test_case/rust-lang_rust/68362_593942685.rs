plain
2020-03-03T13:02:47.4203972Z ========================== Starting Command Output ===========================
2020-03-03T13:02:47.4232532Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bcafb6f9-ef37-4d14-a4f9-70ef90cc40e0.sh
2020-03-03T13:02:47.6661040Z 
2020-03-03T13:02:47.6712595Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T13:02:47.6737793Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T13:02:47.6743161Z Task         : Get sources
2020-03-03T13:02:47.6743480Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T13:02:47.6743816Z Version      : 1.0.0
2020-03-03T13:02:47.6744034Z Author       : Microsoft
---
2020-03-03T13:02:49.9495684Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T13:02:49.9688194Z ##[command]git config gc.auto 0
2020-03-03T13:02:49.9728015Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T13:02:49.9753188Z ##[command]git config --get-all http.proxy
2020-03-03T13:02:49.9846179Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-03-03T13:11:08.6549133Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-03T13:11:09.3121356Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-03T13:11:10.6761227Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-03T13:11:31.1746979Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-03T13:11:31.7067973Z error[E0412]: cannot find type `RegionOutlivesPredicate` in this scope
2020-03-03T13:11:31.7070172Z      |
2020-03-03T13:11:31.7070172Z      |
2020-03-03T13:11:31.7071074Z 152  |     ) -> RegionOutlivesPredicate<Region<'tcx>, Region<'tcx>> {
2020-03-03T13:11:31.7072737Z      | 
2020-03-03T13:11:31.7073541Z     ::: /checkout/src/librustc/ty/mod.rs:1389:1
2020-03-03T13:11:31.7074273Z      |
2020-03-03T13:11:31.7074273Z      |
2020-03-03T13:11:31.7075158Z 1389 | pub struct OutlivesPredicate<A, B>(pub A, pub B); // `A: B`
2020-03-03T13:11:31.7076329Z      | ------------------------------------------------- similarly named struct `OutlivesPredicate` defined here
2020-03-03T13:11:31.7078848Z help: a struct with a similar name exists
2020-03-03T13:11:31.7079494Z      |
2020-03-03T13:11:31.7079494Z      |
2020-03-03T13:11:31.7080274Z 152  |     ) -> OutlivesPredicate<Region<'tcx>, Region<'tcx>> {
2020-03-03T13:11:31.7082244Z help: possible candidate is found in another module, you can import it into scope
2020-03-03T13:11:31.7082989Z      |
2020-03-03T13:11:31.7082989Z      |
2020-03-03T13:11:31.7083764Z 3    | use rustc::ty::RegionOutlivesPredicate;
2020-03-03T13:11:31.7089465Z 
2020-03-03T13:11:34.2165835Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-03T13:11:34.2891565Z error: aborting due to previous error
2020-03-03T13:11:34.2895451Z 
2020-03-03T13:11:34.2895451Z 
2020-03-03T13:11:34.2900334Z For more information about this error, try `rustc --explain E0412`.
2020-03-03T13:11:34.2937197Z error: could not compile `rustc_infer`.
2020-03-03T13:11:34.2938022Z warning: build failed, waiting for other jobs to finish...
2020-03-03T13:11:35.3306552Z error: build failed
2020-03-03T13:11:35.3426395Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-03T13:11:35.3429017Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-03T13:11:35.3429351Z Build completed unsuccessfully in 0:04:52
2020-03-03T13:11:35.3435600Z == clock drift check ==
2020-03-03T13:11:35.3454568Z   local time: Tue Mar  3 13:11:35 UTC 2020
2020-03-03T13:11:35.3454568Z   local time: Tue Mar  3 13:11:35 UTC 2020
2020-03-03T13:11:35.6345857Z   network time: Tue, 03 Mar 2020 13:11:35 GMT
2020-03-03T13:11:35.6349279Z == end clock drift check ==
2020-03-03T13:11:36.3577280Z 
2020-03-03T13:11:36.3640945Z ##[error]Bash exited with code '1'.
2020-03-03T13:11:36.3654064Z ##[section]Finishing: Run build
2020-03-03T13:11:36.3706722Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T13:11:36.3711947Z Task         : Get sources
2020-03-03T13:11:36.3712271Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T13:11:36.3712575Z Version      : 1.0.0
2020-03-03T13:11:36.3712813Z Author       : Microsoft
2020-03-03T13:11:36.3712813Z Author       : Microsoft
2020-03-03T13:11:36.3713142Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T13:11:36.3713516Z ==============================================================================
2020-03-03T13:11:36.6761421Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T13:11:36.6801200Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-03-03T13:11:36.6887334Z Cleaning up task key
2020-03-03T13:11:36.6888537Z Start cleaning up orphan processes.
2020-03-03T13:11:36.7150003Z Terminate orphan process: pid (4671) (python)
2020-03-03T13:11:36.7187297Z ##[section]Finishing: Finalize Job
