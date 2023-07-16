plain
2019-11-27T19:03:05.1877640Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T19:03:05.2098403Z ##[command]git config gc.auto 0
2019-11-27T19:03:05.2208410Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T19:03:05.2255635Z ##[command]git config --get-all http.proxy
2019-11-27T19:03:05.9314806Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66793/merge:refs/remotes/pull/66793/merge
---
2019-11-27T19:08:56.0809884Z    Compiling serde_json v1.0.40
2019-11-27T19:08:57.7409243Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-27T19:09:09.1928309Z     Finished release [optimized] target(s) in 1m 27s
2019-11-27T19:09:09.2020968Z tidy check
2019-11-27T19:09:09.6675806Z tidy error: /checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs: empty file
2019-11-27T19:09:09.6676163Z tidy error: /checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs: leading newline
2019-11-27T19:09:11.9530948Z some tidy checks failed
2019-11-27T19:09:11.9532438Z Found 486 error codes
2019-11-27T19:09:11.9532492Z Found 0 error codes with no tests
2019-11-27T19:09:11.9532560Z Done!
2019-11-27T19:09:11.9532560Z Done!
2019-11-27T19:09:11.9532591Z 
2019-11-27T19:09:11.9532617Z 
2019-11-27T19:09:11.9533493Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-27T19:09:11.9533620Z 
2019-11-27T19:09:11.9533646Z 
2019-11-27T19:09:11.9546861Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-27T19:09:11.9547208Z Build completed unsuccessfully in 0:01:31
2019-11-27T19:09:11.9547208Z Build completed unsuccessfully in 0:01:31
2019-11-27T19:09:11.9599788Z == clock drift check ==
2019-11-27T19:09:11.9611106Z   local time: Wed Nov 27 19:09:11 UTC 2019
2019-11-27T19:09:12.5009497Z   network time: Wed, 27 Nov 2019 19:09:12 GMT
2019-11-27T19:09:12.5013699Z == end clock drift check ==
2019-11-27T19:09:13.8969159Z 
2019-11-27T19:09:13.9082422Z ##[error]Bash exited with code '1'.
2019-11-27T19:09:13.9112661Z ##[section]Starting: Checkout
2019-11-27T19:09:13.9114497Z ==============================================================================
2019-11-27T19:09:13.9114569Z Task         : Get sources
2019-11-27T19:09:13.9114615Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
