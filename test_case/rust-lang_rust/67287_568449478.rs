plain
2019-12-23T11:08:50.4388480Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-23T11:08:50.4405093Z ##[command]git config gc.auto 0
2019-12-23T11:08:50.4408280Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-23T11:08:50.4410914Z ##[command]git config --get-all http.proxy
2019-12-23T11:08:50.4415886Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67287/merge:refs/remotes/pull/67287/merge
---
2019-12-23T11:17:11.4020763Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-12-23T11:17:12.0446574Z error[E0425]: cannot find value `begin` in this scope
2019-12-23T11:17:12.0447882Z    --> src/librustc_typeck/check/pat.rs:358:43
2019-12-23T11:17:12.0448348Z     |
2019-12-23T11:17:12.0448910Z 358 |             self.emit_err_pat_range(span, begin.span, end.span, lhs_fail, rhs_fail, lhs_ty, rhs_ty);
2019-12-23T11:17:12.0454168Z 
2019-12-23T11:17:12.0492519Z error[E0425]: cannot find value `end` in this scope
2019-12-23T11:17:12.0493474Z    --> src/librustc_typeck/check/pat.rs:358:55
2019-12-23T11:17:12.0494560Z     |
2019-12-23T11:17:12.0494560Z     |
2019-12-23T11:17:12.0495151Z 358 |             self.emit_err_pat_range(span, begin.span, end.span, lhs_fail, rhs_fail, lhs_ty, rhs_ty);
2019-12-23T11:17:12.0498894Z 
2019-12-23T11:17:17.0819886Z     Checking rustc_traits v0.0.0 (/checkout/src/librustc_traits)
2019-12-23T11:17:18.1172685Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-23T11:17:20.6786468Z error: aborting due to 2 previous errors
2019-12-23T11:17:20.6786468Z error: aborting due to 2 previous errors
2019-12-23T11:17:20.6787433Z 
2019-12-23T11:17:20.6794801Z For more information about this error, try `rustc --explain E0425`.
2019-12-23T11:17:20.6872803Z error: could not compile `rustc_typeck`.
2019-12-23T11:17:20.6876495Z warning: build failed, waiting for other jobs to finish...
2019-12-23T11:17:41.7232724Z error: build failed
2019-12-23T11:17:41.7273570Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-23T11:17:41.7299565Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-23T11:17:41.7300200Z Build completed unsuccessfully in 0:05:43
2019-12-23T11:17:41.7353306Z == clock drift check ==
2019-12-23T11:17:41.7370594Z   local time: Mon Dec 23 11:17:41 UTC 2019
2019-12-23T11:17:41.7370594Z   local time: Mon Dec 23 11:17:41 UTC 2019
2019-12-23T11:17:41.9176299Z   network time: Mon, 23 Dec 2019 11:17:41 GMT
2019-12-23T11:17:41.9180644Z == end clock drift check ==
2019-12-23T11:17:43.0621238Z 
2019-12-23T11:17:43.0725420Z ##[error]Bash exited with code '1'.
2019-12-23T11:17:43.0752094Z ##[section]Starting: Checkout
2019-12-23T11:17:43.0753616Z ==============================================================================
2019-12-23T11:17:43.0753665Z Task         : Get sources
2019-12-23T11:17:43.0753707Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
