plain
2019-12-20T21:09:41.2137530Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-20T21:09:41.2331741Z ##[command]git config gc.auto 0
2019-12-20T21:09:41.2410382Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-20T21:09:41.2471531Z ##[command]git config --get-all http.proxy
2019-12-20T21:09:41.2629329Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67332/merge:refs/remotes/pull/67332/merge
---
2019-12-20T21:17:21.8143791Z For more information about this error, try `rustc --explain E0308`.
2019-12-20T21:17:21.8478324Z error: could not compile `rustc`.
2019-12-20T21:17:21.8479122Z 
2019-12-20T21:17:21.8480128Z To learn more, run the command again with --verbose.
2019-12-20T21:17:21.8504868Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-20T21:17:21.8518796Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-20T21:17:21.8519021Z Build completed unsuccessfully in 0:04:31
2019-12-20T21:17:21.8576280Z == clock drift check ==
2019-12-20T21:17:21.8593248Z   local time: Fri Dec 20 21:17:21 UTC 2019
2019-12-20T21:17:21.8593248Z   local time: Fri Dec 20 21:17:21 UTC 2019
2019-12-20T21:17:22.0158000Z   network time: Fri, 20 Dec 2019 21:17:22 GMT
2019-12-20T21:17:22.0162132Z == end clock drift check ==
2019-12-20T21:17:22.8487711Z 
2019-12-20T21:17:22.8594233Z ##[error]Bash exited with code '1'.
2019-12-20T21:17:22.8623585Z ##[section]Starting: Checkout
2019-12-20T21:17:22.8625773Z ==============================================================================
2019-12-20T21:17:22.8625848Z Task         : Get sources
2019-12-20T21:17:22.8625897Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
