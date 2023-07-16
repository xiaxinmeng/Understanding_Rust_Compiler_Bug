plain
2019-09-07T15:34:56.1888533Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-07T15:34:56.2116461Z ##[command]git config gc.auto 0
2019-09-07T15:34:56.2196042Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-07T15:34:56.2266250Z ##[command]git config --get-all http.proxy
2019-09-07T15:34:56.2426236Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64256/merge:refs/remotes/pull/64256/merge
---
2019-09-07T15:42:10.5398561Z    Compiling serde_json v1.0.40
2019-09-07T15:42:12.5975715Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-07T15:42:24.6718538Z     Finished release [optimized] target(s) in 1m 39s
2019-09-07T15:42:24.6805222Z tidy check
2019-09-07T15:42:25.2558916Z tidy error: /checkout/src/test/codegen/c-variadic.rs:21: line longer than 100 chars
2019-09-07T15:42:25.2559996Z tidy error: /checkout/src/test/codegen/c-variadic.rs:23: line longer than 100 chars
2019-09-07T15:42:25.2560415Z tidy error: /checkout/src/test/codegen/c-variadic.rs:42: line longer than 100 chars
2019-09-07T15:42:25.2560733Z tidy error: /checkout/src/test/codegen/c-variadic.rs:48: line longer than 100 chars
2019-09-07T15:42:25.2560974Z tidy error: /checkout/src/test/codegen/c-variadic.rs:73: line longer than 100 chars
2019-09-07T15:42:26.8158687Z some tidy checks failed
2019-09-07T15:42:26.8164742Z 
2019-09-07T15:42:26.8164742Z 
2019-09-07T15:42:26.8166090Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-07T15:42:26.8166260Z 
2019-09-07T15:42:26.8166288Z 
2019-09-07T15:42:26.8176653Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-07T15:42:26.8177139Z Build completed unsuccessfully in 0:01:42
2019-09-07T15:42:26.8177139Z Build completed unsuccessfully in 0:01:42
2019-09-07T15:42:26.8255846Z == clock drift check ==
2019-09-07T15:42:26.8275420Z   local time: Sat Sep  7 15:42:26 UTC 2019
2019-09-07T15:42:26.9769425Z   network time: Sat, 07 Sep 2019 15:42:26 GMT
2019-09-07T15:42:26.9769553Z == end clock drift check ==
2019-09-07T15:42:28.3739339Z ##[error]Bash exited with code '1'.
2019-09-07T15:42:28.3777151Z ##[section]Starting: Checkout
2019-09-07T15:42:28.3778885Z ==============================================================================
2019-09-07T15:42:28.3778939Z Task         : Get sources
2019-09-07T15:42:28.3778983Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
