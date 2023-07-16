plain
2019-12-07T02:17:02.8921749Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-07T02:17:02.9066475Z ##[command]git config gc.auto 0
2019-12-07T02:17:02.9142437Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-07T02:17:02.9201569Z ##[command]git config --get-all http.proxy
2019-12-07T02:17:02.9351580Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67077/merge:refs/remotes/pull/67077/merge
---
2019-12-07T02:25:22.3487915Z 
2019-12-07T02:25:22.3789744Z error: could not compile `rustc_codegen_llvm`.
2019-12-07T02:25:22.3790201Z warning: build failed, waiting for other jobs to finish...
2019-12-07T02:25:25.8415853Z error: build failed
2019-12-07T02:25:25.8443741Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-07T02:25:25.8451364Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-07T02:25:25.8451526Z Build completed unsuccessfully in 0:05:07
2019-12-07T02:25:25.8509400Z == clock drift check ==
2019-12-07T02:25:25.8523267Z   local time: Sat Dec  7 02:25:25 UTC 2019
2019-12-07T02:25:25.8523267Z   local time: Sat Dec  7 02:25:25 UTC 2019
2019-12-07T02:25:26.1443414Z   network time: Sat, 07 Dec 2019 02:25:26 GMT
2019-12-07T02:25:26.1443962Z == end clock drift check ==
2019-12-07T02:25:27.5629485Z 
2019-12-07T02:25:27.5730348Z ##[error]Bash exited with code '1'.
2019-12-07T02:25:27.5785913Z ##[section]Starting: Checkout
2019-12-07T02:25:27.5787553Z ==============================================================================
2019-12-07T02:25:27.5787627Z Task         : Get sources
2019-12-07T02:25:27.5787670Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
