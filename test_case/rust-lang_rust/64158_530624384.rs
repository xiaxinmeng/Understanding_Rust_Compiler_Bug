plain
2019-09-12T00:54:56.9694562Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T00:54:56.9892971Z ##[command]git config gc.auto 0
2019-09-12T00:54:56.9965017Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T00:54:57.0034219Z ##[command]git config --get-all http.proxy
2019-09-12T00:54:57.0187519Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64158/merge:refs/remotes/pull/64158/merge
---
2019-09-12T01:02:12.4164953Z    Compiling serde_json v1.0.40
2019-09-12T01:02:14.3210747Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-12T01:02:25.5507947Z     Finished release [optimized] target(s) in 1m 33s
2019-09-12T01:02:25.5597838Z tidy check
2019-09-12T01:02:26.0007768Z tidy error: /checkout/src/libtest/lib.rs:1150: TODO is deprecated; use FIXME
2019-09-12T01:02:26.0010691Z tidy error: /checkout/src/libtest/lib.rs:1493: TODO is deprecated; use FIXME
2019-09-12T01:02:27.7023641Z some tidy checks failed
2019-09-12T01:02:27.7029497Z 
2019-09-12T01:02:27.7029497Z 
2019-09-12T01:02:27.7030820Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-12T01:02:27.7030971Z 
2019-09-12T01:02:27.7031022Z 
2019-09-12T01:02:27.7044514Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-12T01:02:27.7044596Z Build completed unsuccessfully in 0:01:36
2019-09-12T01:02:27.7044596Z Build completed unsuccessfully in 0:01:36
2019-09-12T01:02:27.7095787Z == clock drift check ==
2019-09-12T01:02:27.7112631Z   local time: Thu Sep 12 01:02:27 UTC 2019
2019-09-12T01:02:27.7975365Z   network time: Thu, 12 Sep 2019 01:02:27 GMT
2019-09-12T01:02:27.7975562Z == end clock drift check ==
2019-09-12T01:02:29.1052657Z ##[error]Bash exited with code '1'.
2019-09-12T01:02:29.1085600Z ##[section]Starting: Checkout
2019-09-12T01:02:29.1087186Z ==============================================================================
2019-09-12T01:02:29.1087231Z Task         : Get sources
2019-09-12T01:02:29.1087270Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
