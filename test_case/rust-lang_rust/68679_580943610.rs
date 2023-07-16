plain
2020-01-31T22:25:42.1761432Z ========================== Starting Command Output ===========================
2020-01-31T22:25:42.1764489Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/686747ad-7027-40b0-a169-592c741ff3b4.sh
2020-01-31T22:25:42.1764531Z 
2020-01-31T22:25:42.1769951Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-31T22:25:42.1776222Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-01-31T22:25:42.1778183Z Task         : Get sources
2020-01-31T22:25:42.1778213Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T22:25:42.1778258Z Version      : 1.0.0
2020-01-31T22:25:42.1778287Z Author       : Microsoft
---
2020-01-31T22:25:43.2257851Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-31T22:25:43.2270611Z ##[command]git config gc.auto 0
2020-01-31T22:25:43.2274170Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-31T22:25:43.2276207Z ##[command]git config --get-all http.proxy
2020-01-31T22:25:43.2282906Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68679/merge:refs/remotes/pull/68679/merge
---
2020-01-31T22:33:59.8509618Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-01-31T22:33:59.9687499Z error[E0425]: cannot find value `upvar_ty` in this scope
2020-01-31T22:33:59.9687882Z   --> src/librustc_ty/needs_drop.rs:86:41
2020-01-31T22:33:59.9688807Z    |
2020-01-31T22:33:59.9689193Z 86 |                 if this.seen_tys.insert(upvar_ty) {
2020-01-31T22:33:59.9689631Z 
2020-01-31T22:33:59.9719995Z error[E0425]: cannot find value `upvar_ty` in this scope
2020-01-31T22:33:59.9720349Z   --> src/librustc_ty/needs_drop.rs:87:46
2020-01-31T22:33:59.9720641Z    |
2020-01-31T22:33:59.9720641Z    |
2020-01-31T22:33:59.9721136Z 87 |                     this.unchecked_tys.push((upvar_ty, level + 1));
2020-01-31T22:33:59.9721555Z 
2020-01-31T22:33:59.9752190Z error[E0425]: cannot find value `upvar_ty` in this scope
2020-01-31T22:33:59.9752521Z    --> src/librustc_ty/needs_drop.rs:114:40
2020-01-31T22:33:59.9752771Z     |
2020-01-31T22:33:59.9752771Z     |
2020-01-31T22:33:59.9753054Z 114 | ...                   queue_type(upvar_ty);
2020-01-31T22:33:59.9753471Z 
2020-01-31T22:33:59.9788724Z error[E0425]: cannot find value `upvar_ty` in this scope
2020-01-31T22:33:59.9789117Z    --> src/librustc_ty/needs_drop.rs:126:40
2020-01-31T22:33:59.9789393Z     |
2020-01-31T22:33:59.9789393Z     |
2020-01-31T22:33:59.9789693Z 126 | ...                   queue_type(upvar_ty);
2020-01-31T22:33:59.9790115Z 
2020-01-31T22:34:00.1244474Z error: aborting due to 4 previous errors
2020-01-31T22:34:00.1245408Z 
2020-01-31T22:34:00.1247792Z For more information about this error, try `rustc --explain E0425`.
2020-01-31T22:34:00.1247792Z For more information about this error, try `rustc --explain E0425`.
2020-01-31T22:34:00.1289143Z error: could not compile `rustc_ty`.
2020-01-31T22:34:00.1289768Z warning: build failed, waiting for other jobs to finish...
2020-01-31T22:34:00.5944803Z error: build failed
2020-01-31T22:34:00.5968632Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-31T22:34:00.5981277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-31T22:34:00.5981527Z Build completed unsuccessfully in 0:05:43
2020-01-31T22:34:00.6039939Z == clock drift check ==
2020-01-31T22:34:00.6054262Z   local time: Fri Jan 31 22:34:00 UTC 2020
2020-01-31T22:34:00.6054262Z   local time: Fri Jan 31 22:34:00 UTC 2020
2020-01-31T22:34:00.8912552Z   network time: Fri, 31 Jan 2020 22:34:00 GMT
2020-01-31T22:34:00.8915008Z == end clock drift check ==
2020-01-31T22:34:01.3936986Z 
2020-01-31T22:34:01.4033413Z ##[error]Bash exited with code '1'.
2020-01-31T22:34:01.4046084Z ##[section]Finishing: Run build
2020-01-31T22:34:01.4102221Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-01-31T22:34:01.4103913Z Task         : Get sources
2020-01-31T22:34:01.4103952Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T22:34:01.4104007Z Version      : 1.0.0
2020-01-31T22:34:01.4104041Z Author       : Microsoft
2020-01-31T22:34:01.4104041Z Author       : Microsoft
2020-01-31T22:34:01.4104079Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-31T22:34:01.4104135Z ==============================================================================
2020-01-31T22:34:01.8499226Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-31T22:34:01.8544182Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68679/merge to s
2020-01-31T22:34:01.8684997Z Cleaning up task key
2020-01-31T22:34:01.8685761Z Start cleaning up orphan processes.
2020-01-31T22:34:01.8804879Z Terminate orphan process: pid (3957) (python)
2020-01-31T22:34:01.9371665Z ##[section]Finishing: Finalize Job
