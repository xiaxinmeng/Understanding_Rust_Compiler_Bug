plain
2020-02-22T15:40:43.3918139Z ========================== Starting Command Output ===========================
2020-02-22T15:40:43.3933364Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/bc43eedf-0b17-4e67-a446-460e074da5ef.sh
2020-02-22T15:40:43.6510595Z 
2020-02-22T15:40:43.6615725Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T15:40:43.6645354Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:40:43.6722313Z Task         : Get sources
2020-02-22T15:40:43.6724895Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:40:43.6725325Z Version      : 1.0.0
2020-02-22T15:40:43.6726434Z Author       : Microsoft
---
2020-02-22T15:40:46.2097876Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T15:40:46.2337803Z ##[command]git config gc.auto 0
2020-02-22T15:40:46.2365841Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T15:40:46.2400212Z ##[command]git config --get-all http.proxy
2020-02-22T15:40:46.2500845Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
---
2020-02-22T15:49:46.2865581Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-02-22T15:49:46.7761863Z error[E0433]: failed to resolve: use of undeclared type or module `Symbol`
2020-02-22T15:49:46.7762905Z    --> src/librustc_infer/traits/error_reporting/suggestions.rs:145:33
2020-02-22T15:49:46.7763541Z     |
2020-02-22T15:49:46.7764170Z 145 |                     let param = Symbol::intern(&param_name);
2020-02-22T15:49:46.7765399Z 
2020-02-22T15:49:49.7119569Z error: aborting due to previous error
2020-02-22T15:49:49.7119936Z 
2020-02-22T15:49:49.7140386Z For more information about this error, try `rustc --explain E0433`.
2020-02-22T15:49:49.7140386Z For more information about this error, try `rustc --explain E0433`.
2020-02-22T15:49:49.7166787Z error: could not compile `rustc_infer`.
2020-02-22T15:49:49.7168836Z warning: build failed, waiting for other jobs to finish...
2020-02-22T15:49:49.7612461Z error: build failed
2020-02-22T15:49:49.7635528Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-22T15:49:49.7649353Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-22T15:49:49.7649721Z Build completed unsuccessfully in 0:05:17
2020-02-22T15:49:49.7696655Z == clock drift check ==
2020-02-22T15:49:49.7714117Z   local time: Sat Feb 22 15:49:49 UTC 2020
2020-02-22T15:49:49.7714117Z   local time: Sat Feb 22 15:49:49 UTC 2020
2020-02-22T15:49:49.9349018Z   network time: Sat, 22 Feb 2020 15:49:49 GMT
2020-02-22T15:49:49.9349929Z == end clock drift check ==
2020-02-22T15:49:50.6669736Z 
2020-02-22T15:49:50.6746898Z ##[error]Bash exited with code '1'.
2020-02-22T15:49:50.6761973Z ##[section]Finishing: Run build
2020-02-22T15:49:50.6803461Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:49:50.6808683Z Task         : Get sources
2020-02-22T15:49:50.6808993Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:49:50.6809297Z Version      : 1.0.0
2020-02-22T15:49:50.6809493Z Author       : Microsoft
2020-02-22T15:49:50.6809493Z Author       : Microsoft
2020-02-22T15:49:50.6809809Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T15:49:50.6810202Z ==============================================================================
2020-02-22T15:49:50.9865318Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T15:49:50.9910398Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:49:50.9990760Z Cleaning up task key
2020-02-22T15:49:50.9992047Z Start cleaning up orphan processes.
2020-02-22T15:49:51.0171682Z Terminate orphan process: pid (4242) (python)
2020-02-22T15:49:51.0318158Z ##[section]Finishing: Finalize Job
