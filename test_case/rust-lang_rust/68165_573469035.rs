plain
2020-01-12T22:59:22.8626142Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-12T22:59:22.8714535Z ##[command]git config gc.auto 0
2020-01-12T22:59:22.8763998Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-12T22:59:22.8823552Z ##[command]git config --get-all http.proxy
2020-01-12T22:59:22.8960278Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68165/merge:refs/remotes/pull/68165/merge
---
2020-01-12T23:04:46.5662187Z * 588 error codes
2020-01-12T23:04:46.5663186Z * highest error code: E0745
2020-01-12T23:04:46.6255245Z thread 'main' panicked at 'assertion failed: `(left != right)`
2020-01-12T23:04:46.6262014Z   left: `0`,
2020-01-12T23:04:46.6271921Z  right: `0`: "none" should be used when there is no issue, not "0"', src/tools/tidy/src/features.rs:417:21
2020-01-12T23:04:46.6272172Z 
2020-01-12T23:04:46.6272198Z 
2020-01-12T23:04:46.6272198Z 
2020-01-12T23:04:46.6327210Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-12T23:04:46.6327382Z 
2020-01-12T23:04:46.6327437Z 
2020-01-12T23:04:46.6327500Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-12T23:04:46.6327550Z Build completed unsuccessfully in 0:01:29
2020-01-12T23:04:46.6327550Z Build completed unsuccessfully in 0:01:29
2020-01-12T23:04:46.6357025Z == clock drift check ==
2020-01-12T23:04:46.6357105Z   local time: Sun Jan 12 23:04:46 UTC 2020
2020-01-12T23:04:46.9198906Z   network time: Sun, 12 Jan 2020 23:04:46 GMT
2020-01-12T23:04:46.9199012Z == end clock drift check ==
2020-01-12T23:04:47.6672070Z 
2020-01-12T23:04:47.6796371Z ##[error]Bash exited with code '1'.
2020-01-12T23:04:47.6834069Z ##[section]Starting: Checkout
2020-01-12T23:04:47.6835561Z ==============================================================================
2020-01-12T23:04:47.6835610Z Task         : Get sources
2020-01-12T23:04:47.6835651Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
