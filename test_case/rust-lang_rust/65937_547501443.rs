plain
2019-10-29T15:50:24.9490345Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-29T15:50:24.9698264Z ##[command]git config gc.auto 0
2019-10-29T15:50:24.9751685Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-29T15:50:24.9835349Z ##[command]git config --get-all http.proxy
2019-10-29T15:50:24.9991074Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65937/merge:refs/remotes/pull/65937/merge
---
2019-10-29T15:56:54.1762398Z    Compiling serde_json v1.0.40
2019-10-29T15:56:55.8805370Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-29T15:57:06.8761518Z     Finished release [optimized] target(s) in 1m 25s
2019-10-29T15:57:06.8839343Z tidy check
2019-10-29T15:57:07.3076244Z tidy error: /checkout/src/librustc_passes/ast_validation.rs:248: line longer than 100 chars
2019-10-29T15:57:07.3076748Z tidy error: /checkout/src/librustc_passes/ast_validation.rs:249: line longer than 100 chars
2019-10-29T15:57:09.2128482Z some tidy checks failed
2019-10-29T15:57:09.2129014Z Found 484 error codes
2019-10-29T15:57:09.2129081Z Found 0 error codes with no tests
2019-10-29T15:57:09.2130335Z Done!
2019-10-29T15:57:09.2130335Z Done!
2019-10-29T15:57:09.2134382Z 
2019-10-29T15:57:09.2134723Z 
2019-10-29T15:57:09.2138694Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-29T15:57:09.2138895Z 
2019-10-29T15:57:09.2138921Z 
2019-10-29T15:57:09.2147120Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-29T15:57:09.2147204Z Build completed unsuccessfully in 0:01:28
2019-10-29T15:57:09.2147204Z Build completed unsuccessfully in 0:01:28
2019-10-29T15:57:09.2197196Z == clock drift check ==
2019-10-29T15:57:09.2208627Z   local time: Tue Oct 29 15:57:09 UTC 2019
2019-10-29T15:57:09.2923513Z   network time: Tue, 29 Oct 2019 15:57:09 GMT
2019-10-29T15:57:09.2928289Z == end clock drift check ==
2019-10-29T15:57:10.5693048Z 
2019-10-29T15:57:10.5797472Z ##[error]Bash exited with code '1'.
2019-10-29T15:57:10.5828743Z ##[section]Starting: Checkout
2019-10-29T15:57:10.5830453Z ==============================================================================
2019-10-29T15:57:10.5830504Z Task         : Get sources
2019-10-29T15:57:10.5830548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
