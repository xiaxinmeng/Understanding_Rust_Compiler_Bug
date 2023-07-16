plain
2020-01-05T00:41:02.7982348Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T00:41:02.8154576Z ##[command]git config gc.auto 0
2020-01-05T00:41:02.8220302Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T00:41:02.8261722Z ##[command]git config --get-all http.proxy
2020-01-05T00:41:02.8404993Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67820/merge:refs/remotes/pull/67820/merge
---
2020-01-05T00:47:53.6071694Z For more information about this error, try `rustc --explain E0432`.
2020-01-05T00:47:53.6136256Z error: could not compile `rustc_parse`.
2020-01-05T00:47:53.6155825Z warning: build failed, waiting for other jobs to finish...
2020-01-05T00:48:35.5244190Z error: build failed
2020-01-05T00:48:35.5247568Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-05T00:48:35.5264815Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-05T00:48:35.5265119Z Build completed unsuccessfully in 0:04:11
2020-01-05T00:48:35.5315126Z == clock drift check ==
2020-01-05T00:48:35.5349913Z   local time: Sun Jan  5 00:48:35 UTC 2020
2020-01-05T00:48:35.5349913Z   local time: Sun Jan  5 00:48:35 UTC 2020
2020-01-05T00:48:35.5665785Z   network time: Sun, 05 Jan 2020 00:48:35 GMT
2020-01-05T00:48:35.5670679Z == end clock drift check ==
2020-01-05T00:48:36.7823664Z 
2020-01-05T00:48:36.7919343Z ##[error]Bash exited with code '1'.
2020-01-05T00:48:36.7949414Z ##[section]Starting: Checkout
2020-01-05T00:48:36.7951062Z ==============================================================================
2020-01-05T00:48:36.7951123Z Task         : Get sources
2020-01-05T00:48:36.7951173Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
