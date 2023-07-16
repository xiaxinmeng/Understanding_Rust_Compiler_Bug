plain
2019-10-05T19:28:32.4489755Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T19:28:32.4700260Z ##[command]git config gc.auto 0
2019-10-05T19:28:32.4787667Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T19:28:32.4851782Z ##[command]git config --get-all http.proxy
2019-10-05T19:28:32.4988437Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65077/merge:refs/remotes/pull/65077/merge
---
2019-10-05T19:35:24.4577946Z    Compiling serde_json v1.0.40
2019-10-05T19:35:26.2733210Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-05T19:35:37.2278617Z     Finished release [optimized] target(s) in 1m 27s
2019-10-05T19:35:37.2348193Z tidy check
2019-10-05T19:35:37.9840179Z tidy error: /checkout/src/librustc_typeck/check/method/suggest.rs:561: trailing whitespace
2019-10-05T19:35:40.0781901Z some tidy checks failed
2019-10-05T19:35:40.0784839Z 
2019-10-05T19:35:40.0784839Z 
2019-10-05T19:35:40.0785855Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-05T19:35:40.0786356Z 
2019-10-05T19:35:40.0786958Z 
2019-10-05T19:35:40.0787277Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-05T19:35:40.0787402Z Build completed unsuccessfully in 0:01:30
2019-10-05T19:35:40.0787402Z Build completed unsuccessfully in 0:01:30
2019-10-05T19:35:40.0787710Z == clock drift check ==
2019-10-05T19:35:40.0787833Z   local time: Sat Oct  5 19:35:39 UTC 2019
2019-10-05T19:35:40.0787953Z   network time: Sat, 05 Oct 2019 19:35:39 GMT
2019-10-05T19:35:40.0788091Z == end clock drift check ==
2019-10-05T19:35:40.7288183Z ##[error]Bash exited with code '1'.
2019-10-05T19:35:40.7317260Z ##[section]Starting: Checkout
2019-10-05T19:35:40.7318651Z ==============================================================================
2019-10-05T19:35:40.7318695Z Task         : Get sources
2019-10-05T19:35:40.7319228Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
