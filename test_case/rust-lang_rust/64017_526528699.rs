plain
2019-08-30T08:56:49.2819482Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-30T08:56:49.3016976Z ##[command]git config gc.auto 0
2019-08-30T08:56:49.3107679Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-30T08:56:49.3227969Z ##[command]git config --get-all http.proxy
2019-08-30T08:56:49.3385821Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64017/merge:refs/remotes/pull/64017/merge
---
2019-08-30T09:04:02.4871867Z tidy check
2019-08-30T09:04:03.4561725Z * 578 error codes
2019-08-30T09:04:03.4561866Z * highest error code: E0733
2019-08-30T09:04:03.8402726Z * 263 features
2019-08-30T09:04:04.1208956Z tidy error: `/checkout/src/liballoc/error.rs:920` contains `#[test]`; unit tests and benchmarks must be placed into separate files or directories named `tests.rs`, `benches.rs`, `tests` or `benches`
2019-08-30T09:04:04.5599151Z some tidy checks failed
2019-08-30T09:04:04.5599761Z 
2019-08-30T09:04:04.5599761Z 
2019-08-30T09:04:04.5600574Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-30T09:04:04.5603066Z 
2019-08-30T09:04:04.5603101Z 
2019-08-30T09:04:04.5612918Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-30T09:04:04.5612997Z Build completed unsuccessfully in 0:01:35
2019-08-30T09:04:04.5612997Z Build completed unsuccessfully in 0:01:35
2019-08-30T09:04:04.5664997Z == clock drift check ==
2019-08-30T09:04:04.5677906Z   local time: Fri Aug 30 09:04:04 UTC 2019
2019-08-30T09:04:04.6520694Z   network time: Fri, 30 Aug 2019 09:04:04 GMT
2019-08-30T09:04:04.6526181Z == end clock drift check ==
2019-08-30T09:04:06.0956269Z ##[error]Bash exited with code '1'.
2019-08-30T09:04:06.0990569Z ##[section]Starting: Checkout
2019-08-30T09:04:06.0992720Z ==============================================================================
2019-08-30T09:04:06.0992807Z Task         : Get sources
2019-08-30T09:04:06.0992860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
