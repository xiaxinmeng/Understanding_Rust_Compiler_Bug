plain
2019-10-21T18:53:38.9646892Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-21T18:53:38.9855471Z ##[command]git config gc.auto 0
2019-10-21T18:53:38.9921302Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-21T18:53:38.9972609Z ##[command]git config --get-all http.proxy
2019-10-21T18:53:39.0104162Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65668/merge:refs/remotes/pull/65668/merge
---
2019-10-21T18:59:38.0021275Z    Compiling serde_json v1.0.40
2019-10-21T18:59:39.6574785Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-21T18:59:50.0268197Z     Finished release [optimized] target(s) in 1m 20s
2019-10-21T18:59:50.0338320Z tidy check
2019-10-21T18:59:50.6346142Z tidy error: /checkout/src/librustc_typeck/check/generator_interior.rs:96: line longer than 100 chars
2019-10-21T18:59:51.9461178Z Found 482 error codes
2019-10-21T18:59:51.9461805Z Found 0 error codes with no tests
2019-10-21T18:59:51.9462016Z Done!
2019-10-21T18:59:51.9462128Z some tidy checks failed
2019-10-21T18:59:51.9462128Z some tidy checks failed
2019-10-21T18:59:51.9462223Z 
2019-10-21T18:59:51.9462312Z 
2019-10-21T18:59:51.9463178Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-21T18:59:51.9463505Z 
2019-10-21T18:59:51.9463618Z 
2019-10-21T18:59:51.9467402Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-21T18:59:51.9467784Z Build completed unsuccessfully in 0:01:23
2019-10-21T18:59:51.9467784Z Build completed unsuccessfully in 0:01:23
2019-10-21T18:59:51.9510746Z == clock drift check ==
2019-10-21T18:59:51.9523202Z   local time: Mon Oct 21 18:59:51 UTC 2019
2019-10-21T18:59:51.9975808Z   network time: Mon, 21 Oct 2019 18:59:51 GMT
2019-10-21T18:59:51.9979404Z == end clock drift check ==
2019-10-21T18:59:53.5168295Z 
2019-10-21T18:59:53.5223813Z ##[error]Bash exited with code '1'.
2019-10-21T18:59:53.5262131Z ##[section]Starting: Checkout
2019-10-21T18:59:53.5263506Z ==============================================================================
2019-10-21T18:59:53.5263565Z Task         : Get sources
2019-10-21T18:59:53.5263600Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
