plain
2019-11-30T10:56:54.9680115Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T10:56:55.9342142Z ##[command]git config gc.auto 0
2019-11-30T10:56:55.9344748Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T10:56:55.9346718Z ##[command]git config --get-all http.proxy
2019-11-30T10:56:55.9351654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66880/merge:refs/remotes/pull/66880/merge
---
2019-11-30T11:02:10.3568685Z    Compiling serde_json v1.0.40
2019-11-30T11:02:11.7759313Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-30T11:02:20.6927296Z     Finished release [optimized] target(s) in 1m 10s
2019-11-30T11:02:20.7012258Z tidy check
2019-11-30T11:02:21.2874252Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0203.md: missing trailing newline
2019-11-30T11:02:22.7925011Z Found 486 error codes
2019-11-30T11:02:22.7925935Z Found 0 error codes with no tests
2019-11-30T11:02:22.7926047Z Done!
2019-11-30T11:02:22.7926295Z some tidy checks failed
2019-11-30T11:02:22.7926295Z some tidy checks failed
2019-11-30T11:02:22.7929217Z 
2019-11-30T11:02:22.7929458Z 
2019-11-30T11:02:22.7930404Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-30T11:02:22.7930563Z 
2019-11-30T11:02:22.7930583Z 
2019-11-30T11:02:22.7938087Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-30T11:02:22.7938187Z Build completed unsuccessfully in 0:01:14
2019-11-30T11:02:22.7938187Z Build completed unsuccessfully in 0:01:14
2019-11-30T11:02:22.7976817Z == clock drift check ==
2019-11-30T11:02:22.7986537Z   local time: Sat Nov 30 11:02:22 UTC 2019
2019-11-30T11:02:22.9469668Z   network time: Sat, 30 Nov 2019 11:02:22 GMT
2019-11-30T11:02:22.9470177Z == end clock drift check ==
2019-11-30T11:02:24.3313658Z 
2019-11-30T11:02:24.3393112Z ##[error]Bash exited with code '1'.
2019-11-30T11:02:24.3419363Z ##[section]Starting: Checkout
2019-11-30T11:02:24.3420693Z ==============================================================================
2019-11-30T11:02:24.3420734Z Task         : Get sources
2019-11-30T11:02:24.3420904Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
