plain
2019-09-28T08:52:21.9134747Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T08:52:21.9318387Z ##[command]git config gc.auto 0
2019-09-28T08:52:21.9393003Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T08:52:21.9452855Z ##[command]git config --get-all http.proxy
2019-09-28T08:52:21.9591813Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64837/merge:refs/remotes/pull/64837/merge
---
2019-09-28T08:59:15.3591596Z    Compiling serde_json v1.0.40
2019-09-28T08:59:17.3336894Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-28T08:59:28.7183049Z     Finished release [optimized] target(s) in 1m 30s
2019-09-28T08:59:28.7274092Z tidy check
2019-09-28T08:59:29.4540951Z tidy error: /checkout/src/libcore/mem/maybe_uninit.rs:9: line longer than 100 chars
2019-09-28T08:59:30.8168130Z some tidy checks failed
2019-09-28T08:59:30.8168296Z 
2019-09-28T08:59:30.8168296Z 
2019-09-28T08:59:30.8169614Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-28T08:59:30.8169852Z 
2019-09-28T08:59:30.8169908Z 
2019-09-28T08:59:30.8178882Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-28T08:59:30.8178964Z Build completed unsuccessfully in 0:01:33
2019-09-28T08:59:30.8178964Z Build completed unsuccessfully in 0:01:33
2019-09-28T08:59:30.8234882Z == clock drift check ==
2019-09-28T08:59:30.8308627Z   local time: Sat Sep 28 08:59:30 UTC 2019
2019-09-28T08:59:31.1038589Z   network time: Sat, 28 Sep 2019 08:59:31 GMT
2019-09-28T08:59:31.1049433Z == end clock drift check ==
2019-09-28T08:59:32.4970844Z ##[error]Bash exited with code '1'.
2019-09-28T08:59:32.5014395Z ##[section]Starting: Checkout
2019-09-28T08:59:32.5018742Z ==============================================================================
2019-09-28T08:59:32.5018812Z Task         : Get sources
2019-09-28T08:59:32.5018867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
