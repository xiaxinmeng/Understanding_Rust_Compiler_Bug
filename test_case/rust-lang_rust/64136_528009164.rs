plain
2019-09-04T17:25:02.7194233Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T17:25:02.7375931Z ##[command]git config gc.auto 0
2019-09-04T17:25:02.7496166Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T17:25:02.7546499Z ##[command]git config --get-all http.proxy
2019-09-04T17:25:02.7693604Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64136/merge:refs/remotes/pull/64136/merge
---
2019-09-04T17:32:12.8335394Z    Compiling serde_json v1.0.40
2019-09-04T17:32:14.6221399Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-04T17:32:25.7295601Z     Finished release [optimized] target(s) in 1m 29s
2019-09-04T17:32:25.7374063Z tidy check
2019-09-04T17:32:25.8644644Z tidy error: /checkout/src/libsyntax/parse/parser/expr.rs:70: trailing whitespace
2019-09-04T17:32:25.8645462Z tidy error: /checkout/src/libsyntax/parse/parser/expr.rs:83: trailing whitespace
2019-09-04T17:32:27.6528089Z some tidy checks failed
2019-09-04T17:32:27.6533428Z 
2019-09-04T17:32:27.6533428Z 
2019-09-04T17:32:27.6534875Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-04T17:32:27.6535030Z 
2019-09-04T17:32:27.6535057Z 
2019-09-04T17:32:27.6545234Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-04T17:32:27.6545384Z Build completed unsuccessfully in 0:01:33
2019-09-04T17:32:27.6545384Z Build completed unsuccessfully in 0:01:33
2019-09-04T17:32:27.6598675Z == clock drift check ==
2019-09-04T17:32:27.6612531Z   local time: Wed Sep  4 17:32:27 UTC 2019
2019-09-04T17:32:27.8945621Z   network time: Wed, 04 Sep 2019 17:32:27 GMT
2019-09-04T17:32:27.8949601Z == end clock drift check ==
2019-09-04T17:32:29.4495130Z ##[error]Bash exited with code '1'.
2019-09-04T17:32:29.4537795Z ##[section]Starting: Checkout
2019-09-04T17:32:29.4539545Z ==============================================================================
2019-09-04T17:32:29.4539592Z Task         : Get sources
2019-09-04T17:32:29.4539651Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
