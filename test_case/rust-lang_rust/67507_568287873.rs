plain
2019-12-22T17:31:51.2277302Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-22T17:31:51.2509778Z ##[command]git config gc.auto 0
2019-12-22T17:31:51.2550549Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-22T17:31:51.2604869Z ##[command]git config --get-all http.proxy
2019-12-22T17:31:51.8289976Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67507/merge:refs/remotes/pull/67507/merge
---
2019-12-22T17:38:06.0946887Z    Compiling serde_json v1.0.40
2019-12-22T17:38:07.8050787Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-22T17:38:18.7797954Z     Finished release [optimized] target(s) in 1m 27s
2019-12-22T17:38:18.7902241Z tidy check
2019-12-22T17:38:19.3195693Z tidy error: /checkout/src/test/ui/structs-enums/enum-non-c-like-repr-c-and-int.rs:73: line longer than 100 chars
2019-12-22T17:38:21.5652511Z Found 485 error codes
2019-12-22T17:38:21.5653381Z Found 0 error codes with no tests
2019-12-22T17:38:21.5653658Z Done!
2019-12-22T17:38:21.5654038Z some tidy checks failed
2019-12-22T17:38:21.5654038Z some tidy checks failed
2019-12-22T17:38:21.5657920Z 
2019-12-22T17:38:21.5658250Z 
2019-12-22T17:38:21.5659520Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-22T17:38:21.5660212Z 
2019-12-22T17:38:21.5660570Z 
2019-12-22T17:38:21.5686662Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-22T17:38:21.5687018Z Build completed unsuccessfully in 0:01:32
2019-12-22T17:38:21.5687018Z Build completed unsuccessfully in 0:01:32
2019-12-22T17:38:21.5738602Z == clock drift check ==
2019-12-22T17:38:21.5749197Z   local time: Sun Dec 22 17:38:21 UTC 2019
2019-12-22T17:38:21.8538112Z   network time: Sun, 22 Dec 2019 17:38:21 GMT
2019-12-22T17:38:21.8540578Z == end clock drift check ==
2019-12-22T17:38:23.1835186Z 
2019-12-22T17:38:23.1904542Z ##[error]Bash exited with code '1'.
2019-12-22T17:38:23.1947214Z ##[section]Starting: Checkout
2019-12-22T17:38:23.1949136Z ==============================================================================
2019-12-22T17:38:23.1949216Z Task         : Get sources
2019-12-22T17:38:23.1949287Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
