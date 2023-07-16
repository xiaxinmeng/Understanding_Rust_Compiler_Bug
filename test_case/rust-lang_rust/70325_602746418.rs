plain
2020-03-23T17:12:38.3705661Z ========================== Starting Command Output ===========================
2020-03-23T17:12:38.3707892Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d2616e91-94b4-4ccf-a2d5-ca247ecaa58f.sh
2020-03-23T17:12:38.3708100Z 
2020-03-23T17:12:38.3712190Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T17:12:38.3730634Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70325/merge to s
2020-03-23T17:12:38.3734369Z Task         : Get sources
2020-03-23T17:12:38.3734664Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T17:12:38.3735750Z Version      : 1.0.0
2020-03-23T17:12:38.3735920Z Author       : Microsoft
---
2020-03-23T17:12:39.4028826Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T17:12:39.4035325Z ##[command]git config gc.auto 0
2020-03-23T17:12:39.4040284Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T17:12:39.4050188Z ##[command]git config --get-all http.proxy
2020-03-23T17:12:39.4058518Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70325/merge:refs/remotes/pull/70325/merge
---
2020-03-23T17:19:38.9580953Z     Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
2020-03-23T17:19:39.1158613Z error[E0061]: this function takes 0 arguments but 2 arguments were supplied
2020-03-23T17:19:39.1165691Z    --> src/librustc_ty/needs_drop.rs:104:48
2020-03-23T17:19:39.1166550Z     |
2020-03-23T17:19:39.1167296Z 104 |                         for upvar_ty in substs.upvar_tys(def_id, tcx) {
2020-03-23T17:19:39.1169497Z     |                                                |
2020-03-23T17:19:39.1175439Z     |                                                expected 0 arguments
2020-03-23T17:19:39.1238108Z 
2020-03-23T17:19:39.1240174Z error[E0061]: this function takes 0 arguments but 2 arguments were supplied
2020-03-23T17:19:39.1240174Z error[E0061]: this function takes 0 arguments but 2 arguments were supplied
2020-03-23T17:19:39.1241387Z    --> src/librustc_ty/needs_drop.rs:108:46
2020-03-23T17:19:39.1241815Z     |
2020-03-23T17:19:39.1242337Z 108 |                         let witness = substs.witness(def_id, tcx);
2020-03-23T17:19:39.1244023Z     |                                              |
2020-03-23T17:19:39.1244639Z     |                                              expected 0 arguments
2020-03-23T17:19:39.1244919Z 
2020-03-23T17:19:39.1765186Z error: aborting due to 2 previous errors
2020-03-23T17:19:39.1765186Z error: aborting due to 2 previous errors
2020-03-23T17:19:39.1766127Z 
2020-03-23T17:19:39.1766727Z For more information about this error, try `rustc --explain E0061`.
2020-03-23T17:19:39.1802859Z error: could not compile `rustc_ty`.
2020-03-23T17:19:39.1803323Z 
2020-03-23T17:19:39.1804177Z To learn more, run the command again with --verbose.
2020-03-23T17:19:39.1804983Z warning: build failed, waiting for other jobs to finish...
2020-03-23T17:19:50.4341516Z error: build failed
2020-03-23T17:19:50.4350247Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-23T17:19:50.4358341Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-23T17:19:50.4359100Z Build completed unsuccessfully in 0:04:30
2020-03-23T17:19:50.4403344Z == clock drift check ==
2020-03-23T17:19:50.4418718Z   local time: Mon Mar 23 17:19:50 UTC 2020
2020-03-23T17:19:50.4418718Z   local time: Mon Mar 23 17:19:50 UTC 2020
2020-03-23T17:19:50.7283331Z   network time: Mon, 23 Mar 2020 17:19:50 GMT
2020-03-23T17:19:50.7286149Z == end clock drift check ==
2020-03-23T17:19:51.5461151Z 
2020-03-23T17:19:51.5537218Z ##[error]Bash exited with code '1'.
2020-03-23T17:19:51.5554569Z ##[section]Finishing: Run build
2020-03-23T17:19:51.5604498Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70325/merge to s
2020-03-23T17:19:51.5608667Z Task         : Get sources
2020-03-23T17:19:51.5608956Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T17:19:51.5609235Z Version      : 1.0.0
2020-03-23T17:19:51.5609418Z Author       : Microsoft
2020-03-23T17:19:51.5609418Z Author       : Microsoft
2020-03-23T17:19:51.5609710Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T17:19:51.5610065Z ==============================================================================
2020-03-23T17:19:51.8805402Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T17:19:51.8846134Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70325/merge to s
2020-03-23T17:19:51.8931274Z Cleaning up task key
2020-03-23T17:19:51.8932638Z Start cleaning up orphan processes.
2020-03-23T17:19:51.9112542Z Terminate orphan process: pid (3679) (python)
2020-03-23T17:19:51.9244815Z ##[section]Finishing: Finalize Job
