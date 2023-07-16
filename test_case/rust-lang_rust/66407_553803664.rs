plain
2019-11-14T09:15:42.4392114Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-14T09:15:42.4595011Z ##[command]git config gc.auto 0
2019-11-14T09:15:42.4672564Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-14T09:15:42.4731808Z ##[command]git config --get-all http.proxy
2019-11-14T09:15:42.4899092Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66407/merge:refs/remotes/pull/66407/merge
---
2019-11-14T09:21:42.6989621Z    Compiling serde_json v1.0.40
2019-11-14T09:21:44.3837934Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-14T09:21:54.7443967Z     Finished release [optimized] target(s) in 1m 24s
2019-11-14T09:21:54.7550190Z tidy check
2019-11-14T09:21:54.9518466Z tidy error: /checkout/src/test/ui/parser/issue-58094-missing-right-square-bracket.rs: ignoring trailing newlines unnecessarily
2019-11-14T09:21:57.3356558Z Found 486 error codes
2019-11-14T09:21:57.3357419Z Found 0 error codes with no tests
2019-11-14T09:21:57.3357994Z Done!
2019-11-14T09:21:57.3358387Z some tidy checks failed
2019-11-14T09:21:57.3358387Z some tidy checks failed
2019-11-14T09:21:57.3360647Z 
2019-11-14T09:21:57.3361183Z 
2019-11-14T09:21:57.3365884Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-14T09:21:57.3366688Z 
2019-11-14T09:21:57.3366863Z 
2019-11-14T09:21:57.3378295Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-14T09:21:57.3378878Z Build completed unsuccessfully in 0:01:27
2019-11-14T09:21:57.3378878Z Build completed unsuccessfully in 0:01:27
2019-11-14T09:21:57.3443639Z == clock drift check ==
2019-11-14T09:21:57.3445114Z   local time: Thu Nov 14 09:21:57 UTC 2019
2019-11-14T09:21:57.4954385Z   network time: Thu, 14 Nov 2019 09:21:57 GMT
2019-11-14T09:21:57.4954721Z == end clock drift check ==
2019-11-14T09:21:58.7629442Z 
2019-11-14T09:21:58.7744561Z ##[error]Bash exited with code '1'.
2019-11-14T09:21:58.7767233Z ##[section]Starting: Checkout
2019-11-14T09:21:58.7768779Z ==============================================================================
2019-11-14T09:21:58.7768891Z Task         : Get sources
2019-11-14T09:21:58.7768934Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
