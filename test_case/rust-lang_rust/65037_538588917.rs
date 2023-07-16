plain
2019-10-04T22:56:38.3780429Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-04T22:56:38.3969252Z ##[command]git config gc.auto 0
2019-10-04T22:56:38.4054900Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-04T22:56:38.4132849Z ##[command]git config --get-all http.proxy
2019-10-04T22:56:38.4279408Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65037/merge:refs/remotes/pull/65037/merge
---
2019-10-04T23:03:36.9563171Z    Compiling serde_json v1.0.40
2019-10-04T23:03:38.8567729Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-04T23:03:50.2103719Z     Finished release [optimized] target(s) in 1m 31s
2019-10-04T23:03:50.2140737Z tidy check
2019-10-04T23:03:50.9780915Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:4930: line longer than 80 chars
2019-10-04T23:03:52.2038440Z some tidy checks failed
2019-10-04T23:03:52.2039568Z 
2019-10-04T23:03:52.2039568Z 
2019-10-04T23:03:52.2040615Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-04T23:03:52.2040786Z 
2019-10-04T23:03:52.2040814Z 
2019-10-04T23:03:52.2046767Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-04T23:03:52.2047205Z Build completed unsuccessfully in 0:01:34
2019-10-04T23:03:52.2047205Z Build completed unsuccessfully in 0:01:34
2019-10-04T23:03:52.2098085Z == clock drift check ==
2019-10-04T23:03:52.2115116Z   local time: Fri Oct  4 23:03:52 UTC 2019
2019-10-04T23:03:52.3092097Z   network time: Fri, 04 Oct 2019 23:03:52 GMT
2019-10-04T23:03:52.3097416Z == end clock drift check ==
2019-10-04T23:03:53.7597878Z ##[error]Bash exited with code '1'.
2019-10-04T23:03:53.7628848Z ##[section]Starting: Checkout
2019-10-04T23:03:53.7631321Z ==============================================================================
2019-10-04T23:03:53.7631428Z Task         : Get sources
2019-10-04T23:03:53.7631477Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
