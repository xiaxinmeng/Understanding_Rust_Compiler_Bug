plain
2019-10-12T16:33:22.4550987Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-12T16:33:22.4661549Z ##[command]git config gc.auto 0
2019-10-12T16:33:22.4773697Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-12T16:33:22.4833741Z ##[command]git config --get-all http.proxy
2019-10-12T16:33:22.4989981Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65160/merge:refs/remotes/pull/65160/merge
---
2019-10-12T16:39:39.1171026Z    Compiling serde_json v1.0.40
2019-10-12T16:39:41.2818285Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-12T16:39:53.8120934Z     Finished release [optimized] target(s) in 1m 37s
2019-10-12T16:39:53.8216513Z tidy check
2019-10-12T16:39:54.3684277Z tidy error: /checkout/src/librustc_mir/hair/pattern/_match.rs:1971: line longer than 100 chars
2019-10-12T16:39:56.4120613Z Found 482 error codes
2019-10-12T16:39:56.4120813Z Found 0 error codes with no tests
2019-10-12T16:39:56.4120864Z Done!
2019-10-12T16:39:56.4121207Z 
2019-10-12T16:39:56.4121207Z 
2019-10-12T16:39:56.4121237Z 
2019-10-12T16:39:56.4122252Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-12T16:39:56.4122361Z 
2019-10-12T16:39:56.4122406Z 
2019-10-12T16:39:56.4122455Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-12T16:39:56.4122506Z Build completed unsuccessfully in 0:01:41
2019-10-12T16:39:56.4122506Z Build completed unsuccessfully in 0:01:41
2019-10-12T16:39:56.4122552Z some tidy checks failed
2019-10-12T16:39:56.4194464Z == clock drift check ==
2019-10-12T16:39:56.4205528Z   local time: Sat Oct 12 16:39:56 UTC 2019
2019-10-12T16:39:56.5723432Z   network time: Sat, 12 Oct 2019 16:39:56 GMT
2019-10-12T16:39:56.5723638Z == end clock drift check ==
2019-10-12T16:39:57.4251257Z ##[error]Bash exited with code '1'.
2019-10-12T16:39:57.4297022Z ##[section]Starting: Checkout
2019-10-12T16:39:57.4299534Z ==============================================================================
2019-10-12T16:39:57.4299590Z Task         : Get sources
2019-10-12T16:39:57.4299641Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
