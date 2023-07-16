plain
2019-09-30T02:09:04.8375129Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-30T02:09:05.6211944Z ##[command]git config gc.auto 0
2019-09-30T02:09:05.6220739Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-30T02:09:05.6223777Z ##[command]git config --get-all http.proxy
2019-09-30T02:09:05.6227311Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64909/merge:refs/remotes/pull/64909/merge
---
2019-09-30T02:16:21.0444056Z    Compiling serde_json v1.0.40
2019-09-30T02:16:22.8490762Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-30T02:16:33.9162864Z     Finished release [optimized] target(s) in 1m 29s
2019-09-30T02:16:33.9259241Z tidy check
2019-09-30T02:16:34.7831531Z tidy error: /checkout/src/libsyntax/parse/diagnostics.rs:551: line longer than 100 chars
2019-09-30T02:16:35.9042289Z some tidy checks failed
2019-09-30T02:16:35.9042435Z 
2019-09-30T02:16:35.9042435Z 
2019-09-30T02:16:35.9043486Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-30T02:16:35.9043632Z 
2019-09-30T02:16:35.9043662Z 
2019-09-30T02:16:35.9052289Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-30T02:16:35.9052662Z Build completed unsuccessfully in 0:01:32
2019-09-30T02:16:35.9052662Z Build completed unsuccessfully in 0:01:32
2019-09-30T02:16:35.9104076Z == clock drift check ==
2019-09-30T02:16:35.9119839Z   local time: Mon Sep 30 02:16:35 UTC 2019
2019-09-30T02:16:36.0742960Z   network time: Mon, 30 Sep 2019 02:16:36 GMT
2019-09-30T02:16:36.0744443Z == end clock drift check ==
2019-09-30T02:16:37.4749474Z ##[error]Bash exited with code '1'.
2019-09-30T02:16:37.4807258Z ##[section]Starting: Checkout
2019-09-30T02:16:37.4809842Z ==============================================================================
2019-09-30T02:16:37.4809902Z Task         : Get sources
2019-09-30T02:16:37.4810003Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
