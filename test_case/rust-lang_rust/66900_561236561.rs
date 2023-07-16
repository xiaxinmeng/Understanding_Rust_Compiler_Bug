plain
2019-12-03T15:48:24.0401879Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-03T15:48:24.0414443Z ##[command]git config gc.auto 0
2019-12-03T15:48:24.0416482Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-03T15:48:24.0418167Z ##[command]git config --get-all http.proxy
2019-12-03T15:48:24.0420390Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66900/merge:refs/remotes/pull/66900/merge
---
2019-12-03T15:53:55.6194050Z    Compiling serde_json v1.0.40
2019-12-03T15:53:57.2018465Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-03T15:54:07.3882407Z     Finished release [optimized] target(s) in 1m 21s
2019-12-03T15:54:07.3973590Z tidy check
2019-12-03T15:54:08.1018342Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0107.md:22: line longer than 80 chars
2019-12-03T15:54:09.9925970Z Found 486 error codes
2019-12-03T15:54:09.9927020Z Found 0 error codes with no tests
2019-12-03T15:54:09.9927099Z Done!
2019-12-03T15:54:09.9928697Z some tidy checks failed
2019-12-03T15:54:09.9928697Z some tidy checks failed
2019-12-03T15:54:09.9928739Z 
2019-12-03T15:54:09.9928765Z 
2019-12-03T15:54:09.9938294Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-03T15:54:09.9938438Z 
2019-12-03T15:54:09.9938465Z 
2019-12-03T15:54:09.9943759Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-03T15:54:09.9943827Z Build completed unsuccessfully in 0:01:24
2019-12-03T15:54:09.9943827Z Build completed unsuccessfully in 0:01:24
2019-12-03T15:54:09.9986762Z == clock drift check ==
2019-12-03T15:54:09.9994698Z   local time: Tue Dec  3 15:54:09 UTC 2019
2019-12-03T15:54:10.2633746Z   network time: Tue, 03 Dec 2019 15:54:10 GMT
2019-12-03T15:54:10.2639246Z == end clock drift check ==
2019-12-03T15:54:11.6316203Z 
2019-12-03T15:54:11.6426604Z ##[error]Bash exited with code '1'.
2019-12-03T15:54:11.6453254Z ##[section]Starting: Checkout
2019-12-03T15:54:11.6454779Z ==============================================================================
2019-12-03T15:54:11.6454831Z Task         : Get sources
2019-12-03T15:54:11.6454895Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
