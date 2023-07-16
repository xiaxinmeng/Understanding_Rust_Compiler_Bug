plain
2019-11-29T20:16:09.2724085Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T20:16:09.2887227Z ##[command]git config gc.auto 0
2019-11-29T20:16:09.2952066Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T20:16:10.1083580Z ##[command]git config --get-all http.proxy
2019-11-29T20:16:10.1094395Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66877/merge:refs/remotes/pull/66877/merge
---
2019-11-29T20:21:45.1084103Z    Compiling serde_json v1.0.40
2019-11-29T20:21:46.6978396Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-29T20:21:56.9171684Z     Finished release [optimized] target(s) in 1m 21s
2019-11-29T20:21:56.9262880Z tidy check
2019-11-29T20:21:57.7391025Z tidy error: /checkout/src/librustc/mir/interpret/mod.rs:611: line longer than 100 chars
2019-11-29T20:21:57.7391459Z tidy error: /checkout/src/librustc/mir/interpret/mod.rs: missing trailing newline
2019-11-29T20:21:59.5172759Z some tidy checks failed
2019-11-29T20:21:59.5178863Z Found 486 error codes
2019-11-29T20:21:59.5178934Z Found 0 error codes with no tests
2019-11-29T20:21:59.5178981Z Done!
2019-11-29T20:21:59.5178981Z Done!
2019-11-29T20:21:59.5184647Z 
2019-11-29T20:21:59.5184980Z 
2019-11-29T20:21:59.5185828Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-29T20:21:59.5186183Z 
2019-11-29T20:21:59.5186280Z 
2019-11-29T20:21:59.5186567Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-29T20:21:59.5186999Z Build completed unsuccessfully in 0:01:24
2019-11-29T20:21:59.5186999Z Build completed unsuccessfully in 0:01:24
2019-11-29T20:21:59.5242662Z == clock drift check ==
2019-11-29T20:21:59.5263961Z   local time: Fri Nov 29 20:21:59 UTC 2019
2019-11-29T20:21:59.8129915Z   network time: Fri, 29 Nov 2019 20:21:59 GMT
2019-11-29T20:21:59.8132404Z == end clock drift check ==
2019-11-29T20:22:01.1054298Z 
2019-11-29T20:22:01.1152256Z ##[error]Bash exited with code '1'.
2019-11-29T20:22:01.1178757Z ##[section]Starting: Checkout
2019-11-29T20:22:01.1180412Z ==============================================================================
2019-11-29T20:22:01.1180667Z Task         : Get sources
2019-11-29T20:22:01.1180723Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
