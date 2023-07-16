plain
2020-01-03T01:58:13.1355775Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-03T01:58:14.0628035Z ##[command]git config gc.auto 0
2020-01-03T01:58:14.0632388Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-03T01:58:14.0636908Z ##[command]git config --get-all http.proxy
2020-01-03T01:58:14.0641003Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67822/merge:refs/remotes/pull/67822/merge
---
2020-01-03T02:04:42.4344144Z    Compiling serde_json v1.0.40
2020-01-03T02:04:43.8662022Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-03T02:04:53.3059079Z     Finished release [optimized] target(s) in 1m 14s
2020-01-03T02:04:53.3146130Z tidy check
2020-01-03T02:04:53.9272694Z tidy error: /checkout/src/test/ui/consts/const-prop-overflowing-casts.rs: ignoring line length unnecessarily
2020-01-03T02:04:55.6442206Z some tidy checks failed
2020-01-03T02:04:55.6446032Z Found 486 error codes
2020-01-03T02:04:55.6446275Z Found 0 error codes with no tests
2020-01-03T02:04:55.6446639Z Done!
2020-01-03T02:04:55.6446639Z Done!
2020-01-03T02:04:55.6446848Z 
2020-01-03T02:04:55.6446971Z 
2020-01-03T02:04:55.6448785Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-03T02:04:55.6449195Z 
2020-01-03T02:04:55.6449311Z 
2020-01-03T02:04:55.6457246Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-03T02:04:55.6457356Z Build completed unsuccessfully in 0:01:24
2020-01-03T02:04:55.6457356Z Build completed unsuccessfully in 0:01:24
2020-01-03T02:04:55.6509149Z == clock drift check ==
2020-01-03T02:04:55.6518505Z   local time: Fri Jan  3 02:04:55 UTC 2020
2020-01-03T02:04:55.8025721Z   network time: Fri, 03 Jan 2020 02:04:55 GMT
2020-01-03T02:04:55.8029595Z == end clock drift check ==
2020-01-03T02:04:57.1623994Z 
2020-01-03T02:04:57.1717932Z ##[error]Bash exited with code '1'.
2020-01-03T02:04:57.1752113Z ##[section]Starting: Checkout
2020-01-03T02:04:57.1753531Z ==============================================================================
2020-01-03T02:04:57.1753589Z Task         : Get sources
2020-01-03T02:04:57.1753643Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
