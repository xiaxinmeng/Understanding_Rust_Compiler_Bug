plain
2019-11-30T22:21:50.7725718Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T22:21:50.7910345Z ##[command]git config gc.auto 0
2019-11-30T22:21:50.7981088Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T22:21:50.8031258Z ##[command]git config --get-all http.proxy
2019-11-30T22:21:50.8187608Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66703/merge:refs/remotes/pull/66703/merge
---
2019-11-30T22:27:40.6372382Z    Compiling serde_json v1.0.40
2019-11-30T22:27:42.2971519Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-30T22:27:52.8245236Z     Finished release [optimized] target(s) in 1m 24s
2019-11-30T22:27:52.8339787Z tidy check
2019-11-30T22:27:53.5074883Z tidy error: /checkout/src/librustc_mir/build/mod.rs:591: line longer than 100 chars
2019-11-30T22:27:55.5217010Z some tidy checks failed
2019-11-30T22:27:55.5217117Z Found 486 error codes
2019-11-30T22:27:55.5217160Z Found 0 error codes with no tests
2019-11-30T22:27:55.5266613Z Done!
2019-11-30T22:27:55.5266613Z Done!
2019-11-30T22:27:55.5274485Z 
2019-11-30T22:27:55.5274567Z 
2019-11-30T22:27:55.5275580Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-30T22:27:55.5275737Z 
2019-11-30T22:27:55.5275761Z 
2019-11-30T22:27:55.5275803Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-30T22:27:55.5275864Z Build completed unsuccessfully in 0:01:27
2019-11-30T22:27:55.5275864Z Build completed unsuccessfully in 0:01:27
2019-11-30T22:27:55.5289949Z == clock drift check ==
2019-11-30T22:27:55.5290021Z   local time: Sat Nov 30 22:27:55 UTC 2019
2019-11-30T22:27:55.8057999Z   network time: Sat, 30 Nov 2019 22:27:55 GMT
2019-11-30T22:27:55.8058116Z == end clock drift check ==
2019-11-30T22:27:57.2423700Z 
2019-11-30T22:27:57.2523178Z ##[error]Bash exited with code '1'.
2019-11-30T22:27:57.2550505Z ##[section]Starting: Checkout
2019-11-30T22:27:57.2552458Z ==============================================================================
2019-11-30T22:27:57.2552505Z Task         : Get sources
2019-11-30T22:27:57.2552562Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
