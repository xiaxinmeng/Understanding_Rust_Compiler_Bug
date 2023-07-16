plain
2019-10-14T04:49:54.7159777Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T04:49:54.7388569Z ##[command]git config gc.auto 0
2019-10-14T04:49:54.7467762Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T04:49:54.7540517Z ##[command]git config --get-all http.proxy
2019-10-14T04:49:54.7687653Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65398/merge:refs/remotes/pull/65398/merge
---
2019-10-14T04:56:37.1500523Z    Compiling serde_json v1.0.40
2019-10-14T04:56:38.8858302Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-14T04:56:50.2033821Z     Finished release [optimized] target(s) in 1m 27s
2019-10-14T04:56:50.2109804Z tidy check
2019-10-14T04:56:50.6253485Z tidy error: /checkout/src/librustc_errors/lib.rs:253: trailing whitespace
2019-10-14T04:56:52.5305701Z Found 482 error codes
2019-10-14T04:56:52.5306632Z Found 0 error codes with no tests
2019-10-14T04:56:52.5307153Z Done!
2019-10-14T04:56:52.5310855Z some tidy checks failed
2019-10-14T04:56:52.5310855Z some tidy checks failed
2019-10-14T04:56:52.5313393Z 
2019-10-14T04:56:52.5313663Z 
2019-10-14T04:56:52.5314717Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-14T04:56:52.5315381Z 
2019-10-14T04:56:52.5315734Z 
2019-10-14T04:56:52.5319335Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-14T04:56:52.5319693Z Build completed unsuccessfully in 0:01:31
2019-10-14T04:56:52.5319693Z Build completed unsuccessfully in 0:01:31
2019-10-14T04:56:52.5369027Z == clock drift check ==
2019-10-14T04:56:52.5383576Z   local time: Mon Oct 14 04:56:52 UTC 2019
2019-10-14T04:56:52.6878273Z   network time: Mon, 14 Oct 2019 04:56:52 GMT
2019-10-14T04:56:52.6881657Z == end clock drift check ==
2019-10-14T04:56:53.6297967Z ##[error]Bash exited with code '1'.
2019-10-14T04:56:53.6351751Z ##[section]Starting: Checkout
2019-10-14T04:56:53.6353287Z ==============================================================================
2019-10-14T04:56:53.6353329Z Task         : Get sources
2019-10-14T04:56:53.6353366Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
