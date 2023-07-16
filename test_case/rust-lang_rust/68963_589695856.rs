plain
2020-02-21T15:05:03.0291131Z ========================== Starting Command Output ===========================
2020-02-21T15:05:03.0296997Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/441531fc-e64f-475c-a7d7-f6a755cebb1b.sh
2020-02-21T15:05:03.0297500Z 
2020-02-21T15:05:03.0301549Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-21T15:05:03.0324464Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-21T15:05:03.0327822Z Task         : Get sources
2020-02-21T15:05:03.0328121Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T15:05:03.0328402Z Version      : 1.0.0
2020-02-21T15:05:03.0328595Z Author       : Microsoft
---
2020-02-21T15:05:04.6684478Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-21T15:05:04.6691592Z ##[command]git config gc.auto 0
2020-02-21T15:05:04.6695669Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-21T15:05:04.6699425Z ##[command]git config --get-all http.proxy
2020-02-21T15:05:04.6707661Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
---
2020-02-21T15:14:26.9959530Z For more information about this error, try `rustc --explain E0412`.
2020-02-21T15:14:27.0007763Z error: could not compile `rustc_infer`.
2020-02-21T15:14:27.0009792Z warning: build failed, waiting for other jobs to finish...
2020-02-21T15:14:28.1091780Z error: build failed
2020-02-21T15:14:28.1122024Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-21T15:14:28.1136625Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-21T15:14:28.1137038Z Build completed unsuccessfully in 0:05:36
2020-02-21T15:14:28.1194909Z == clock drift check ==
2020-02-21T15:14:28.1209900Z   local time: Fri Feb 21 15:14:28 UTC 2020
2020-02-21T15:14:28.1209900Z   local time: Fri Feb 21 15:14:28 UTC 2020
2020-02-21T15:14:28.6647809Z   network time: Fri, 21 Feb 2020 15:14:28 GMT
2020-02-21T15:14:28.6648222Z == end clock drift check ==
2020-02-21T15:14:29.3011115Z 
2020-02-21T15:14:29.3084563Z ##[error]Bash exited with code '1'.
2020-02-21T15:14:29.3099687Z ##[section]Finishing: Run build
2020-02-21T15:14:29.3147094Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-21T15:14:29.3152369Z Task         : Get sources
2020-02-21T15:14:29.3152754Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-21T15:14:29.3153126Z Version      : 1.0.0
2020-02-21T15:14:29.3153372Z Author       : Microsoft
2020-02-21T15:14:29.3153372Z Author       : Microsoft
2020-02-21T15:14:29.3153764Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-21T15:14:29.3154235Z ==============================================================================
2020-02-21T15:14:30.3630144Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-21T15:14:30.3669772Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-21T15:14:30.3771510Z Cleaning up task key
2020-02-21T15:14:30.3772995Z Start cleaning up orphan processes.
2020-02-21T15:14:30.3981459Z Terminate orphan process: pid (3643) (python)
2020-02-21T15:14:30.4330319Z ##[section]Finishing: Finalize Job
