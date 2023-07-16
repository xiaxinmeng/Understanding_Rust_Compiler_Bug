plain
2019-10-09T07:04:28.8351442Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T07:04:28.8530364Z ##[command]git config gc.auto 0
2019-10-09T07:04:28.8599814Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T07:04:28.8649215Z ##[command]git config --get-all http.proxy
2019-10-09T07:04:28.8784282Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64873/merge:refs/remotes/pull/64873/merge
---
2019-10-09T07:11:29.0928449Z    Compiling serde_json v1.0.40
2019-10-09T07:11:30.9182156Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-09T07:11:42.7204570Z     Finished release [optimized] target(s) in 1m 31s
2019-10-09T07:11:42.7286049Z tidy check
2019-10-09T07:11:43.3241384Z tidy error: /checkout/src/libtest/lib.rs:1766: line longer than 100 chars
2019-10-09T07:11:43.3241568Z tidy error: /checkout/src/libtest/lib.rs:1767: line longer than 100 chars
2019-10-09T07:11:44.1815939Z tidy error: The Unstable Book has a 'library feature' section 'report-time' which doesn't correspond to an unstable library feature
2019-10-09T07:11:44.8207425Z Found 478 error codes
2019-10-09T07:11:44.8207523Z Found 0 error codes with no tests
2019-10-09T07:11:44.8207566Z Done!
2019-10-09T07:11:44.8207628Z some tidy checks failed
2019-10-09T07:11:44.8207628Z some tidy checks failed
2019-10-09T07:11:44.8207658Z 
2019-10-09T07:11:44.8207717Z 
2019-10-09T07:11:44.8208732Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T07:11:44.8208860Z 
2019-10-09T07:11:44.8208886Z 
2019-10-09T07:11:44.8212448Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T07:11:44.8212544Z Build completed unsuccessfully in 0:01:34
2019-10-09T07:11:44.8212544Z Build completed unsuccessfully in 0:01:34
2019-10-09T07:11:44.8261523Z == clock drift check ==
2019-10-09T07:11:44.8298021Z   local time: Wed Oct  9 07:11:44 UTC 2019
2019-10-09T07:11:44.9679177Z   network time: Wed, 09 Oct 2019 07:11:44 GMT
2019-10-09T07:11:44.9683186Z == end clock drift check ==
2019-10-09T07:11:46.3279312Z ##[error]Bash exited with code '1'.
2019-10-09T07:11:46.3311907Z ##[section]Starting: Checkout
2019-10-09T07:11:46.3313718Z ==============================================================================
2019-10-09T07:11:46.3313791Z Task         : Get sources
2019-10-09T07:11:46.3313855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
