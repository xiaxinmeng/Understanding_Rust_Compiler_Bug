plain
2020-03-15T13:27:37.6381018Z ========================== Starting Command Output ===========================
2020-03-15T13:27:37.6383838Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cd4b71ac-b05d-417c-b5d0-08b3237498da.sh
2020-03-15T13:27:37.6384154Z 
2020-03-15T13:27:37.6388532Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-15T13:27:37.6408861Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68970/merge to s
2020-03-15T13:27:37.6412524Z Task         : Get sources
2020-03-15T13:27:37.6412846Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T13:27:37.6413156Z Version      : 1.0.0
2020-03-15T13:27:37.6413384Z Author       : Microsoft
---
2020-03-15T13:27:38.6313806Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-15T13:27:38.6319538Z ##[command]git config gc.auto 0
2020-03-15T13:27:38.6323531Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-15T13:27:38.6327225Z ##[command]git config --get-all http.proxy
2020-03-15T13:27:38.6333981Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68970/merge:refs/remotes/pull/68970/merge
---
2020-03-15T13:36:23.1266543Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2020-03-15T13:36:26.0537384Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-03-15T13:36:26.1794336Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-03-15T13:36:26.5794871Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-03-15T13:36:26.5962760Z error[E0432]: unresolved import `rustc_infer::traits::wf`
2020-03-15T13:36:26.5963921Z   --> src/librustc_typeck/impl_wf_check/min_specialization.rs:80:5
2020-03-15T13:36:26.5965097Z 80 | use rustc_infer::traits::wf;
2020-03-15T13:36:26.5965097Z 80 | use rustc_infer::traits::wf;
2020-03-15T13:36:26.5965963Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ no `wf` in `traits`
2020-03-15T13:36:28.0716083Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-15T13:36:31.0569769Z error: aborting due to previous error
2020-03-15T13:36:31.0570128Z 
2020-03-15T13:36:31.0577696Z For more information about this error, try `rustc --explain E0432`.
2020-03-15T13:36:31.0577696Z For more information about this error, try `rustc --explain E0432`.
2020-03-15T13:36:31.0654708Z error: could not compile `rustc_typeck`.
2020-03-15T13:36:31.0656734Z warning: build failed, waiting for other jobs to finish...
2020-03-15T13:36:43.0913186Z error: build failed
2020-03-15T13:36:43.1023732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-15T13:36:43.1025197Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-15T13:36:43.1025578Z Build completed unsuccessfully in 0:05:15
2020-03-15T13:36:43.1057968Z == clock drift check ==
2020-03-15T13:36:43.1074588Z   local time: Sun Mar 15 13:36:43 UTC 2020
2020-03-15T13:36:43.1074588Z   local time: Sun Mar 15 13:36:43 UTC 2020
2020-03-15T13:36:43.3979936Z   network time: Sun, 15 Mar 2020 13:36:43 GMT
2020-03-15T13:36:43.3983182Z == end clock drift check ==
2020-03-15T13:36:44.1987755Z 
2020-03-15T13:36:44.2073119Z ##[error]Bash exited with code '1'.
2020-03-15T13:36:44.2088167Z ##[section]Finishing: Run build
2020-03-15T13:36:44.2145397Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68970/merge to s
2020-03-15T13:36:44.2150793Z Task         : Get sources
2020-03-15T13:36:44.2151183Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-15T13:36:44.2151556Z Version      : 1.0.0
2020-03-15T13:36:44.2151806Z Author       : Microsoft
2020-03-15T13:36:44.2151806Z Author       : Microsoft
2020-03-15T13:36:44.2152199Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-15T13:36:44.2152677Z ==============================================================================
2020-03-15T13:36:44.6016620Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-15T13:36:44.6065898Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68970/merge to s
2020-03-15T13:36:44.6165782Z Cleaning up task key
2020-03-15T13:36:44.6167179Z Start cleaning up orphan processes.
2020-03-15T13:36:44.6384614Z Terminate orphan process: pid (5838) (python)
2020-03-15T13:36:44.6605798Z ##[section]Finishing: Finalize Job
