plain
2019-09-26T00:22:33.8989149Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-26T00:22:33.9179110Z ##[command]git config gc.auto 0
2019-09-26T00:22:33.9303725Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-26T00:22:33.9340285Z ##[command]git config --get-all http.proxy
2019-09-26T00:22:33.9513131Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64783/merge:refs/remotes/pull/64783/merge
---
2019-09-26T00:29:28.7752104Z    Compiling serde_json v1.0.40
2019-09-26T00:29:30.6474224Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-26T00:29:41.7428418Z     Finished release [optimized] target(s) in 1m 28s
2019-09-26T00:29:41.7496461Z tidy check
2019-09-26T00:29:41.9917086Z tidy error: /checkout/src/test/ui/issues/issue-64732.rs:5: trailing whitespace
2019-09-26T00:29:41.9917464Z tidy error: /checkout/src/test/ui/issues/issue-64732.rs:6: trailing whitespace
2019-09-26T00:29:43.8034432Z some tidy checks failed
2019-09-26T00:29:43.8034591Z 
2019-09-26T00:29:43.8034591Z 
2019-09-26T00:29:43.8036282Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-26T00:29:43.8036428Z 
2019-09-26T00:29:43.8036495Z 
2019-09-26T00:29:43.8055971Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-26T00:29:43.8056068Z Build completed unsuccessfully in 0:01:31
2019-09-26T00:29:43.8056068Z Build completed unsuccessfully in 0:01:31
2019-09-26T00:29:43.8109049Z == clock drift check ==
2019-09-26T00:29:43.8123399Z   local time: Thu Sep 26 00:29:43 UTC 2019
2019-09-26T00:29:43.9646918Z   network time: Thu, 26 Sep 2019 00:29:43 GMT
2019-09-26T00:29:43.9650475Z == end clock drift check ==
2019-09-26T00:29:45.5624882Z ##[error]Bash exited with code '1'.
2019-09-26T00:29:45.5666858Z ##[section]Starting: Checkout
2019-09-26T00:29:45.5669874Z ==============================================================================
2019-09-26T00:29:45.5669941Z Task         : Get sources
2019-09-26T00:29:45.5669990Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
