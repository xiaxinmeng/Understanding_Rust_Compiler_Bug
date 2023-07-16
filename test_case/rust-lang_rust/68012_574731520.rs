plain
2020-01-15T15:52:13.9193579Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-15T15:52:13.9212079Z ##[command]git config gc.auto 0
2020-01-15T15:52:13.9216505Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-15T15:52:13.9220923Z ##[command]git config --get-all http.proxy
2020-01-15T15:52:13.9226259Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68012/merge:refs/remotes/pull/68012/merge
---
2020-01-15T15:57:58.8995569Z Found 0 error codes with no tests
2020-01-15T15:57:58.8995641Z Done!
2020-01-15T15:57:58.9000906Z 
2020-01-15T15:57:58.9000981Z 
2020-01-15T15:57:58.9001938Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-15T15:57:58.9002080Z 
2020-01-15T15:57:58.9002108Z 
2020-01-15T15:57:58.9010060Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-15T15:57:58.9010144Z Build completed unsuccessfully in 0:01:38
2020-01-15T15:57:58.9010144Z Build completed unsuccessfully in 0:01:38
2020-01-15T15:57:58.9056455Z == clock drift check ==
2020-01-15T15:57:58.9065366Z   local time: Wed Jan 15 15:57:58 UTC 2020
2020-01-15T15:57:59.1972811Z   network time: Wed, 15 Jan 2020 15:57:59 GMT
2020-01-15T15:57:59.1976815Z == end clock drift check ==
2020-01-15T15:57:59.9389216Z 
2020-01-15T15:57:59.9532819Z ##[error]Bash exited with code '1'.
2020-01-15T15:57:59.9571738Z ##[section]Starting: Checkout
2020-01-15T15:57:59.9573581Z ==============================================================================
2020-01-15T15:57:59.9573641Z Task         : Get sources
2020-01-15T15:57:59.9573712Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
