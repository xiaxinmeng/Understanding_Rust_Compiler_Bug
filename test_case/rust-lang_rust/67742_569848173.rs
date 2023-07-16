plain
2019-12-31T01:49:34.0222074Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-31T01:49:34.0237467Z ##[command]git config gc.auto 0
2019-12-31T01:49:34.0241446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-31T01:49:34.0245984Z ##[command]git config --get-all http.proxy
2019-12-31T01:49:34.0253526Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67742/merge:refs/remotes/pull/67742/merge
---
2019-12-31T01:55:46.4358051Z    Compiling serde_json v1.0.40
2019-12-31T01:55:48.0151421Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-31T01:55:58.3539759Z     Finished release [optimized] target(s) in 1m 22s
2019-12-31T01:55:58.3628783Z tidy check
2019-12-31T01:55:59.1706169Z tidy error: /checkout/src/librustc/ty/context.rs:1521: TODO is deprecated; use FIXME
2019-12-31T01:56:00.8199934Z Found 486 error codes
2019-12-31T01:56:00.8200170Z Found 0 error codes with no tests
2019-12-31T01:56:00.8200521Z Done!
2019-12-31T01:56:00.8202390Z some tidy checks failed
2019-12-31T01:56:00.8202390Z some tidy checks failed
2019-12-31T01:56:00.8205326Z 
2019-12-31T01:56:00.8205770Z 
2019-12-31T01:56:00.8206838Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-31T01:56:00.8206994Z 
2019-12-31T01:56:00.8207016Z 
2019-12-31T01:56:00.8216600Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-31T01:56:00.8217014Z Build completed unsuccessfully in 0:01:32
2019-12-31T01:56:00.8217014Z Build completed unsuccessfully in 0:01:32
2019-12-31T01:56:00.8264561Z == clock drift check ==
2019-12-31T01:56:00.8275158Z   local time: Tue Dec 31 01:56:00 UTC 2019
2019-12-31T01:56:01.1150963Z   network time: Tue, 31 Dec 2019 01:56:01 GMT
2019-12-31T01:56:01.1151157Z == end clock drift check ==
2019-12-31T01:56:02.6078063Z 
2019-12-31T01:56:02.6168807Z ##[error]Bash exited with code '1'.
2019-12-31T01:56:02.6211911Z ##[section]Starting: Checkout
2019-12-31T01:56:02.6213456Z ==============================================================================
2019-12-31T01:56:02.6213526Z Task         : Get sources
2019-12-31T01:56:02.6213570Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
