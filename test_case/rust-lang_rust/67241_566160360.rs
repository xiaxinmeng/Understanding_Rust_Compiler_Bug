plain
2019-12-16T17:19:41.1728785Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T17:19:41.2011763Z ##[command]git config gc.auto 0
2019-12-16T17:19:41.2086428Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T17:19:41.2161000Z ##[command]git config --get-all http.proxy
2019-12-16T17:19:41.2300818Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-16T17:27:40.3736163Z For more information about this error, try `rustc --explain E0425`.
2019-12-16T17:27:40.4374378Z error: could not compile `rustc_mir`.
2019-12-16T17:27:40.4374774Z warning: build failed, waiting for other jobs to finish...
2019-12-16T17:27:42.9626357Z error: build failed
2019-12-16T17:27:42.9653864Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-16T17:27:42.9678320Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-16T17:27:42.9678652Z Build completed unsuccessfully in 0:04:34
2019-12-16T17:27:42.9725985Z == clock drift check ==
2019-12-16T17:27:42.9741400Z   local time: Mon Dec 16 17:27:42 UTC 2019
2019-12-16T17:27:42.9741400Z   local time: Mon Dec 16 17:27:42 UTC 2019
2019-12-16T17:27:43.2419061Z   network time: Mon, 16 Dec 2019 17:27:43 GMT
2019-12-16T17:27:43.2422403Z == end clock drift check ==
2019-12-16T17:27:44.5314094Z 
2019-12-16T17:27:44.5412614Z ##[error]Bash exited with code '1'.
2019-12-16T17:27:44.5448123Z ##[section]Starting: Checkout
2019-12-16T17:27:44.5449881Z ==============================================================================
2019-12-16T17:27:44.5449936Z Task         : Get sources
2019-12-16T17:27:44.5449983Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
