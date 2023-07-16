plain
2019-11-08T01:27:54.8013318Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T01:27:54.8222955Z ##[command]git config gc.auto 0
2019-11-08T01:27:54.8292509Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T01:27:54.8360739Z ##[command]git config --get-all http.proxy
2019-11-08T01:27:54.8522338Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66204/merge:refs/remotes/pull/66204/merge
---
2019-11-08T01:34:13.4616936Z    Compiling serde_json v1.0.40
2019-11-08T01:34:15.4712778Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-08T01:34:27.8177001Z     Finished release [optimized] target(s) in 1m 35s
2019-11-08T01:34:27.8254037Z tidy check
2019-11-08T01:34:28.4033297Z tidy error: /checkout/src/librustc_target/spec/x86_64_apple_darwin.rs:9: line longer than 100 chars
2019-11-08T01:34:30.7407509Z Found 485 error codes
2019-11-08T01:34:30.7407622Z Found 0 error codes with no tests
2019-11-08T01:34:30.7407705Z Done!
2019-11-08T01:34:30.7407798Z some tidy checks failed
2019-11-08T01:34:30.7407798Z some tidy checks failed
2019-11-08T01:34:30.7407827Z 
2019-11-08T01:34:30.7407853Z 
2019-11-08T01:34:30.7415041Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-08T01:34:30.7415402Z 
2019-11-08T01:34:30.7415428Z 
2019-11-08T01:34:30.7420115Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-08T01:34:30.7420382Z Build completed unsuccessfully in 0:01:39
2019-11-08T01:34:30.7420382Z Build completed unsuccessfully in 0:01:39
2019-11-08T01:34:30.7468249Z == clock drift check ==
2019-11-08T01:34:30.7506373Z   local time: Fri Nov  8 01:34:30 UTC 2019
2019-11-08T01:34:30.8970630Z   network time: Fri, 08 Nov 2019 01:34:30 GMT
2019-11-08T01:34:30.8986211Z == end clock drift check ==
2019-11-08T01:34:32.2932624Z 
2019-11-08T01:34:32.3043837Z ##[error]Bash exited with code '1'.
2019-11-08T01:34:32.3070638Z ##[section]Starting: Checkout
2019-11-08T01:34:32.3072934Z ==============================================================================
2019-11-08T01:34:32.3072993Z Task         : Get sources
2019-11-08T01:34:32.3073042Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
