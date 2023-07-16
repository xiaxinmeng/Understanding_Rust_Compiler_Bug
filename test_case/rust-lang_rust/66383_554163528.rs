plain
2019-11-15T00:55:43.8827446Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-15T00:55:43.9014301Z ##[command]git config gc.auto 0
2019-11-15T00:55:43.9084482Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-15T00:55:43.9135028Z ##[command]git config --get-all http.proxy
2019-11-15T00:55:43.9258868Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66383/merge:refs/remotes/pull/66383/merge
---
2019-11-15T01:01:09.3011325Z    Compiling serde_json v1.0.40
2019-11-15T01:01:10.7206220Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-15T01:01:19.5261713Z     Finished release [optimized] target(s) in 1m 11s
2019-11-15T01:01:19.5347754Z tidy check
2019-11-15T01:01:19.6527435Z tidy error: /checkout/src/liballoc/benches/vec.rs:488: line longer than 100 chars
2019-11-15T01:01:19.6600170Z tidy error: /checkout/src/liballoc/vec.rs: too many lines (3015) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-15T01:01:21.5399004Z Found 441 error codes
2019-11-15T01:01:21.5399734Z Found 0 error codes with no tests
2019-11-15T01:01:21.5399996Z Done!
2019-11-15T01:01:21.5400189Z some tidy checks failed
2019-11-15T01:01:21.5400189Z some tidy checks failed
2019-11-15T01:01:21.5400344Z 
2019-11-15T01:01:21.5400500Z 
2019-11-15T01:01:21.5403185Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-15T01:01:21.5403331Z 
2019-11-15T01:01:21.5403373Z 
2019-11-15T01:01:21.5405727Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-15T01:01:21.5406159Z Build completed unsuccessfully in 0:01:14
2019-11-15T01:01:21.5406159Z Build completed unsuccessfully in 0:01:14
2019-11-15T01:01:21.5447005Z == clock drift check ==
2019-11-15T01:01:21.5458806Z   local time: Fri Nov 15 01:01:21 UTC 2019
2019-11-15T01:01:21.6297359Z   network time: Fri, 15 Nov 2019 01:01:21 GMT
2019-11-15T01:01:21.6300335Z == end clock drift check ==
2019-11-15T01:01:23.0126180Z 
2019-11-15T01:01:23.0184538Z ##[error]Bash exited with code '1'.
2019-11-15T01:01:23.0210699Z ##[section]Starting: Checkout
2019-11-15T01:01:23.0212289Z ==============================================================================
2019-11-15T01:01:23.0212332Z Task         : Get sources
2019-11-15T01:01:23.0212368Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
