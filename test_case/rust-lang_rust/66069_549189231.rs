plain
2019-11-03T22:50:08.1675683Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-03T22:50:08.1864026Z ##[command]git config gc.auto 0
2019-11-03T22:50:08.1931436Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-03T22:50:08.1979061Z ##[command]git config --get-all http.proxy
2019-11-03T22:50:08.2114523Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66069/merge:refs/remotes/pull/66069/merge
---
2019-11-03T22:56:13.6979244Z    Compiling serde_json v1.0.40
2019-11-03T22:56:15.4146015Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-03T22:56:26.7114194Z     Finished release [optimized] target(s) in 1m 26s
2019-11-03T22:56:26.7193614Z tidy check
2019-11-03T22:56:26.8427301Z tidy error: /checkout/src/liballoc/tests/vec.rs:1267: line longer than 100 chars
2019-11-03T22:56:26.8483747Z tidy error: /checkout/src/liballoc/vec.rs:650: line longer than 100 chars
2019-11-03T22:56:26.8484115Z tidy error: /checkout/src/liballoc/vec.rs:683: TODO is deprecated; use FIXME
2019-11-03T22:56:26.8493216Z tidy error: /checkout/src/liballoc/vec.rs: too many lines (3029) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-03T22:56:29.0878041Z Found 485 error codes
2019-11-03T22:56:29.0878146Z Found 0 error codes with no tests
2019-11-03T22:56:29.0878467Z Done!
2019-11-03T22:56:29.0884481Z 
2019-11-03T22:56:29.0884481Z 
2019-11-03T22:56:29.0884713Z 
2019-11-03T22:56:29.0885663Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-03T22:56:29.0886563Z 
2019-11-03T22:56:29.0886753Z 
2019-11-03T22:56:29.0886915Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-03T22:56:29.0887067Z Build completed unsuccessfully in 0:01:30
2019-11-03T22:56:29.0887067Z Build completed unsuccessfully in 0:01:30
2019-11-03T22:56:29.0887227Z some tidy checks failed
2019-11-03T22:56:29.0934334Z == clock drift check ==
2019-11-03T22:56:29.0955303Z   local time: Sun Nov  3 22:56:29 UTC 2019
2019-11-03T22:56:29.2421794Z   network time: Sun, 03 Nov 2019 22:56:29 GMT
2019-11-03T22:56:29.2422823Z == end clock drift check ==
2019-11-03T22:56:30.6137233Z 
2019-11-03T22:56:30.6208240Z ##[error]Bash exited with code '1'.
2019-11-03T22:56:30.6233933Z ##[section]Starting: Checkout
2019-11-03T22:56:30.6235402Z ==============================================================================
2019-11-03T22:56:30.6235452Z Task         : Get sources
2019-11-03T22:56:30.6235507Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
