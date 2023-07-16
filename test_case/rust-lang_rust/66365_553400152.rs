plain
2019-11-13T13:00:02.6833105Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T13:00:02.7022873Z ##[command]git config gc.auto 0
2019-11-13T13:00:02.7058503Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T13:00:02.7120465Z ##[command]git config --get-all http.proxy
2019-11-13T13:00:02.7245654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66365/merge:refs/remotes/pull/66365/merge
---
2019-11-13T13:05:38.1818103Z    Compiling serde_json v1.0.40
2019-11-13T13:05:39.8461476Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-13T13:05:50.2530817Z     Finished release [optimized] target(s) in 1m 22s
2019-11-13T13:05:50.2639374Z tidy check
2019-11-13T13:05:51.0363396Z tidy error: /checkout/src/libcore/array/iter.rs:100: undocumented unsafe
2019-11-13T13:05:51.0364315Z tidy error: /checkout/src/libcore/array/iter.rs:196: undocumented unsafe
2019-11-13T13:05:52.6559746Z some tidy checks failed
2019-11-13T13:05:52.6559837Z Found 485 error codes
2019-11-13T13:05:52.6559873Z Found 0 error codes with no tests
2019-11-13T13:05:52.6559930Z Done!
2019-11-13T13:05:52.6559930Z Done!
2019-11-13T13:05:52.6559952Z 
2019-11-13T13:05:52.6559972Z 
2019-11-13T13:05:52.6560761Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-13T13:05:52.6560870Z 
2019-11-13T13:05:52.6560891Z 
2019-11-13T13:05:52.6567514Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-13T13:05:52.6568188Z Build completed unsuccessfully in 0:01:26
2019-11-13T13:05:52.6568188Z Build completed unsuccessfully in 0:01:26
2019-11-13T13:05:52.6618128Z == clock drift check ==
2019-11-13T13:05:52.6627606Z   local time: Wed Nov 13 13:05:52 UTC 2019
2019-11-13T13:05:52.7773642Z   network time: Wed, 13 Nov 2019 13:05:52 GMT
2019-11-13T13:05:52.7803566Z == end clock drift check ==
2019-11-13T13:05:54.1822218Z 
2019-11-13T13:05:54.1901053Z ##[error]Bash exited with code '1'.
2019-11-13T13:05:54.1927162Z ##[section]Starting: Checkout
2019-11-13T13:05:54.1928636Z ==============================================================================
2019-11-13T13:05:54.1928699Z Task         : Get sources
2019-11-13T13:05:54.1928736Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
