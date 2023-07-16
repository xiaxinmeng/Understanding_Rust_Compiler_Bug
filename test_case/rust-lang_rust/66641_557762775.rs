plain
2019-11-23T03:22:15.8813703Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-23T03:22:15.9015725Z ##[command]git config gc.auto 0
2019-11-23T03:22:15.9073528Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-23T03:22:15.9144520Z ##[command]git config --get-all http.proxy
2019-11-23T03:22:15.9283250Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66641/merge:refs/remotes/pull/66641/merge
---
2019-11-23T03:28:27.9571273Z    Compiling serde_json v1.0.40
2019-11-23T03:28:29.6007683Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-23T03:28:39.9204687Z     Finished release [optimized] target(s) in 1m 23s
2019-11-23T03:28:40.7976064Z tidy check
2019-11-23T03:28:40.7977262Z tidy error: /checkout/src/librustc_parse/parser/item.rs:1340: line longer than 100 chars
2019-11-23T03:28:42.4505420Z Found 441 error codes
2019-11-23T03:28:42.4505543Z Found 0 error codes with no tests
2019-11-23T03:28:42.4505642Z Done!
2019-11-23T03:28:42.4505692Z some tidy checks failed
2019-11-23T03:28:42.4505692Z some tidy checks failed
2019-11-23T03:28:42.4505764Z 
2019-11-23T03:28:42.4505792Z 
2019-11-23T03:28:42.4506891Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-23T03:28:42.4507406Z 
2019-11-23T03:28:42.4507436Z 
2019-11-23T03:28:42.4511830Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-23T03:28:42.4512108Z Build completed unsuccessfully in 0:01:27
2019-11-23T03:28:42.4512108Z Build completed unsuccessfully in 0:01:27
2019-11-23T03:28:42.4564600Z == clock drift check ==
2019-11-23T03:28:42.4575293Z   local time: Sat Nov 23 03:28:42 UTC 2019
2019-11-23T03:28:42.7475231Z   network time: Sat, 23 Nov 2019 03:28:42 GMT
2019-11-23T03:28:42.7479216Z == end clock drift check ==
2019-11-23T03:28:44.2036915Z 
2019-11-23T03:28:44.2139559Z ##[error]Bash exited with code '1'.
2019-11-23T03:28:44.2168843Z ##[section]Starting: Checkout
2019-11-23T03:28:44.2170289Z ==============================================================================
2019-11-23T03:28:44.2170333Z Task         : Get sources
2019-11-23T03:28:44.2170369Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
