plain
2019-09-12T15:47:50.3529663Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T15:47:50.3700530Z ##[command]git config gc.auto 0
2019-09-12T15:47:50.3782239Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T15:47:50.3839187Z ##[command]git config --get-all http.proxy
2019-09-12T15:47:50.3983314Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64324/merge:refs/remotes/pull/64324/merge
---
2019-09-12T15:54:48.8597363Z    Compiling serde_json v1.0.40
2019-09-12T15:54:50.7236294Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-12T15:55:01.8601462Z     Finished release [optimized] target(s) in 1m 32s
2019-09-12T15:55:01.8688741Z tidy check
2019-09-12T15:55:02.3676176Z tidy error: /checkout/src/librustc_metadata/encoder.rs:1632: line longer than 100 chars
2019-09-12T15:55:03.8432893Z some tidy checks failed
2019-09-12T15:55:03.8437622Z 
2019-09-12T15:55:03.8437622Z 
2019-09-12T15:55:03.8438739Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-12T15:55:03.8439611Z 
2019-09-12T15:55:03.8439783Z 
2019-09-12T15:55:03.8447626Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-12T15:55:03.8449238Z Build completed unsuccessfully in 0:01:35
2019-09-12T15:55:03.8449238Z Build completed unsuccessfully in 0:01:35
2019-09-12T15:55:03.8501467Z == clock drift check ==
2019-09-12T15:55:03.8515654Z   local time: Thu Sep 12 15:55:03 UTC 2019
2019-09-12T15:55:04.0087458Z   network time: Thu, 12 Sep 2019 15:55:04 GMT
2019-09-12T15:55:04.0089942Z == end clock drift check ==
2019-09-12T15:55:05.2921473Z ##[error]Bash exited with code '1'.
2019-09-12T15:55:05.2955804Z ##[section]Starting: Checkout
2019-09-12T15:55:05.2957524Z ==============================================================================
2019-09-12T15:55:05.2957597Z Task         : Get sources
2019-09-12T15:55:05.2957645Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
