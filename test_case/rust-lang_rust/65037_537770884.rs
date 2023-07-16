plain
2019-10-03T03:21:11.0901968Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T03:21:11.1117824Z ##[command]git config gc.auto 0
2019-10-03T03:21:11.1182037Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T03:21:11.1231586Z ##[command]git config --get-all http.proxy
2019-10-03T03:21:11.1368753Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65037/merge:refs/remotes/pull/65037/merge
---
2019-10-03T03:28:20.0881395Z tidy error: /checkout/src/test/ui/feature-gates/feature-gate-track_caller.rs: leading newline
2019-10-03T03:28:21.5766318Z some tidy checks failed
2019-10-03T03:28:21.5773806Z 
2019-10-03T03:28:21.5773963Z 
2019-10-03T03:28:21.5775627Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T03:28:21.5775804Z 
2019-10-03T03:28:21.5775830Z 
2019-10-03T03:28:21.5781214Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T03:28:21.5781296Z Build completed unsuccessfully in 0:01:28
2019-10-03T03:28:21.5781296Z Build completed unsuccessfully in 0:01:28
2019-10-03T03:28:21.5820479Z == clock drift check ==
2019-10-03T03:28:21.5846464Z   local time: Thu Oct  3 03:28:21 UTC 2019
2019-10-03T03:28:21.7307273Z   network time: Thu, 03 Oct 2019 03:28:21 GMT
2019-10-03T03:28:21.7311599Z == end clock drift check ==
2019-10-03T03:28:23.0547430Z ##[error]Bash exited with code '1'.
2019-10-03T03:28:23.0582646Z ##[section]Starting: Checkout
2019-10-03T03:28:23.0584095Z ==============================================================================
2019-10-03T03:28:23.0584154Z Task         : Get sources
2019-10-03T03:28:23.0584194Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
