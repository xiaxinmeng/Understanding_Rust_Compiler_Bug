plain
2019-11-20T16:02:59.1984724Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T16:03:00.0431378Z ##[command]git config gc.auto 0
2019-11-20T16:03:00.0435518Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T16:03:00.0440007Z ##[command]git config --get-all http.proxy
2019-11-20T16:03:00.0442520Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66577/merge:refs/remotes/pull/66577/merge
---
2019-11-20T16:08:40.8537373Z    Compiling serde_json v1.0.40
2019-11-20T16:08:42.6561840Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-20T16:08:52.4864382Z     Finished release [optimized] target(s) in 1m 20s
2019-11-20T16:08:52.4959577Z tidy check
2019-11-20T16:08:53.3217507Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs:7: line longer than 100 chars
2019-11-20T16:08:53.3236806Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs: too many lines (3091) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-20T16:08:54.9859518Z some tidy checks failed
2019-11-20T16:08:54.9860731Z Found 441 error codes
2019-11-20T16:08:54.9861215Z Found 0 error codes with no tests
2019-11-20T16:08:54.9861484Z Done!
2019-11-20T16:08:54.9861484Z Done!
2019-11-20T16:08:54.9861670Z 
2019-11-20T16:08:54.9861852Z 
2019-11-20T16:08:54.9862861Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-20T16:08:54.9863399Z 
2019-11-20T16:08:54.9863599Z 
2019-11-20T16:08:54.9865764Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-20T16:08:54.9866051Z Build completed unsuccessfully in 0:01:23
2019-11-20T16:08:54.9866051Z Build completed unsuccessfully in 0:01:23
2019-11-20T16:08:54.9916763Z == clock drift check ==
2019-11-20T16:08:54.9926882Z   local time: Wed Nov 20 16:08:54 UTC 2019
2019-11-20T16:08:56.5153482Z   network time: Wed, 20 Nov 2019 16:08:56 GMT
2019-11-20T16:08:56.5153590Z == end clock drift check ==
2019-11-20T16:08:57.8028650Z 
2019-11-20T16:08:57.8125032Z ##[error]Bash exited with code '1'.
2019-11-20T16:08:57.8154642Z ##[section]Starting: Checkout
2019-11-20T16:08:57.8156372Z ==============================================================================
2019-11-20T16:08:57.8156422Z Task         : Get sources
2019-11-20T16:08:57.8156485Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
