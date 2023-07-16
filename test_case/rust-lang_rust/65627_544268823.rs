plain
2019-10-20T16:18:33.6129403Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-20T16:18:33.6311969Z ##[command]git config gc.auto 0
2019-10-20T16:18:33.6402262Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-20T16:18:33.6469375Z ##[command]git config --get-all http.proxy
2019-10-20T16:18:33.6600942Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65627/merge:refs/remotes/pull/65627/merge
---
2019-10-20T16:25:01.7522979Z    Compiling serde_json v1.0.40
2019-10-20T16:25:03.5002560Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-20T16:25:15.1959807Z     Finished release [optimized] target(s) in 1m 30s
2019-10-20T16:25:15.2050824Z tidy check
2019-10-20T16:25:16.2692558Z tidy error: duplicate error code: 739
2019-10-20T16:25:16.2692719Z tidy error: /checkout/src/librustc/error_codes.rs:2222:     E0739, // invalid track_caller application/syntax
2019-10-20T16:25:16.2692827Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:4977: E0739: r##"
2019-10-20T16:25:17.5759164Z Found 482 error codes
2019-10-20T16:25:17.5759251Z Found 0 error codes with no tests
2019-10-20T16:25:17.5759331Z Done!
2019-10-20T16:25:17.5759371Z some tidy checks failed
2019-10-20T16:25:17.5759371Z some tidy checks failed
2019-10-20T16:25:17.5766832Z 
2019-10-20T16:25:17.5772043Z 
2019-10-20T16:25:17.5773610Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-20T16:25:17.5773763Z 
2019-10-20T16:25:17.5773809Z 
2019-10-20T16:25:17.5778710Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-20T16:25:17.5778939Z Build completed unsuccessfully in 0:01:34
2019-10-20T16:25:17.5778939Z Build completed unsuccessfully in 0:01:34
2019-10-20T16:25:17.5826084Z == clock drift check ==
2019-10-20T16:25:17.5852354Z   local time: Sun Oct 20 16:25:17 UTC 2019
2019-10-20T16:25:17.8649310Z   network time: Sun, 20 Oct 2019 16:25:17 GMT
2019-10-20T16:25:17.8652453Z == end clock drift check ==
2019-10-20T16:25:19.2164542Z 
2019-10-20T16:25:19.2271080Z ##[error]Bash exited with code '1'.
2019-10-20T16:25:19.2304280Z ##[section]Starting: Checkout
2019-10-20T16:25:19.2306006Z ==============================================================================
2019-10-20T16:25:19.2306064Z Task         : Get sources
2019-10-20T16:25:19.2306114Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
