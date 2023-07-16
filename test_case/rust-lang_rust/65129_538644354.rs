plain
2019-10-05T11:48:45.7209838Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T11:48:46.5662902Z ##[command]git config gc.auto 0
2019-10-05T11:48:46.5665586Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T11:48:46.5668127Z ##[command]git config --get-all http.proxy
2019-10-05T11:48:46.5670703Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65129/merge:refs/remotes/pull/65129/merge
---
2019-10-05T11:55:52.6596527Z    Compiling serde_json v1.0.40
2019-10-05T11:55:54.4449349Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-05T11:56:05.6344922Z     Finished release [optimized] target(s) in 1m 28s
2019-10-05T11:56:05.6423336Z tidy check
2019-10-05T11:56:06.0419977Z tidy error: /checkout/src/bootstrap/builder.rs:824: trailing whitespace
2019-10-05T11:56:07.5399128Z some tidy checks failed
2019-10-05T11:56:07.5403864Z 
2019-10-05T11:56:07.5403864Z 
2019-10-05T11:56:07.5404878Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-05T11:56:07.5405330Z 
2019-10-05T11:56:07.5405353Z 
2019-10-05T11:56:07.5416532Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-05T11:56:07.5416802Z Build completed unsuccessfully in 0:01:32
2019-10-05T11:56:07.5416802Z Build completed unsuccessfully in 0:01:32
2019-10-05T11:56:07.5463995Z == clock drift check ==
2019-10-05T11:56:07.5476746Z   local time: Sat Oct  5 11:56:07 UTC 2019
2019-10-05T11:56:07.6964011Z   network time: Sat, 05 Oct 2019 11:56:07 GMT
2019-10-05T11:56:07.6968714Z == end clock drift check ==
2019-10-05T11:56:09.0082026Z ##[error]Bash exited with code '1'.
2019-10-05T11:56:09.0113891Z ##[section]Starting: Checkout
2019-10-05T11:56:09.0115367Z ==============================================================================
2019-10-05T11:56:09.0115411Z Task         : Get sources
2019-10-05T11:56:09.0115452Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
