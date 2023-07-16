plain
2019-08-04T16:44:45.1136276Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-04T16:44:45.1388395Z ##[command]git config gc.auto 0
2019-08-04T16:44:45.1471283Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-04T16:44:45.1536057Z ##[command]git config --get-all http.proxy
2019-08-04T16:44:45.1687506Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63152/merge:refs/remotes/pull/63152/merge
---
2019-08-04T16:45:19.9890294Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-04T16:45:19.9890333Z 
2019-08-04T16:45:19.9890600Z   git checkout -b <new-branch-name>
2019-08-04T16:45:19.9890636Z 
2019-08-04T16:45:19.9890697Z HEAD is now at f68701c09 Merge 4f39c8fc0d3698b0133e340fe7023bdd60875c51 into 460072ebeed5a2463109894592ac172b47cdfb74
2019-08-04T16:45:20.0046736Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-04T16:45:20.0049745Z ==============================================================================
2019-08-04T16:45:20.0049806Z Task         : Bash
2019-08-04T16:45:20.0049855Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-04T16:54:10.3842208Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2019-08-04T16:54:11.6159035Z error[E0614]: type `rustc::mir::Local` cannot be dereferenced
2019-08-04T16:54:11.6165914Z    --> src/librustc_codegen_ssa/mir/analyze.rs:191:53
2019-08-04T16:54:11.6172580Z     |
2019-08-04T16:54:11.6179061Z 191 |             let decl_span = self.fx.mir.local_decls[*index].source_info.span;
2019-08-04T16:54:11.6189821Z 
2019-08-04T16:54:13.1035023Z error: aborting due to previous error
2019-08-04T16:54:13.1040414Z 
2019-08-04T16:54:13.1047432Z For more information about this error, try `rustc --explain E0614`.
2019-08-04T16:54:13.1047432Z For more information about this error, try `rustc --explain E0614`.
2019-08-04T16:54:13.1265717Z error: Could not compile `rustc_codegen_ssa`.
2019-08-04T16:54:13.1284204Z warning: build failed, waiting for other jobs to finish...
2019-08-04T16:54:15.5968016Z error: build failed
2019-08-04T16:54:15.6004422Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-08-04T16:54:15.6004530Z expected success, got: exit code: 101
2019-08-04T16:54:15.6020585Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-04T16:54:15.6020677Z Build completed unsuccessfully in 0:05:56
2019-08-04T16:54:16.9933504Z ##[error]Bash exited with code '1'.
2019-08-04T16:54:16.9994160Z ##[section]Starting: Checkout
2019-08-04T16:54:16.9996100Z ==============================================================================
2019-08-04T16:54:16.9996161Z Task         : Get sources
2019-08-04T16:54:16.9996230Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
