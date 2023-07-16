plain
2020-03-19T15:08:26.3704655Z ========================== Starting Command Output ===========================
2020-03-19T15:08:26.3709510Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/464aed32-24d1-46f3-9e24-2246a884a080.sh
2020-03-19T15:08:26.3710016Z 
2020-03-19T15:08:26.3715562Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T15:08:26.3736160Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-19T15:08:26.3739697Z Task         : Get sources
2020-03-19T15:08:26.3740023Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T15:08:26.3740336Z Version      : 1.0.0
2020-03-19T15:08:26.3740569Z Author       : Microsoft
---
2020-03-19T15:08:27.5248176Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T15:08:27.5254714Z ##[command]git config gc.auto 0
2020-03-19T15:08:27.5263683Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T15:08:27.5267790Z ##[command]git config --get-all http.proxy
2020-03-19T15:08:27.5276036Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-03-19T15:15:58.5316336Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-19T15:15:59.4471654Z error[E0599]: no variant or associated item named `Method` found for enum `rustc_hir::hir::ImplItemKind<'_>` in the current scope
2020-03-19T15:15:59.4472709Z     --> src/librustc_typeck/collect.rs:2630:68
2020-03-19T15:15:59.4473247Z      |
2020-03-19T15:15:59.4474094Z 2630 |     if let Node::ImplItem(hir::ImplItem { kind: hir::ImplItemKind::Method(..), .. }) = node {
2020-03-19T15:15:59.4513422Z 
2020-03-19T15:15:59.7339063Z error: aborting due to previous error
2020-03-19T15:15:59.7344153Z 
2020-03-19T15:15:59.7351538Z For more information about this error, try `rustc --explain E0599`.
2020-03-19T15:15:59.7351538Z For more information about this error, try `rustc --explain E0599`.
2020-03-19T15:15:59.7387681Z error: could not compile `rustc_typeck`.
2020-03-19T15:15:59.7387985Z 
2020-03-19T15:15:59.7388472Z To learn more, run the command again with --verbose.
2020-03-19T15:15:59.7389088Z warning: build failed, waiting for other jobs to finish...
2020-03-19T15:16:12.5723518Z error: build failed
2020-03-19T15:16:12.5752883Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-19T15:16:12.5760073Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-19T15:16:12.5760658Z Build completed unsuccessfully in 0:05:01
2020-03-19T15:16:12.5808620Z == clock drift check ==
2020-03-19T15:16:12.5829307Z   local time: Thu Mar 19 15:16:12 UTC 2020
2020-03-19T15:16:12.5829307Z   local time: Thu Mar 19 15:16:12 UTC 2020
2020-03-19T15:16:12.7431376Z   network time: Thu, 19 Mar 2020 15:16:12 GMT
2020-03-19T15:16:12.7435814Z == end clock drift check ==
2020-03-19T15:16:13.4780714Z 
2020-03-19T15:16:13.4863009Z ##[error]Bash exited with code '1'.
2020-03-19T15:16:13.4878928Z ##[section]Finishing: Run build
2020-03-19T15:16:13.4933945Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-19T15:16:13.4940172Z Task         : Get sources
2020-03-19T15:16:13.4940560Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T15:16:13.4940949Z Version      : 1.0.0
2020-03-19T15:16:13.4941203Z Author       : Microsoft
2020-03-19T15:16:13.4941203Z Author       : Microsoft
2020-03-19T15:16:13.4941597Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T15:16:13.4942076Z ==============================================================================
2020-03-19T15:16:13.8713760Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T15:16:13.8757210Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-19T15:16:13.8845925Z Cleaning up task key
2020-03-19T15:16:13.8847164Z Start cleaning up orphan processes.
2020-03-19T15:16:13.9047770Z Terminate orphan process: pid (3945) (python)
2020-03-19T15:16:13.9254482Z ##[section]Finishing: Finalize Job
