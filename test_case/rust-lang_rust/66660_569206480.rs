plain
2019-12-27T06:43:33.7476600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-27T06:43:34.6173182Z ##[command]git config gc.auto 0
2019-12-27T06:43:34.6177444Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-27T06:43:34.6186376Z ##[command]git config --get-all http.proxy
2019-12-27T06:43:34.6190189Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66660/merge:refs/remotes/pull/66660/merge
---
2019-12-27T06:52:36.5801230Z     |
2019-12-27T06:52:36.5801531Z 347 |     fn check_pat(&mut self, cx: &LateContext<'_, '_>, p: &hir::Pat) {
2019-12-27T06:52:36.5801956Z     |                                                           ^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
2019-12-27T06:52:36.5802200Z     |
2019-12-27T06:52:36.5802487Z     = note: `-D elided-lifetimes-in-paths` implied by `-D warnings`
2019-12-27T06:52:37.2711812Z error: aborting due to previous error
2019-12-27T06:52:37.2711978Z 
2019-12-27T06:52:37.2779848Z error: could not compile `rustc_lint`.
2019-12-27T06:52:37.2801853Z warning: build failed, waiting for other jobs to finish...
2019-12-27T06:52:37.2801853Z warning: build failed, waiting for other jobs to finish...
2019-12-27T06:52:52.0894294Z error: build failed
2019-12-27T06:52:52.0912116Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-27T06:52:52.0926401Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-27T06:52:52.0926679Z Build completed unsuccessfully in 0:05:58
2019-12-27T06:52:52.0980487Z == clock drift check ==
2019-12-27T06:52:52.0996830Z   local time: Fri Dec 27 06:52:52 UTC 2019
2019-12-27T06:52:52.0996830Z   local time: Fri Dec 27 06:52:52 UTC 2019
2019-12-27T06:52:52.1932168Z   network time: Fri, 27 Dec 2019 06:52:52 GMT
2019-12-27T06:52:52.1932374Z == end clock drift check ==
2019-12-27T06:52:52.9261552Z 
2019-12-27T06:52:52.9362846Z ##[error]Bash exited with code '1'.
2019-12-27T06:52:52.9401790Z ##[section]Starting: Checkout
2019-12-27T06:52:52.9403520Z ==============================================================================
2019-12-27T06:52:52.9403589Z Task         : Get sources
2019-12-27T06:52:52.9403634Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
