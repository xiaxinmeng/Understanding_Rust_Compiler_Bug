plain
2019-09-04T17:45:12.0407632Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T17:45:12.0630363Z ##[command]git config gc.auto 0
2019-09-04T17:45:12.0722598Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T17:45:12.0797638Z ##[command]git config --get-all http.proxy
2019-09-04T17:45:12.0953386Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64151/merge:refs/remotes/pull/64151/merge
---
2019-09-04T17:52:28.6356068Z tidy error: /checkout/src/librustc/hir/mod.rs: missing trailing newline
2019-09-04T17:52:30.3028193Z some tidy checks failed
2019-09-04T17:52:30.3030914Z 
2019-09-04T17:52:30.3031327Z 
2019-09-04T17:52:30.3032556Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-04T17:52:30.3033245Z 
2019-09-04T17:52:30.3039988Z 
2019-09-04T17:52:30.3040481Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-04T17:52:30.3040709Z Build completed unsuccessfully in 0:01:36
2019-09-04T17:52:30.3040709Z Build completed unsuccessfully in 0:01:36
2019-09-04T17:52:30.3090153Z == clock drift check ==
2019-09-04T17:52:30.3104860Z   local time: Wed Sep  4 17:52:30 UTC 2019
2019-09-04T17:52:30.4609341Z   network time: Wed, 04 Sep 2019 17:52:30 GMT
2019-09-04T17:52:30.4611661Z == end clock drift check ==
2019-09-04T17:52:31.8155767Z ##[error]Bash exited with code '1'.
2019-09-04T17:52:31.8190620Z ##[section]Starting: Checkout
2019-09-04T17:52:31.8192410Z ==============================================================================
2019-09-04T17:52:31.8192490Z Task         : Get sources
2019-09-04T17:52:31.8192539Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
